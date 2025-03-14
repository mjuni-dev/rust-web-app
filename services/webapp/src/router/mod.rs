use askama::Template;
use axum::{Router, http::StatusCode, middleware, response::Html, routing::get};
use tower_http::services::ServeDir;

use crate::features::auth::routes::auth_middleware;
use crate::state::AppState;

use super::features::about::routes::about_routes;
use super::features::auth::routes::auth_routes;
use super::features::contact::routes::contact_routes;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .layer(middleware::from_fn_with_state(
            state.auth_service().clone(),
            auth_middleware,
        ))
        .route("/", get(root))
        .merge(about_routes())
        .merge(contact_routes())
        .nest("/auth", auth_routes(state.auth_service().clone()))
        .nest_service("/assets", ServeDir::new("services/webapp/assets"))
    // .with_state(state)
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
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
    )
}
