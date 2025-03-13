use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    title: &'a str,
    name: &'a str,
}

pub async fn about() -> Html<String> {
    Html(
        AboutTemplate {
            title: "About",
            name: "about template",
        }
        .render()
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
    )
}
