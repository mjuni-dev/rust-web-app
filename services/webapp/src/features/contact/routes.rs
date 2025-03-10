use axum::{routing::get, Router};
use super::pages::contact;

pub fn contact_routes() -> Router {
    Router::new().route("/contact", get(contact))
}