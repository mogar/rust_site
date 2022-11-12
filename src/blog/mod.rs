
/// Generate an index of blog posts
/// This will eventually iterate through markdown files in a
/// directory and populate an index at runtime
pub fn index() -> String {
    "<h1>Blog Incoming</h1>".to_string()
}

/// Render and display a single blog post
/// This will eventually parse and render a blog
/// post from a given markdown file.
pub fn post(slug: String) -> String {
    format!("<h1>post name</h1>
    <p>post id {}</p>", slug)
}