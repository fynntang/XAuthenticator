use anyhow::{anyhow, Result};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};

pub fn encrypt_xchacha20poly1305(plaintext: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>)> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let mut nonce_bytes = [0u8; 24];
    getrandom::fill(&mut nonce_bytes)?;
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| anyhow!("encryption failed"))?;
    Ok((nonce_bytes.to_vec(), ciphertext))
}

#[allow(dead_code)]
pub fn decrypt_xchacha20poly1305(
    ciphertext: &[u8],
    nonce: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>> {
    if nonce.len() != 24 {
        return Err(anyhow!("invalid nonce size: {}", nonce.len()));
    }
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XNonce::from_slice(nonce);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("decryption failed"))?;
    Ok(plaintext)
}
