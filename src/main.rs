#[macro_use]
extern crate lazy_static;

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
use tera::{Context, Tera};

mod blog;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

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
    // TODO: custom MakeService here
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

/// handler to generate a user-viewable failure message in case of a bad url
pub async fn fallback(uri: http::Uri) -> impl IntoResponse {
    (http::StatusCode::NOT_FOUND, format!("404: No route {}", uri))
}

/// safe shutdown handler
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctl-c");
    println!("signal shutdown");
}

/// Prepare global page context (title, navbar, etc.)
/// Constructs and populates a new Tera context.
fn get_page_context() -> Context {
    let mut context = Context::new();
    context.insert("page_title", &"Mogar");

    let pagelinks = vec!["/", "/blog", "/contact"];
    context.insert("navbar", &pagelinks);
    // TODO: other global items (e.g. header, footer, etc.)

    context
}

/// Render the context into a template determined by the current page
fn get_rendered_html(page: &str, context: Context) -> Html<String> {
    // NOTE: currently all pages use the same template
    match TEMPLATES.render("base.html", &context) {
        Ok(s) => Html(s),
        Err(e) => Html(format!("500: Render failure {:?} for {}", e, page)),
    }
}

/// Render and return Html for the home page
async fn home() -> Html<String> {
    let mut context = get_page_context();
    context.insert("page_content", &"<p>Hello World!</p>");

    get_rendered_html("home", context)
}

/// Render and return Html for the contact page
async fn contact() -> Html<String> {
    let mut context = get_page_context();
    context.insert("page_content", &"<ul>
        <li><a href=\"https://twitter.com/MorgneticField\">Twitter</a></li>
        <li><a href=\"https://github.com/mogar\">GitHub</a></li>
        <li><a href=\"https://www.linkedin.com/in/morgan-redfield/\">LinkedIn</a></li>
    </ul>");

    get_rendered_html("contact", context)
}

/// Render and return Html for the blog index
pub async fn blog_index() -> Html<String> {
    let mut context = get_page_context();
    context.insert("page_content", &blog::index());

    get_rendered_html("blog_index", context)
}

/// Render and return Html for a blog post determined by `slug`
pub async fn blog_post(axum::extract::Path(slug): axum::extract::Path<String>) -> Html<String> {
    let mut context = get_page_context();
    context.insert("page_content", &blog::post(slug));

    get_rendered_html("blog_post", context)
}
