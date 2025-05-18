#[cfg(test)]
mod tests {
    use dodopayments::{configs::Config, storage::Storage};

    #[tokio::test]
    async fn test_storage_connection() {
        let config = Config::new().unwrap();
        let storage = Storage::new(&config.database).await;
        assert!(storage.is_ok());
    }
}
