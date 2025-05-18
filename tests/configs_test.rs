#[cfg(test)]
mod tests {
    use dodopayments::configs::Config;

    /// Tests for the configuration module.
    #[test]
    fn test_load_config() {
        // Test that the config can be loaded successfully.
        let config = Config::new();
        assert!(config.is_ok());
    }

    /// Tests the values of the configuration.
    #[test]
    fn test_config_values() {
        // Load the config.
        let config = Config::new().unwrap();
        // Assert that the config values are as expected.
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3001);
        assert_eq!(config.database.username, "db_user");
        assert_eq!(config.database.dbname, "dodopayments");
    }
}
