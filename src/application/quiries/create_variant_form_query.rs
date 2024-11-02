use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use axum_htmx::HxBoosted;

use crate::{errors::AppError, state::SharedState};

pub async fn create_variant_form_query(
    boosted: HxBoosted,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let rendered = state.render(boosted, "variant/create.html.jinja")?;

    Ok(rendered)
}
