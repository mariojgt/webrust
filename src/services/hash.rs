use bcrypt::{hash, verify, DEFAULT_COST};

pub fn make(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn check(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}
