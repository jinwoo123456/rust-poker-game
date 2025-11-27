use super::handlers;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/signup", post(handlers::signup_handler))
        .route("/api/login", post(handlers::login_handler))
}
