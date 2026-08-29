use axum::{Router, extract::State, http::StatusCode, routing::get};

use crate::{app::AppState, utils::response::ApiResponse};

pub fn router() -> Router<AppState> {
    Router::new().route("/users", get(get_users))
}

async fn get_users(State(_state): State<AppState>) -> ApiResponse<()> {
    ApiResponse {
        http_status: StatusCode::NO_CONTENT,
        status: false,
        message: String::from("No users found"),
        data: None,
    }
}
