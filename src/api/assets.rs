use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{
        header::{CONTENT_ENCODING, CONTENT_TYPE},
        HeaderMap, HeaderValue,
    },
    middleware,
    routing::get,
    Router,
};

use crate::state::SharedState;

use super::middlewares::cache_control;

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

                if [Some("css"), Some("js")].contains(&asset.ext()) {
                    headers.insert(CONTENT_ENCODING, HeaderValue::from_static("br"));
                }

                (headers, asset.contents.clone()).into_response()
            }),
        )
        .layer(middleware::from_fn(cache_control))
        .with_state(state)
}
