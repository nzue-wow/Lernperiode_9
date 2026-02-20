use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello from  Rust backend!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}