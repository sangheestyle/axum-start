use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

const SALT_LENGTH: usize = 16;

pub fn generate_salt() -> String {
    let salt: [u8; SALT_LENGTH] = rand::thread_rng().gen();
    general_purpose::STANDARD.encode(&salt)
}

pub fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn verify_password(password: &str, password_hash: &str, salt: &str) -> bool {
    hash_password(password, salt) == password_hash
}
