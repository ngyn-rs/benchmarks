use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("*", handler(|_| "Hello, World!"));

    let _ = app.listen("0.0.0.0:3000").await;
}
