use pbkdf2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Pbkdf2,
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn validate_password(password: &str, password_hash: String) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    let is_valid = Pbkdf2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}

#[cfg(test)]
mod password {
    use pbkdf2::password_hash::{PasswordHash, PasswordVerifier};
    use pbkdf2::Pbkdf2;

    #[test]
    fn test_hash_password() {
        let password = "password";
        let password_hash = super::hash_password(password).unwrap();
        let parsed_hash = PasswordHash::new(&password_hash).unwrap();

        assert_eq!(
            Pbkdf2.verify_password(password.as_bytes(), &parsed_hash),
            Ok(())
        );
    }
    #[test]
    fn test_validate_password_success() {
        let password = "password";
        let password_hash = super::hash_password(password).unwrap();

        assert_eq!(
            super::validate_password(password, password_hash).unwrap(),
            true
        );
    }
    #[test]
    fn test_validate_password_fail() {
        let password = "password";
        let password_hash = super::hash_password(password).unwrap();

        assert_eq!(
            super::validate_password("fake_password", password_hash).unwrap(),
            false
        );
    }
}
