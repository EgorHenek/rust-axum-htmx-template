use asset_cache::SharedAssetCache;
use serde::Serialize;

pub mod asset_cache;
pub mod assets;
pub mod controllers;
pub mod middlewares;
mod robots;
pub mod router;
pub mod state;

pub type SharedBaseTemplateData = &'static BaseTemplateData;

#[derive(Clone, Serialize)]
pub struct BaseTemplateData {
    styles: String,
    scripts: String,
}

impl BaseTemplateData {
    pub fn new(assets: SharedAssetCache) -> Self {
        let styles = assets
            .get("index.css")
            .expect("failed to build base template data: index.css")
            .path
            .clone();

        let scripts = assets
            .get("index.js")
            .expect("failed to build base template data: index.js")
            .path
            .clone();

        Self { styles, scripts }
    }
}
