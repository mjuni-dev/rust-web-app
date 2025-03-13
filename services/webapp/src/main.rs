use tokio::net::TcpListener;

mod features;
mod router;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router::routes()).await.unwrap();
}
