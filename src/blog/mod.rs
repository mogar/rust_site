
/// Generate an index of blog posts
/// This will eventually iterate through markdown files in a
/// directory and populate an index at runtime
pub fn index() -> String {
    // TODO: get list blog posts in directory

    // for file in posts
    // generate summary and title
    // images?

    markdown::to_html("# Blog _Incoming_")
}

/// Render and display a single blog post
/// This will eventually parse and render a blog
/// post from a given markdown file.
pub fn post(slug: String) -> String {
    // TODO: convert slug to filename

    // TODO: read entire file into string md

    // TODO: extract metadata (SEO stuff??)

    // TODO: format SEO stuff separately

    let md = format!("# post name\n\npost id {}", slug);
    markdown::to_html(&md)
}