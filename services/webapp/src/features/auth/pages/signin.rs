use askama::Template;
use axum::{http::StatusCode, response::Html};

#[derive(Template)]
#[template(path = "auth/signin.html")]
struct SignInTemplate<'a> {
    title: &'a str,
    error: Option<&'a str>,
}

pub async fn signin_page(error: Option<String>) -> Html<String> {
    Html(
        SignInTemplate {
            title: "Sign In",
            error: error.as_deref(),
        }
        .render()
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
    )
}
