use state::AppState;
use tokio::net::TcpListener;

mod error;
mod features;
mod router;
mod state;

use error::Result;

// for creating test user with in mem db
use auth::{AuthServiceTrait, RegisterUser};

#[tokio::main]
async fn main() {
    let app_state = AppState::new().await.unwrap();

    // for testing with in mem db
    create_test_user(&app_state).await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router::routes(app_state))
        .await
        .unwrap();
}

async fn create_test_user(state: &AppState) {
    print!("Creating user");
    let user_data = RegisterUser {
        email: "user@email.com".to_string(),
        password: "password1".to_string(),
        name: "Matt".to_string(),
    };

    state.auth_service().clone().register(user_data).await;
}
