use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Links {
    pub user_id: u32,
    pub overtime_pay: Option<u32>,
    pub expected_salary: u32,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
