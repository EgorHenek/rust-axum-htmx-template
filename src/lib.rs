use errors::AppError;
use minijinja::{path_loader, Environment};

pub mod api;
pub mod application;
pub mod asset_cache;
pub mod domain;
pub mod errors;
pub mod infrastructure;
pub mod state;

pub fn leak_alloc<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

pub fn import_templates() -> Result<Environment<'static>, AppError> {
    let mut env = Environment::new();

    env.set_loader(path_loader("templates"));

    Ok(env)
}
