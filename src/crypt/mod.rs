use aes_gcm::{
    Aes256Gcm, Key,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Context, Error};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::TryRngCore;
use base64::{engine::general_purpose, Engine as _};

const ARGON_SALT_LENGTH: usize = 16;
const ARGON_ITERATIONS: u32 = 3;
const ARGON_MEMORY: u32 = 64 * 1024;
const ARGON_THREADS: u32 = 4;
const ARGON_KEY_LENGTH: usize = 32;

pub struct Encrypted {
    salt: Vec<u8>,
    nonce: Vec<u8>,
    encoded: Vec<u8>,
}

impl Encrypted {
    /// Encodes the encrypted data into a base64 string
    pub fn to_string(&self) -> String {
        let mut combined = Vec::with_capacity(self.nonce.len() + self.salt.len() + self.encoded.len());
        combined.extend_from_slice(&self.nonce);
        combined.extend_from_slice(&self.salt);
        combined.extend_from_slice(&self.encoded);
        general_purpose::STANDARD.encode(&combined)
    }

}

pub fn encrypt(data: &[u8], password: &str) -> Result<Encrypted, Error> {
    // 1. generate argon key (random, password)
    let (key, salt) = generate_argon_key(password)?;

    let key_typed = Key::<Aes256Gcm>::from_slice(&key);

    // 2. create block cipher
    let cipher = Aes256Gcm::new(&key_typed);

    // 3. generate nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encoded = cipher
        .encrypt(&nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

    Ok(Encrypted {
        salt,
        nonce: nonce.to_vec(),
        encoded,
    })
}

fn generate_argon_key(password: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
    // generate salt
    let mut salt = vec![0u8; ARGON_SALT_LENGTH];

    rand::rngs::OsRng
        .try_fill_bytes(&mut salt)
        .map_err(|e| anyhow::anyhow!("Failed to generate random salt: {}", e))?;

    let key = derive_argon_key(&salt, password)?;

    Ok((key, salt))
}

fn derive_argon_key(salt: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    let params = Params::new(
        ARGON_MEMORY,
        ARGON_ITERATIONS,
        ARGON_THREADS,
        Some(ARGON_KEY_LENGTH),
    )
    .map_err(|e| anyhow::anyhow!("failed to create Argon2 params: {}", e))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = vec![0u8; ARGON_KEY_LENGTH];

    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut output)
        .map_err(|e| anyhow::anyhow!("Argon2 hashing failed: {}", e))?;

    Ok(output)
}

#[cfg(test)]
mod tests;
