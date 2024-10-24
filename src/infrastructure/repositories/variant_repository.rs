use sqlx::SqlitePool;

use crate::{domain::models::variant::Variant, errors::AppError};

pub trait VariantRepository {
    fn create_variant(
        &self,
        content: Variant,
    ) -> impl std::future::Future<Output = Result<Variant, AppError>> + Send;
}

pub struct SqliteVariantRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SqliteVariantRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }
}

impl<'a> VariantRepository for SqliteVariantRepository<'a> {
    async fn create_variant(&self, content: Variant) -> Result<Variant, AppError> {
        let variant = sqlx::query_as!(
            Variant,
            r#"
            INSERT INTO variants(id, title, confirmed, created_at)  
            VALUES (?, ?, ?, ?)
            RETURNING id as "id: uuid::Uuid", title, confirmed, created_at 
            "#,
            content.id,
            content.title,
            content.confirmed,
            content.created_at
        )
        .fetch_one(self.pool)
        .await?;

        Ok(variant)
    }
}
