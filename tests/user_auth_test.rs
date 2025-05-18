#[cfg(test)]
mod tests {
    use dodopayments::routes::user::password::{generate_password_hash, is_correct_password};
    use dodopayments::utils::{generate_jwt};
    use serde::Serialize;

    #[tokio::test]
    async fn test_generate_password_hash() {
        let password = "Password123!".to_string();
        let hash_result = generate_password_hash(password);
        assert!(hash_result.is_ok());
    }

    #[tokio::test]
    async fn test_is_correct_password() {
        let password = "Password123!".to_string();
        let hash = generate_password_hash(password.clone()).unwrap();
        let is_correct = is_correct_password(&password, &hash).unwrap();
        assert!(is_correct);

        let is_incorrect = is_correct_password(&"WrongPassword".to_string(), &hash).unwrap();
        assert!(!is_incorrect);
    }

    #[tokio::test]
    async fn test_generate_jwt() {
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
