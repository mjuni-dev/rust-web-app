use axum::{Router, extract::Query, response::Html, routing::get};
use serde::Deserialize;

use super::pages::{login::login_page, register::register_page};

#[derive(Debug, Deserialize)]
struct ErrorQuery {
    error: Option<String>,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
}

async fn login_handler(Query(query): Query<ErrorQuery>) -> Html<String> {
    // login_page(Some("error message from handler".to_string())).await
    println!("query {:?}", query);
    login_page(query.error).await
}

async fn register_handler(Query(query): Query<ErrorQuery>) -> Html<String> {
    register_page(query.error).await
}
