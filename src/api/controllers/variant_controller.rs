use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
    Form,
};
use axum_htmx::HxBoosted;
use minijinja::context;
use serde::{Deserialize, Serialize};

use crate::{
    api::state::SharedState,
    application::commands::create_variant::handler::{CreateVariantCommand, CreateVariantHandler},
    errors::AppError,
    infrastructure::repositories::variant_repository::SqliteVariantRepository,
};

#[derive(Deserialize, Serialize)]
pub struct CreateVariantRequest {
    pub title: String,
}

pub async fn create_variant(
    boosted: HxBoosted,
    State(state): State<SharedState>,
    Form(form_data): Form<CreateVariantRequest>,
) -> Result<impl IntoResponse, AppError> {
    let repository = SqliteVariantRepository::new(&state.db_pool);
    let command = CreateVariantCommand {
        title: form_data.title,
    };
    let handler = CreateVariantHandler::new(repository);

    let variant = handler.handle(command).await?;

    let rendered = state.render_with_context(
        boosted,
        "variant/create-result.html.jinja",
        context! { id => variant.id },
    )?;

    Ok((StatusCode::CREATED, rendered))
}

pub async fn create_variant_form(
    boosted: HxBoosted,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let rendered = state.render(boosted, "variant/create.html.jinja")?;

    Ok(rendered)
}
