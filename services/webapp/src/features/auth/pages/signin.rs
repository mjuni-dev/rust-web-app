use std::sync::Arc;

use askama::Template;
use auth::{AuthServiceTrait, Credentials};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;

pub async fn signin_handler() -> Html<String> {
    signin_page(None).await
}

pub async fn signin_submit_handler(
    State(auth_service): State<Arc<dyn AuthServiceTrait>>,
    Form(form): Form<SignInForm>,
) -> impl IntoResponse {
    let creds = Credentials {
        email: form.email,
        password: form.password,
    };

    match auth_service.signin(creds).await {
        Ok(token) => {
            let cookie = Cookie::build(("auth_token", token))
                .path("/")
                .same_site(SameSite::Strict)
                .http_only(true)
                .build();

            (CookieJar::new().add(cookie), Redirect::to("/")).into_response()
        }
        Err(_) => signin_page(Some("Invalid email or password".to_string()))
            .await
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

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
