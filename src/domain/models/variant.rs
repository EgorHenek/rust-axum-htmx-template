use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Variant {
    pub id: Option<Uuid>,
    pub title: String,
    pub confirmed: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}
