use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // build application with route
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/blog", get(|| async { "Hello blogger!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
