#[cfg(test)]
mod tests {
    use dodopayments::{configs::Config, storage::Storage};

    /// Tests for the storage module.
    #[tokio::test]
    async fn test_storage_connection() {
        // Load the config.
        let config = Config::new().unwrap();
        // Create a new storage instance.
        let storage = Storage::new(&config.database).await;
        // Assert that the storage connection is successful.
        assert!(storage.is_ok());
    }
}
