#[cfg(test)]
mod tests {
    use dodopayments::utils::generate_jwt;

    /// Tests for the utils module.
    #[test]
    fn test_generate_jwt() {
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        struct TestClaims {
            sub: String,
            company: String,
        }

        let claims = TestClaims {
            sub: "test".to_string(),
            company: "dodopayments".to_string(),
        };

        let jwt_secret = "secret".to_string();

        let token_result = generate_jwt(&claims, &jwt_secret);

        assert!(token_result.is_ok());
    }
}
