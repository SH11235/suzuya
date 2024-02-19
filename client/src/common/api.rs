pub fn backend_url() -> String {
    std::env::var("BACKEND_URL").unwrap_or("http://localhost:1123".to_string())
}

pub const RELEASE_DATE_TEXT: &str = "発売日未定";
