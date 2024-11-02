use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Variant {
    pub id: Option<Uuid>,
    pub title: String,
    pub confirmed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
}
