use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    error: Option<&'a str>,
}

pub async fn login_page(error: Option<String>) -> Html<String> {
    Html(
        LoginTemplate {
            title: "Login",
            error: error.as_deref(),
        }
        .render()
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
    )
}
