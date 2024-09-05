use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.any("*", handler(|_cx: &mut NgynContext| "Hello, World!"));

    let _ = app.listen("0.0.0.0:3000").await;
}
