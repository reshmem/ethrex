use getrandom::getrandom;
use ethereum_types::Address;
use slh_dsa::{Shake256f, Signature, SigningKey, VerifyingKey};
use slh_dsa::signature::rand_core::{CryptoRng, RngCore};
use slh_dsa::signature::{Signer, Verifier};
use thiserror::Error;

pub const PUBKEY_LEN: usize = 64;
pub const SIG_LEN: usize = 49_856;
pub const SIG_WITH_PUBKEY_LEN: usize = 49_920;
const PRIVKEY_LEN: usize = 128;
const HASH_LEN: usize = 32;

#[derive(Debug, Error)]
pub enum SlhError {
    #[error("Invalid message length: {0}")]
    InvalidMessageLen(usize),
    #[error("Invalid public key length: {0}")]
    InvalidPublicKeyLen(usize),
    #[error("Invalid private key length: {0}")]
    InvalidPrivateKeyLen(usize),
    #[error("Invalid signature length: {0}")]
    InvalidSignatureLen(usize),
    #[error("Signature verification failed")]
    InvalidSignature,
    #[error("Signature creation failed")]
    SignFailed,
}

#[derive(Clone)]
pub struct SlhPrivateKey {
    inner: SigningKey<Shake256f>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlhPublicKey {
    inner: VerifyingKey<Shake256f>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlhSignature {
    bytes: Vec<u8>,
}

struct OsRng;

impl RngCore for OsRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_be_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_be_bytes(bytes)
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        getrandom(dst).expect("os random source must be available");
    }
}

impl CryptoRng for OsRng {}

impl SlhPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SlhError> {
        if bytes.len() != PRIVKEY_LEN {
            return Err(SlhError::InvalidPrivateKeyLen(bytes.len()));
        }
        let inner = SigningKey::<Shake256f>::try_from(bytes).map_err(|_| {
            SlhError::InvalidPrivateKeyLen(bytes.len())
        })?;
        Ok(Self { inner })
    }

    pub fn to_bytes(&self) -> [u8; PRIVKEY_LEN] {
        let bytes = self.inner.to_bytes();
        let mut out = [0u8; PRIVKEY_LEN];
        out.copy_from_slice(bytes.as_slice());
        out
    }

    pub fn public_key(&self) -> SlhPublicKey {
        SlhPublicKey {
            inner: self.inner.as_ref().clone(),
        }
    }
}

impl SlhPublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SlhError> {
        if bytes.len() != PUBKEY_LEN {
            return Err(SlhError::InvalidPublicKeyLen(bytes.len()));
        }
        let inner = VerifyingKey::<Shake256f>::try_from(bytes)
            .map_err(|_| SlhError::InvalidPublicKeyLen(bytes.len()))?;
        Ok(Self { inner })
    }

    pub fn to_bytes(&self) -> [u8; PUBKEY_LEN] {
        let bytes = self.inner.to_bytes();
        let mut out = [0u8; PUBKEY_LEN];
        out.copy_from_slice(bytes.as_slice());
        out
    }
}

impl SlhSignature {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SlhError> {
        if bytes.len() != SIG_WITH_PUBKEY_LEN {
            return Err(SlhError::InvalidSignatureLen(bytes.len()));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub fn generate_slh_key() -> (SlhPrivateKey, SlhPublicKey) {
    let mut rng = OsRng;
    let sk = SigningKey::<Shake256f>::new(&mut rng);
    let pk = sk.as_ref().clone();
    (
        SlhPrivateKey { inner: sk },
        SlhPublicKey { inner: pk },
    )
}

pub fn slh_sign(hash: &[u8], sk: &SlhPrivateKey) -> Result<SlhSignature, SlhError> {
    if hash.len() != HASH_LEN {
        return Err(SlhError::InvalidMessageLen(hash.len()));
    }
    let sig = sk.inner.try_sign(hash).map_err(|_| SlhError::SignFailed)?;
    let mut bytes = Vec::with_capacity(SIG_WITH_PUBKEY_LEN);
    bytes.extend_from_slice(&sig.to_vec());
    bytes.extend_from_slice(sk.public_key().to_bytes().as_slice());
    debug_assert_eq!(bytes.len(), SIG_WITH_PUBKEY_LEN);
    Ok(SlhSignature { bytes })
}

pub fn slh_verify(hash: &[u8], sig_with_pk: &SlhSignature) -> bool {
    if hash.len() != HASH_LEN {
        return false;
    }
    if sig_with_pk.bytes.len() != SIG_WITH_PUBKEY_LEN {
        return false;
    }
    let (sig_bytes, pk_bytes) = sig_with_pk.bytes.split_at(SIG_LEN);
    let sig = match Signature::<Shake256f>::try_from(sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let pk = match VerifyingKey::<Shake256f>::try_from(pk_bytes) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    pk.verify(hash, &sig).is_ok()
}

pub fn slh_recover(hash: &[u8], sig_with_pk: &SlhSignature) -> Result<SlhPublicKey, SlhError> {
    if !slh_verify(hash, sig_with_pk) {
        return Err(SlhError::InvalidSignature);
    }
    SlhPublicKey::from_bytes(&sig_with_pk.bytes[SIG_LEN..])
}

pub fn slh_pubkey_to_address(pk: &SlhPublicKey) -> Address {
    let pk_bytes = pk.to_bytes();
    let hash = crate::keccak::keccak_hash(&pk_bytes[1..]);
    Address::from_slice(&hash[12..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen_sign_verify_roundtrip() {
        let (sk, _pk) = generate_slh_key();
        let hash = [0x42u8; HASH_LEN];
        let sig = slh_sign(&hash, &sk).expect("signing should succeed");
        assert!(slh_verify(&hash, &sig));
    }

    #[test]
    fn signature_sizes_match() {
        let (sk, _pk) = generate_slh_key();
        let hash = [0x11u8; HASH_LEN];
        let sig = slh_sign(&hash, &sk).expect("signing should succeed");
        assert_eq!(sig.as_bytes().len(), SIG_WITH_PUBKEY_LEN);
        assert_eq!(SIG_WITH_PUBKEY_LEN, SIG_LEN + PUBKEY_LEN);
        assert_eq!(SIG_LEN, 49_856);
        assert_eq!(PUBKEY_LEN, 64);
    }

    #[test]
    fn recover_pubkey_from_signature() {
        let (sk, pk) = generate_slh_key();
        let hash = [0x99u8; HASH_LEN];
        let sig = slh_sign(&hash, &sk).expect("signing should succeed");
        let recovered = slh_recover(&hash, &sig).expect("recover should succeed");
        assert_eq!(recovered, pk);
    }

    #[test]
    fn address_derivation_matches_quranium_style() {
        let (_sk, pk) = generate_slh_key();
        let pk_bytes = pk.to_bytes();
        let expected_hash = crate::keccak::keccak_hash(&pk_bytes[1..]);
        let expected = Address::from_slice(&expected_hash[12..]);
        let derived = slh_pubkey_to_address(&pk);
        assert_eq!(derived, expected);
    }

    #[test]
    fn slh_recover_validates_signature() {
        let (sk, _pk) = generate_slh_key();
        let hash = [0x01u8; HASH_LEN];
        let mut sig = slh_sign(&hash, &sk).expect("signing should succeed");
        let bytes = sig.bytes.as_mut_slice();
        bytes[0] ^= 0x01;
        assert!(slh_recover(&hash, &sig).is_err());
    }
}
