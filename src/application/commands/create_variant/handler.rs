use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::models::variant::Variant, errors::AppError,
    infrastructure::repositories::variant_repository::VariantRepository,
};

pub struct CreateVariantCommand {
    pub title: String,
}

pub struct CreateVariantHandler<R>
where
    R: VariantRepository,
{
    repository: R,
}

impl<R> CreateVariantHandler<R>
where
    R: VariantRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreateVariantCommand) -> Result<Variant, AppError> {
        let datetime = Utc::now().naive_utc();

        let variant = Variant {
            id: Some(Uuid::now_v7()),
            title: command.title,
            confirmed: Some(false),
            created_at: Some(datetime),
        };

        let variant = self.repository.create_variant(variant).await?;

        Ok(variant)
    }
}
