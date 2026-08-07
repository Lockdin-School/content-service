pub fn generate_code(prefix: &str) -> String {
    format!("{}-{}", prefix, chrono::Utc::now().timestamp_millis())
}
