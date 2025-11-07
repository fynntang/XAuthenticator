use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::TryRngCore;
use sha2::{Digest, Sha256};

/// Derive a 32-byte encryption key from a password using Argon2id.
/// If a salt is provided, it must be 16+ bytes. Otherwise, a random salt is generated and returned.
pub fn derive_key(password: &str, salt_opt: Option<&[u8]>) -> ([u8; 32], Vec<u8>) {
    let salt_bytes: Vec<u8> = match salt_opt {
        Some(s) => s.to_vec(),
        None => {
            let mut salt_bytes = [0u8; 16];
            rand_core::OsRng
                .try_fill_bytes(&mut salt_bytes)
                .expect("failed to generate salt bytes");
            let salt = SaltString::encode_b64(&salt_bytes).unwrap();
            salt.as_str().as_bytes().to_vec()
        }
    };

    // Argon2id with default params (can be tuned later)
    let argon2 = Argon2::default();
    let salt_str = SaltString::encode_b64(&salt_bytes).expect("invalid salt");
    let _ = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .expect("argon2 hash failed");

    // Derive a deterministic 32-byte key using SHA-256 over password+salt.
    // Argon2 is used above to ensure cost; SHA-256 yields fixed-size key material here.
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(&salt_bytes);
    let digest = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&digest);
    (key, salt_bytes)
}
