use axum::{routing::get, Router};
use super::pages::about;

pub fn about_routes() -> Router {
    Router::new().route("/about", get(about))
}