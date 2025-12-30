use secp256k1::{
    Message, PublicKey, SecretKey,
    ecdsa::{RecoverableSignature, RecoveryId},
};
use thiserror::Error;

pub const EC_SIGNATURE_LEN: usize = 65;

#[derive(Debug, Error)]
pub enum EcError {
    #[error("Invalid message length: {0}")]
    InvalidMessageLen(usize),
    #[error("Invalid signature length: {0}")]
    InvalidSignatureLen(usize),
    #[error("Invalid public key length: {0}")]
    InvalidPublicKeyLen(usize),
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid signature")]
    InvalidSignature,
}

pub fn ec_signature_len() -> usize {
    EC_SIGNATURE_LEN
}

pub fn ec_sign(hash: &[u8], secret_key: &SecretKey) -> Result<[u8; EC_SIGNATURE_LEN], EcError> {
    if hash.len() != 32 {
        return Err(EcError::InvalidMessageLen(hash.len()));
    }
    let msg = Message::from_digest_slice(hash).map_err(|_| EcError::InvalidMessageLen(hash.len()))?;
    let sig = secp256k1::SECP256K1.sign_ecdsa_recoverable(&msg, secret_key);
    let (recovery_id, sig_bytes) = sig.serialize_compact();
    let mut out = [0u8; EC_SIGNATURE_LEN];
    out[..64].copy_from_slice(&sig_bytes);
    out[64] = i32::from(recovery_id) as u8;
    Ok(out)
}

pub fn ec_verify(hash: &[u8], signature: &[u8], public_key: &PublicKey) -> bool {
    if hash.len() != 32 || signature.len() != EC_SIGNATURE_LEN {
        return false;
    }
    let msg = match Message::from_digest_slice(hash) {
        Ok(msg) => msg,
        Err(_) => return false,
    };
    let recovery_id = match RecoveryId::try_from(signature[64] as i32) {
        Ok(id) => id,
        Err(_) => return false,
    };
    let sig = match RecoverableSignature::from_compact(&signature[..64], recovery_id) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let recovered = match secp256k1::SECP256K1.recover_ecdsa(&msg, &sig) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    recovered == *public_key
}

pub fn ec_compress(public_key_uncompressed: &[u8]) -> Result<[u8; 33], EcError> {
    let key = match public_key_uncompressed.len() {
        64 => {
            let mut buf = [0u8; 65];
            buf[0] = 0x04;
            buf[1..].copy_from_slice(public_key_uncompressed);
            PublicKey::from_slice(&buf)
        }
        65 => PublicKey::from_slice(public_key_uncompressed),
        other => return Err(EcError::InvalidPublicKeyLen(other)),
    }
    .map_err(|_| EcError::InvalidPublicKeyLen(public_key_uncompressed.len()))?;
    Ok(key.serialize())
}

pub fn ec_decompress(public_key_compressed: &[u8]) -> Result<[u8; 64], EcError> {
    if public_key_compressed.len() != 33 {
        return Err(EcError::InvalidPublicKeyLen(public_key_compressed.len()));
    }
    let key = PublicKey::from_slice(public_key_compressed)
        .map_err(|_| EcError::InvalidPublicKeyLen(public_key_compressed.len()))?;
    let uncompressed = key.serialize_uncompressed();
    let mut out = [0u8; 64];
    out.copy_from_slice(&uncompressed[1..]);
    Ok(out)
}
