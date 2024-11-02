use std::{future::ready, time::Duration};

use api::{
    asset_cache::AssetCache, assets::static_file_handler, middlewares, router::create_router,
    state::AppState, BaseTemplateData,
};
use axum::{middleware, routing::get, Router};
use config::AppConfig;
use errors::AppError;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use minijinja::{path_loader, Environment};
use sqlx::SqlitePool;
use tokio::{net::TcpListener, signal};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;
pub mod application;
pub mod config;
pub mod domain;
pub mod errors;
pub mod infrastructure;

pub fn leak_alloc<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

pub fn import_templates() -> Result<Environment<'static>, AppError> {
    let mut env = Environment::new();

    env.set_loader(path_loader("templates"));

    Ok(env)
}

pub fn setup_tracing() {
    let layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into());
    tracing_subscriber::registry()
        .with(layer)
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
}

pub async fn start_main_server(config: &'static AppConfig) {
    let assets = leak_alloc(AssetCache::load_files().await);
    let env = import_templates().unwrap();
    let base_template_data = leak_alloc(BaseTemplateData::new(assets));
    let db_pool = SqlitePool::connect(config.db_url.as_str()).await.unwrap();
    let state = leak_alloc(AppState {
        assets,
        env,
        base_template_data,
        db_pool,
    });

    let app = create_router()
        .with_state(state)
        .nest("/assets", static_file_handler(state))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .route_layer(middleware::from_fn(middlewares::track_metrics));

    let listener = TcpListener::bind((config.host.as_str(), config.port as u16))
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn start_metrics_server(config: &'static AppConfig) {
    let recorder_handle = setup_metrics_recorder();
    let app = Router::new()
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route("/healthz", get(|| async {}));
    let listener = TcpListener::bind((config.host.as_str(), config.metrics_port as u16))
        .await
        .unwrap();

    tracing::debug!("listening metrics on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}
