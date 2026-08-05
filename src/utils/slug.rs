pub fn generate_slug(prefix: &str, title: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        title.trim().to_lowercase().replace(' ', "-")
    )
}