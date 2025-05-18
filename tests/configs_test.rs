#[cfg(test)]
mod tests {
    use dodopayments::configs::Config;

    #[test]
    fn test_load_config() {
        let config = Config::new();
        assert!(config.is_ok());
    }

    #[test]
    fn test_config_values() {
        let config = Config::new().unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3001);
        assert_eq!(config.database.username, "db_user");
        assert_eq!(config.database.dbname, "dodopayments");
    }
}
