use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Debug, FromRow, Deserialize)]
pub struct Users {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
