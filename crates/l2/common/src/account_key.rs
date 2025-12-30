use ethrex_common::{Address, utils::keccak};
use ethrex_crypto::slh_dsa::{SlhPrivateKey, slh_pubkey_to_address};
use thiserror::Error;

#[cfg(feature = "secp256k1")]
use secp256k1::SecretKey as LegacySecpKey;

#[cfg(not(feature = "secp256k1"))]
use k256::SecretKey as LegacySecpKey;

const LEGACY_SECP_KEY_LEN: usize = 32;
const SLH_KEY_LEN: usize = 128;

#[derive(Debug, Error)]
pub enum AccountKeyError {
    #[error("Invalid key length: {0}")]
    InvalidKeyLength(usize),
    #[error("Invalid legacy secp256k1 key: {0}")]
    InvalidLegacySecpKey(String),
    #[error("Invalid SLH-DSA key: {0}")]
    InvalidSlhKey(String),
    #[error("Failed to derive address: {0}")]
    AddressDerivationFailed(String),
}

#[derive(Clone)]
enum AccountPrivateKeyKind {
    Slh(SlhPrivateKey),
    LegacySecp(LegacySecpKey),
}

#[derive(Clone)]
pub struct AccountPrivateKey {
    kind: AccountPrivateKeyKind,
    address: Address,
}

impl std::fmt::Debug for AccountPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match &self.kind {
            AccountPrivateKeyKind::Slh(_) => "slh",
            AccountPrivateKeyKind::LegacySecp(_) => "legacy_secp256k1",
        };
        f.debug_struct("AccountPrivateKey")
            .field("kind", &kind)
            .field("address", &self.address)
            .finish()
    }
}

impl AccountPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AccountKeyError> {
        match bytes.len() {
            LEGACY_SECP_KEY_LEN => {
                let key = legacy_secp_key_from_bytes(bytes)?;
                Self::from_legacy_key(key)
            }
            SLH_KEY_LEN => {
                let key = SlhPrivateKey::from_bytes(bytes)
                    .map_err(|e| AccountKeyError::InvalidSlhKey(e.to_string()))?;
                Ok(Self::from_slh(key))
            }
            other => Err(AccountKeyError::InvalidKeyLength(other)),
        }
    }

    pub fn from_slh(key: SlhPrivateKey) -> Self {
        let address = slh_pubkey_to_address(&key.public_key());
        Self {
            kind: AccountPrivateKeyKind::Slh(key),
            address,
        }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn slh_private_key(&self) -> Option<&SlhPrivateKey> {
        match &self.kind {
            AccountPrivateKeyKind::Slh(key) => Some(key),
            _ => None,
        }
    }

    pub fn slh_private_key_bytes(&self) -> Option<[u8; SLH_KEY_LEN]> {
        self.slh_private_key().map(|key| key.to_bytes())
    }

    pub fn legacy_secp_bytes(&self) -> Option<[u8; LEGACY_SECP_KEY_LEN]> {
        match &self.kind {
            AccountPrivateKeyKind::LegacySecp(key) => Some(legacy_secp_bytes(key)),
            _ => None,
        }
    }

    #[cfg(feature = "secp256k1")]
    pub fn legacy_secp_key(&self) -> Option<&LegacySecpKey> {
        match &self.kind {
            AccountPrivateKeyKind::LegacySecp(key) => Some(key),
            _ => None,
        }
    }

    fn from_legacy_key(key: LegacySecpKey) -> Result<Self, AccountKeyError> {
        let address = legacy_secp_address(&key)?;
        Ok(Self {
            kind: AccountPrivateKeyKind::LegacySecp(key),
            address,
        })
    }
}

impl From<SlhPrivateKey> for AccountPrivateKey {
    fn from(value: SlhPrivateKey) -> Self {
        Self::from_slh(value)
    }
}

#[cfg(feature = "secp256k1")]
impl From<secp256k1::SecretKey> for AccountPrivateKey {
    fn from(value: secp256k1::SecretKey) -> Self {
        let address = legacy_secp_address(&value)
            .unwrap_or_else(|_| Address::zero());
        Self {
            kind: AccountPrivateKeyKind::LegacySecp(value),
            address,
        }
    }
}

fn legacy_secp_key_from_bytes(bytes: &[u8]) -> Result<LegacySecpKey, AccountKeyError> {
    LegacySecpKey::from_slice(bytes)
        .map_err(|e| AccountKeyError::InvalidLegacySecpKey(e.to_string()))
}

#[cfg(feature = "secp256k1")]
fn legacy_secp_bytes(key: &LegacySecpKey) -> [u8; LEGACY_SECP_KEY_LEN] {
    key.secret_bytes()
}

#[cfg(not(feature = "secp256k1"))]
fn legacy_secp_bytes(key: &LegacySecpKey) -> [u8; LEGACY_SECP_KEY_LEN] {
    key.to_bytes().into()
}

#[cfg(feature = "secp256k1")]
fn legacy_secp_address(key: &LegacySecpKey) -> Result<Address, AccountKeyError> {
    let public_key = key.public_key(secp256k1::SECP256K1).serialize_uncompressed();
    address_from_public_key(&public_key[1..])
}

#[cfg(not(feature = "secp256k1"))]
fn legacy_secp_address(key: &LegacySecpKey) -> Result<Address, AccountKeyError> {
    use k256::elliptic_curve::sec1::ToEncodedPoint;

    let public_key = key.public_key().to_encoded_point(false);
    address_from_public_key(
        public_key
            .as_bytes()
            .get(1..)
            .ok_or_else(|| {
                AccountKeyError::AddressDerivationFailed("Failed to slice public key".to_string())
            })?,
    )
}

fn address_from_public_key(public_key: &[u8]) -> Result<Address, AccountKeyError> {
    let hash = keccak(public_key);
    let address_slice = hash.as_ref().get(12..32).ok_or_else(|| {
        AccountKeyError::AddressDerivationFailed("Failed to slice address bytes".to_string())
    })?;
    let address_bytes: [u8; 20] = address_slice
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| {
            AccountKeyError::AddressDerivationFailed(e.to_string())
        })?;
    Ok(Address::from(address_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethrex_crypto::slh_dsa::generate_slh_key;

    #[test]
    fn address_derivation_uses_slh_pubkey_path() {
        let (sk, pk) = generate_slh_key();
        let key = AccountPrivateKey::from(sk);
        let expected = slh_pubkey_to_address(&pk);
        assert_eq!(key.address(), expected);
    }
}
