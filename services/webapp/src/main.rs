use tokio::net::TcpListener;

mod error;
mod features;
mod router;
mod state;

use error::Result;

#[tokio::main]
async fn main() {
    let app_state = state::AppState::new().await.unwrap();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router::routes(app_state))
        .await
        .unwrap();
}
