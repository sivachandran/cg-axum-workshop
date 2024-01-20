use axum::extract::Path;

pub async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_greet() {
        assert_eq!(greet(Path("World".to_string())).await, "Hello, World!");
    }
}