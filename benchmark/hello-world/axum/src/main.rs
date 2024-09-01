use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let _ = axum::serve(listener, app);
}
