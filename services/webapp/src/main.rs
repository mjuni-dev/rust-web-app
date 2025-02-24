use askama::Template;
use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("services/webapp/assets"));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    Html(IndexTemplate {
        title: "Index",
        name: "Axum 0.8!!!",
    }.render().expect("error"))
}

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate<'a> {
    title: &'a str,
    name: &'a str,
}