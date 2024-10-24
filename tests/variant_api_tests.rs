use axum::http::StatusCode;
use axum_test::TestServer;
use putin_bingo::{
    api::{
        controllers::variant_controller::CreateVariantRequest, router::create_router,
        BaseTemplateData,
    },
    asset_cache::AssetCache,
    import_templates, leak_alloc,
    state::AppState,
};
use sqlx::SqlitePool;

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

    // Создаем тестовый запрос
    let server = TestServer::new(app).unwrap();

    // Отправляем запрос
    let body = CreateVariantRequest {
        title: "Test variant".to_string(),
    };
    let response = server.post("/variants/add").form(&body).await;

    // Проверяем результат
    assert_eq!(response.status_code(), StatusCode::CREATED);
}
