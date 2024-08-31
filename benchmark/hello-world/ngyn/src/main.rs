use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("*", |_cx: &mut NgynContext, res: &mut NgynResponse| {
        *res.body_mut() = "Hello, World!".into();
    });

    let _ = app.listen("0.0.0.0:3000").await;
}
