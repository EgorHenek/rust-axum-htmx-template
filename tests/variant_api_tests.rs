use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use rust_axum_htmx_template::{
    api::{asset_cache::AssetCache, router::create_router, state::AppState, BaseTemplateData},
    import_templates, leak_alloc,
};
use sqlx::SqlitePool;
use tower::ServiceExt;

async fn setup_test_db() -> SqlitePool {
    let db_url = "sqlite::memory:";
    let pool = SqlitePool::connect(db_url).await.unwrap();

    // Применяем миграции к тестовой БД
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}

#[tokio::test]
async fn test_create_variant_integration() {
    // Настраиваем тестовое окружение
    let assets = leak_alloc(AssetCache::load_files().await);
    let env = import_templates().unwrap();
    let base_template_data = leak_alloc(BaseTemplateData::new(assets));
    let db_pool = setup_test_db().await;
    let state = leak_alloc(AppState {
        assets,
        env,
        base_template_data,
        db_pool,
    });

    let app = create_router().with_state(state);

    // Отправляем запрос
    let form_body = format!("title={}", urlencoding::encode("Test variant"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/variants/add")
                .method(Method::POST)
                .header(
                    header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from(form_body))
                .unwrap(),
        )
        .await
        .unwrap();

    // Проверяем результат
    assert_eq!(response.status(), StatusCode::CREATED);
}
