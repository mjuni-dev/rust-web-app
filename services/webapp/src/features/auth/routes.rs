use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use super::pages::{
    register::{register_handler, register_submit_handler},
    signin::{signin_handler, signin_submit_handler},
};
use auth::AuthServiceTrait;

pub fn auth_routes<S: AuthServiceTrait>(auth_service: Arc<S>) -> Router {
    Router::new()
        .route("/signin", get(signin_handler))
        .route("/signin", post(signin_submit_handler))
        .route("/register", get(register_handler))
        .route("/register", post(register_submit_handler))
        .with_state(auth_service)
}
