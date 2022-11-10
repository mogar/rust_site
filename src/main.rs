use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    http,
    handler::Handler,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // build application with route
    let app = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog_index))
        .route("/blog/:slug", get(blog_post))
        .route("/contact", get(contact))
        .fallback(fallback.into_service());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
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
