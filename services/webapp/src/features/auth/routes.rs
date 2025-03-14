use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use std::sync::Arc;
use time::Duration;

use super::pages::{
    register::{register_handler, register_submit_handler},
    signin::{signin_handler, signin_submit_handler},
};
use auth::{AuthService, AuthServiceTrait, InMemoryUserRepository};

pub fn auth_routes<S: AuthServiceTrait>(auth_service: Arc<S>) -> Router {
    Router::new()
        .route("/signin", get(signin_handler))
        .route("/signin", post(signin_submit_handler))
        .route("/register", get(register_handler))
        .route("/register", post(register_submit_handler))
        .route("/logout", get(logout_handler))
        .with_state(auth_service)
}

async fn logout_handler() -> impl IntoResponse {
    let cookie = Cookie::build(("auth_token", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .same_site(SameSite::Strict)
        .http_only(true)
        .build();

    (CookieJar::new().add(cookie), Redirect::to("/auth/signin"))
}

// Middleware that can be used to protect routes
pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService<InMemoryUserRepository>>>,
    cookie_jar: CookieJar,
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl IntoResponse {
    let token = cookie_jar
        .get("auth_token")
        .map(|cookie| cookie.value().to_string());
    match token {
        Some(token) => {
            match auth_service.validate_token(&token).await {
                Ok(user) => {
                    // Add user to request extensions for handlers to access
                    req.extensions_mut().insert(user);
                    next.run(req).await
                }
                Err(_) => Redirect::to("/").into_response(),
            }
        }
        None => Redirect::to("/").into_response(),
    }
}
