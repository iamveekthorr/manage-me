use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: String,

    // These fields don't need to be visible outside for now
    updated_at: Option<DateTime<Utc>>, // Nullable field
    created_at: DateTime<Utc>,
}
