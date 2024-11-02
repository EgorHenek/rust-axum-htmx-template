use crate::api::controllers::variant_controller::{create_variant, create_variant_form};

use super::{robots::robots, state::SharedState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<SharedState> {
    Router::new()
        .route(
            "/variants/add",
            post(create_variant).get(create_variant_form),
        )
        .route("/robots.txt", get(robots))
}
