#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::models::variant::Variant, errors::AppError};
    use chrono::Utc;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn setup_test_db() -> SqlitePool {
        let db_url = "sqlite::memory:";
        let pool = SqlitePool::connect(db_url).await.unwrap();

        // Применяем миграции к тестовой БД
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_variant_success() {
        let pool = setup_test_db().await;
        let repo = SqliteVariantRepository::new(&pool);

        let datetime = Utc::now().naive_utc();
        let variant = Variant {
            id: Some(Uuid::now_v7()),
            title: "Test Variant".to_string(),
            confirmed: Some(false),
            created_at: Some(datetime),
        };

        let result = repo.create_variant(variant).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_variant_error() {
        let pool = setup_test_db().await;
        let repo = SqliteVariantRepository::new(&pool);

        let variant = Variant {
            id: None,
            title: "Test Variant".to_string(),
            confirmed: Some(false),
            created_at: None,
        };

        let result = repo.create_variant(variant).await;
        assert!(result.is_err());
        match result {
            Err(AppError::DatabaseError(_)) => (),
            _ => panic!("Expected DatabaseError"),
        }
    }
}
