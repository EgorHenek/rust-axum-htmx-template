use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
    Form,
};
use axum_htmx::HxBoosted;
use chrono::Utc;
use minijinja::context;
use uuid::Uuid;

use crate::{domain::models::variant::Variant, errors::AppError, state::SharedState};

pub async fn create_variant_command(
    boosted: HxBoosted,
    State(state): State<SharedState>,
    Form(mut body): Form<Variant>,
) -> Result<impl IntoResponse, AppError> {
    let datetime = Utc::now();

    body.id = Some(Uuid::now_v7());
    body.confirmed = Some(false);
    body.createdAt = Some(datetime);

    let variant = body.to_owned();

    let rendered = state.render_with_context(
        boosted,
        "variant/create-result.html.jinja",
        context! { id => variant.id },
    )?;

    Ok((StatusCode::CREATED, rendered))
}
