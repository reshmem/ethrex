use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
    Nonce,
};
use crate::account_key::AccountPrivateKey;
use ethrex_crypto::slh_dsa::SlhPrivateKey;
use pbkdf2::pbkdf2_hmac;
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{fs, path::Path};
use thiserror::Error;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KDF_ITERATIONS: u32 = 100_000;

#[derive(Debug, Error)]
pub enum KeystoreError {
    #[error("Failed to encrypt data")]
    EncryptFailed,
    #[error("Failed to decrypt data")]
    DecryptFailed,
    #[error("Failed to parse hex: {0}")]
    HexDecodeFailed(String),
    #[error("Failed to parse SLH private key: {0}")]
    InvalidSlhKey(String),
    #[error("Failed to parse account key: {0}")]
    InvalidAccountKey(String),
    #[error("Failed to read keystore file: {0}")]
    ReadFailed(String),
    #[error("Failed to write keystore file: {0}")]
    WriteFailed(String),
    #[error("Failed to deserialize keystore file: {0}")]
    DeserializeFailed(String),
    #[error("Missing key material for keystore")]
    MissingKeyMaterial,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountKeyType {
    Slh,
    LegacySecp256k1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountKeystore {
    version: u8,
    key_type: AccountKeyType,
    kdf: String,
    kdf_iterations: u32,
    salt: String,
    nonce: String,
    ciphertext: String,
}

impl AccountKeystore {
    pub fn encrypt_account_key(
        key: &AccountPrivateKey,
        password: &str,
    ) -> Result<Self, KeystoreError> {
        if let Some(slh_key) = key.slh_private_key() {
            return Self::encrypt_slh(slh_key, password);
        }
        if let Some(legacy_bytes) = key.legacy_secp_bytes() {
            return Self::encrypt_legacy(&legacy_bytes, password);
        }
        Err(KeystoreError::MissingKeyMaterial)
    }

    pub fn encrypt_slh(private_key: &SlhPrivateKey, password: &str) -> Result<Self, KeystoreError> {
        let plaintext = private_key.to_bytes();
        Self::encrypt_bytes(AccountKeyType::Slh, &plaintext, password)
    }

    pub fn encrypt_legacy(
        legacy_private_key_bytes: &[u8],
        password: &str,
    ) -> Result<Self, KeystoreError> {
        Self::encrypt_bytes(AccountKeyType::LegacySecp256k1, legacy_private_key_bytes, password)
    }

    pub fn decrypt_account_key(&self, password: &str) -> Result<AccountPrivateKey, KeystoreError> {
        let plaintext = self.decrypt_bytes(password)?;
        AccountPrivateKey::from_bytes(&plaintext)
            .map_err(|e| KeystoreError::InvalidAccountKey(e.to_string()))
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), KeystoreError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| KeystoreError::WriteFailed(e.to_string()))?;
        fs::write(path, json).map_err(|e| KeystoreError::WriteFailed(e.to_string()))
    }

    pub fn read_from_file(path: &Path) -> Result<Self, KeystoreError> {
        let contents =
            fs::read_to_string(path).map_err(|e| KeystoreError::ReadFailed(e.to_string()))?;
        serde_json::from_str(&contents)
            .map_err(|e| KeystoreError::DeserializeFailed(e.to_string()))
    }

    fn encrypt_bytes(
        key_type: AccountKeyType,
        plaintext: &[u8],
        password: &str,
    ) -> Result<Self, KeystoreError> {
        let mut salt = [0u8; SALT_LEN];
        let mut nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);

        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, KDF_ITERATIONS, &mut key);
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| KeystoreError::EncryptFailed)?;

        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| KeystoreError::EncryptFailed)?;

        Ok(Self {
            version: 1,
            key_type,
            kdf: "pbkdf2-sha256".to_string(),
            kdf_iterations: KDF_ITERATIONS,
            salt: hex::encode(salt),
            nonce: hex::encode(nonce),
            ciphertext: hex::encode(ciphertext),
        })
    }

    fn decrypt_bytes(&self, password: &str) -> Result<Vec<u8>, KeystoreError> {
        let salt = hex::decode(&self.salt).map_err(|e| KeystoreError::HexDecodeFailed(e.to_string()))?;
        let nonce =
            hex::decode(&self.nonce).map_err(|e| KeystoreError::HexDecodeFailed(e.to_string()))?;
        let ciphertext = hex::decode(&self.ciphertext)
            .map_err(|e| KeystoreError::HexDecodeFailed(e.to_string()))?;

        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, self.kdf_iterations, &mut key);
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| KeystoreError::DecryptFailed)?;

        cipher
            .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
            .map_err(|_| KeystoreError::DecryptFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethrex_crypto::slh_dsa::generate_slh_key;

    #[test]
    fn encrypt_decrypt_roundtrip_slh() {
        let (sk, _) = generate_slh_key();
        let keystore = match AccountKeystore::encrypt_slh(&sk, "correct horse battery staple") {
            Ok(value) => value,
            Err(err) => {
                assert!(false, "keystore encryption failed: {err}");
                return;
            }
        };
        let recovered = match keystore.decrypt_account_key("correct horse battery staple") {
            Ok(value) => value,
            Err(err) => {
                assert!(false, "keystore decryption failed: {err}");
                return;
            }
        };
        let recovered_bytes = match recovered.slh_private_key_bytes() {
            Some(value) => value,
            None => {
                assert!(false, "keystore returned non-SLH key");
                return;
            }
        };
        assert_eq!(sk.to_bytes(), recovered_bytes);
    }
}
