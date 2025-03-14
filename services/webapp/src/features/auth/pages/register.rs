use std::sync::Arc;

use askama::Template;
use auth::{AuthServiceTrait, RegisterUser};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;

pub async fn register_handler() -> Html<String> {
    register_page(None).await
}

pub async fn register_submit_handler(
    State(auth_service): State<Arc<dyn AuthServiceTrait>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let user_data = RegisterUser {
        email: form.email,
        password: form.password,
        name: form.name,
    };

    match auth_service.register(user_data).await {
        Ok(_) => Redirect::to("/auth/signin").into_response(),
        Err(err) => {
            let error_message = match err {
                auth::AuthError::UserExists => "Email already registered",
                auth::AuthError::PasswordValidation(_) => "Password Validation",
                _ => "Registration failed. Please try again.",
            };

            register_page(Some(error_message.to_string()))
                .await
                .into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub name: String,
}

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
