use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
    routing::get,
    Router,
};

use crate::state::SharedState;

pub fn static_file_handler(state: SharedState) -> Router {
    Router::new()
        .route(
            "/:file",
            get(|state: State<SharedState>, path: Path<String>| async move {
                let Some(asset) = state.assets.get_from_path(&path) else {
                    return StatusCode::NOT_FOUND.into_response();
                };

                let mut headers = HeaderMap::new();

                // We set the content type explicitly here as it will otherwise
                // be inferred as an `octet-stream`
                headers.insert(CONTENT_TYPE, HeaderValue::from_static(asset.content_type()));

                (headers, asset.contents.clone()).into_response()
            }),
        )
        .with_state(state)
}
