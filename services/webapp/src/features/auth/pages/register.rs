use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    error: Option<&'a str>,
}

pub async fn register_page(error: Option<String>) -> Html<String> {
    Html(
        RegisterTemplate {
            title: "Register",
            error: error.as_deref(),
        }
        .render()
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
    )
}
