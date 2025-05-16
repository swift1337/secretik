use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::Result;
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose, Engine as _};
use rand::TryRngCore;

const ARGON_SALT_LENGTH: usize = 16;
const ARGON_ITERATIONS: u32 = 3;
const ARGON_MEMORY: u32 = 64 * 1024;
const ARGON_THREADS: u32 = 4;
const ARGON_KEY_LENGTH: usize = 32;

const AES_NONCE_LENGTH: usize = 12;

pub struct Encrypted {
    salt: Vec<u8>,
    nonce: Vec<u8>,
    encoded: Vec<u8>,
}

impl Encrypted {
    /// Encodes the encrypted data into a base64 string
    pub fn to_base64(&self) -> String {
        let cap = self.nonce.len() + self.salt.len() + self.encoded.len();
        let mut combined = Vec::with_capacity(cap);

        combined.extend_from_slice(&self.nonce);
        combined.extend_from_slice(&self.salt);
        combined.extend_from_slice(&self.encoded);

        general_purpose::STANDARD.encode(&combined)
    }

    /// Decodes the encrypted data from a base64 string
    pub fn from_base64(s: &str) -> Result<Self> {
        let data = general_purpose::STANDARD
            .decode(s)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))?;

        if data.len() < (ARGON_SALT_LENGTH + AES_NONCE_LENGTH + 1) {
            return Err(anyhow::anyhow!(
                "Invalid encrypted data length: {}",
                data.len()
            ));
        }

        let nonce = data[..AES_NONCE_LENGTH].to_vec();
        let salt = data[AES_NONCE_LENGTH..AES_NONCE_LENGTH + ARGON_SALT_LENGTH].to_vec();
        let encoded = data[AES_NONCE_LENGTH + ARGON_SALT_LENGTH..].to_vec();

        Ok(Self {
            nonce,
            salt,
            encoded,
        })
    }
}

pub fn encrypt(data: &[u8], password: &str) -> Result<Encrypted> {
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

pub fn decrypt(base64_data: &str, password: &str) -> Result<Vec<u8>> {
    let encrypted = Encrypted::from_base64(base64_data)?;

    let key = derive_argon_key(&encrypted.salt, password)?;
    let key_typed = Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(&key_typed);

    let nonce = Nonce::from_slice(encrypted.nonce.as_ref());

    let decrypted = cipher
        .decrypt(&nonce, encrypted.encoded.as_ref())
        .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;

    Ok(decrypted)
}

fn generate_argon_key(password: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    // generate salt
    let mut salt = vec![0u8; ARGON_SALT_LENGTH];

    rand::rngs::OsRng
        .try_fill_bytes(&mut salt)
        .map_err(|e| anyhow::anyhow!("Failed to generate random salt: {}", e))?;

    let key = derive_argon_key(&salt, password)?;

    Ok((key, salt))
}

fn derive_argon_key(salt: &[u8], password: &str) -> Result<Vec<u8>> {
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
