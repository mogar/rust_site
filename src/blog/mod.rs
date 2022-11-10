use axum::response::Html;

pub fn index() -> Html<String> {
    Html(format!("<p>{}</p>", "Blog Incoming"))
}

pub fn post(slug: String) -> Html<String> {
    Html(format!("<p>post id {}</p>", slug))
}