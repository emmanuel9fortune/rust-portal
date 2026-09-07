use argon2::{
    password_hash::{
        phc::PasswordHash,
        PasswordHasher,
        PasswordVerifier,
    },
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|error| error.to_string())?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, String> {
    let parsed_hash =
        PasswordHash::new(password_hash)
            .map_err(|error| error.to_string())?;

    let argon2 = Argon2::default();

    match argon2.verify_password(
        password.as_bytes(),
        &parsed_hash,
    ) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_and_verify_work() {
        let password = "MyPAssword123!";

        let hash = hash_password(password).unwrap();

        assert_ne!(hash, password);

        assert!(verify_password(password, &hash).unwrap());

        assert!(!verify_password("WrongPAssword123!", &hash).unwrap());

    }
}