use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    http,
    handler::Handler,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() {
    // start tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Hello, world!");

    // address to bind to
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // build application with route
    let app = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog_index))
        .route("/blog/:slug", get(blog_post))
        .route("/contact", get(contact))
        .fallback(fallback.into_service());

    // run it with hyper
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn fallback(uri: http::Uri) -> impl IntoResponse {
    (http::StatusCode::NOT_FOUND, format!("404: No route {}", uri))
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctl-c");
    println!("signal shutdown");
}


async fn home() -> Html<String> {
    Html(format!("<p>{}</p>", "Hello world"))
}

async fn blog_index() -> Html<String> {
    Html(format!("<p>{}</p>", "Blog Incoming"))
}

async fn blog_post(axum::extract::Path(slug): axum::extract::Path<String>) -> Html<String> {
    Html(format!("<p>post id {}</p>", slug))
}

async fn contact() -> Html<String> {
    Html(format!("<p>{}</p>", "find me here"))
}
