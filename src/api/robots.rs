use axum::{
    extract::State,
    response::{IntoResponse, Result},
};

use crate::{errors::AppError, state::SharedState};

pub async fn robots(state: State<SharedState>) -> Result<impl IntoResponse, AppError> {
    let asset = state
        .assets
        .get("robots.txt")
        .ok_or(AppError::NotFound("robots.txt was not found".to_string()))?;

    let content = String::from_utf8(asset.contents.clone().to_vec())?;

    Ok(content.into_response())
}
