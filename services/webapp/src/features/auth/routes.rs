use axum::{Router, extract::Query, response::Html, routing::get};
use serde::Deserialize;

use super::pages::{register::register_page, signin::signin_page};

#[derive(Debug, Deserialize)]
struct ErrorQuery {
    error: Option<String>,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signin", get(signin_handler))
        .route("/register", get(register_handler))
}

async fn signin_handler(Query(query): Query<ErrorQuery>) -> Html<String> {
    signin_page(query.error).await
}

async fn register_handler(Query(query): Query<ErrorQuery>) -> Html<String> {
    register_page(query.error).await
}
