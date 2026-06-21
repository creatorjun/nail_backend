// src/infrastructure/crypto.rs
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;
use std::env;

fn cipher() -> Result<Aes256Gcm> {
    let key_b64 = env::var("SOCIAL_ID_ENCRYPTION_KEY_BASE64")?;
    let key_bytes = STANDARD.decode(key_b64)?;
    if key_bytes.len() != 32 {
        return Err(anyhow!("SOCIAL_ID_ENCRYPTION_KEY_BASE64 must decode to 32 bytes"));
    }
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Ok(Aes256Gcm::new(key))
}

pub fn encrypt_social_id(plain: &str) -> Result<String> {
    let cipher = cipher()?;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plain.as_bytes()).map_err(|_| anyhow!("encryption failed"))?;
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    Ok(STANDARD.encode(combined))
}

pub fn decrypt_social_id(encoded: &str) -> Result<String> {
    let cipher = cipher()?;
    let combined = STANDARD.decode(encoded)?;
    if combined.len() < 13 {
        return Err(anyhow!("invalid encrypted social id"));
    }
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plain = cipher.decrypt(nonce, ciphertext).map_err(|_| anyhow!("decryption failed"))?;
    Ok(String::from_utf8(plain)?)
}
