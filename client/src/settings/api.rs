pub fn backend_url() -> String {
    let api_base_url = std::env::var("BACKEND_URL").unwrap_or("http://localhost:1123".to_string());
    api_base_url
}