use crate::{
    application::{
        commands::create_variant_command::create_variant_command,
        quiries::create_variant_form_query::create_variant_form_query,
    },
    state::SharedState,
};

use super::{health::health_check_handler, middlewares::cache_control, robots::robots};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<SharedState> {
    Router::new()
        .route("/_healthz", get(health_check_handler))
        .route(
            "/variants/add",
            post(create_variant_command).get(create_variant_form_query),
        )
        .route("/robots.txt", get(robots))
        .layer(middleware::from_fn(cache_control))
}
