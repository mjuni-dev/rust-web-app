use askama::Template;
use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("services/webapp/assets"))
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

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    title: &'a str,
    name: &'a str,
}
