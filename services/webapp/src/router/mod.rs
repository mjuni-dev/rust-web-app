use askama::Template;
use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

use super::features::about::routes::about_routes;
use super::features::contact::routes::contact_routes;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .merge(about_routes())
        .merge(contact_routes())
        .nest_service("/assets", ServeDir::new("services/webapp/assets"))
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    title: &'a str,
    name: &'a str,
}

async fn root() -> Html<String> {
    Html(
        IndexTemplate {
            title: "Index",
            name: "Axum 0.8!!!",
        }
        .render()
        .expect("error"),
    )
}
