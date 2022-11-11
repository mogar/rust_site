
pub fn index() -> String {
    "Blog Incoming".to_string()
}

pub fn post(slug: String) -> String {
    format!("post id {}", slug)
}