use zero2prod::health_check;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert_eq!(response, "Hello World");
    }
}
