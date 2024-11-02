use axum::response::Html;
use axum_htmx::HxBoosted;
use minijinja::{context, Value};
use sqlx::{Pool, Sqlite};

use crate::{api::SharedBaseTemplateData, errors::AppError};

use super::asset_cache::SharedAssetCache;

pub type SharedState = &'static AppState;

#[derive(Clone)]
pub struct AppState {
    pub assets: SharedAssetCache,
    pub base_template_data: SharedBaseTemplateData,
    pub db_pool: Pool<Sqlite>,
    pub env: minijinja::Environment<'static>,
}

impl AppState {
    pub fn render(
        &self,
        HxBoosted(boosted): HxBoosted,
        template: &str,
    ) -> Result<Html<String>, AppError> {
        let template = self.env.get_template(template)?;

        if boosted {
            let rendered = template.render(context! {})?;

            return Ok(Html(rendered));
        }

        let rendered = template.render(context! { base => Some(self.base_template_data )})?;

        Ok(Html(rendered))
    }

    pub fn render_with_context(
        &self,
        HxBoosted(boosted): HxBoosted,
        template: &str,
        ctx: Value,
    ) -> Result<Html<String>, AppError> {
        let template = self.env.get_template(template)?;

        if boosted {
            let rendered = template.render(ctx)?;

            return Ok(Html(rendered));
        }

        let rendered = template.render(context! { base => Some(self.base_template_data), ..ctx})?;

        Ok(Html(rendered))
    }
}
