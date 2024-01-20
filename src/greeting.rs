pub async fn greet() -> String {
    "Hello, World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_greet() {
        assert_eq!(greet().await, "Hello, World!");
    }
}