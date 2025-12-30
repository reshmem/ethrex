use bytes::Bytes;
use ethereum_types::Address;
use ethrex_crypto::slh_dsa::slh_sign;
use ethrex_l2_common::account_key::AccountPrivateKey;
use ethrex_common::types::FeeTokenTransaction;
use ethrex_common::utils::keccak;
use ethrex_common::{
    U256,
    types::{
        EIP1559Transaction, EIP2930Transaction, EIP4844Transaction, EIP7702Transaction,
        LegacyTransaction, Transaction, TxType,
    },
};
use ethrex_rlp::{encode::PayloadRLPEncode, structs::Encoder};
use reqwest::{Client, StatusCode, Url};
use hex::FromHexError;
use secp256k1::{Message, PublicKey, SECP256K1};
use serde::Serialize;
use url::ParseError;

#[derive(Clone, Debug)]
pub enum Signer {
    Local(LocalSigner),
    Remote(RemoteSigner),
}

#[derive(Clone, Serialize, PartialEq, Default)]
pub struct SignerHealth {
    signer: String,
    address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_signer_healthcheck: Option<serde_json::Value>,
}

impl Signer {
    pub async fn sign(&self, data: Bytes) -> Result<Bytes, SignerError> {
        match self {
            Self::Local(signer) => Ok(signer.sign(data)),
            Self::Remote(signer) => signer.sign(data).await,
        }
    }

    pub fn address(&self) -> Address {
        match self {
            Self::Local(signer) => signer.address,
            Self::Remote(signer) => signer.address,
        }
    }

    pub async fn health(&self) -> SignerHealth {
        match &self {
            Signer::Local(local) => SignerHealth {
                address: local.address,
                signer: "local".to_string(),
                ..Default::default()
            },
            Signer::Remote(remote) => SignerHealth {
                address: remote.address,
                public_key: Some(remote.public_key.to_string()),
                signer: "remote".to_string(),
                url: Some(remote.url.to_string()),
                remote_signer_healthcheck: Some(remote.health().await),
            },
        }
    }
}

impl From<LocalSigner> for Signer {
    fn from(value: LocalSigner) -> Self {
        Self::Local(value)
    }
}

impl From<RemoteSigner> for Signer {
    fn from(value: RemoteSigner) -> Self {
        Self::Remote(value)
    }
}

#[derive(Clone, Debug)]
pub struct LocalSigner {
    pub private_key: AccountPrivateKey,
    pub address: Address,
}

impl LocalSigner {
    pub fn new<K>(private_key: K) -> Self
    where
        K: Into<AccountPrivateKey>,
    {
        let private_key = private_key.into();
        let address = private_key.address();
        Self { private_key, address }
    }

    pub fn sign(&self, data: Bytes) -> Bytes {
        let hash = keccak(data);
        if let Some(slh_key) = self.private_key.slh_private_key() {
            let sig = match slh_sign(&hash.0, slh_key) {
                Ok(sig) => sig,
                Err(_) => return Bytes::new(),
            };
            return Bytes::from(sig.as_bytes().to_vec());
        }

        let Some(secp_key) = self.private_key.legacy_secp_key() else {
            return Bytes::new();
        };
        let msg = Message::from_digest(hash.0);
        let (recovery_id, signature) = SECP256K1
            .sign_ecdsa_recoverable(&msg, secp_key)
            .serialize_compact();

        Bytes::from(
            [
                signature.as_slice(),
                &[Into::<i32>::into(recovery_id) as u8],
            ]
            .concat(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct RemoteSigner {
    pub url: Url,
    pub public_key: PublicKey,
    pub address: Address,
}

impl RemoteSigner {
    pub fn new(url: Url, public_key: PublicKey) -> Self {
        let address = Address::from(keccak(&public_key.serialize_uncompressed()[1..]));
        Self {
            url,
            public_key,
            address,
        }
    }

    pub async fn sign(&self, data: Bytes) -> Result<Bytes, SignerError> {
        let url = self
            .url
            .join("api/v1/eth1/sign/")?
            .join(&hex::encode(&self.public_key.serialize_uncompressed()[1..]))?;
        let body = format!("{{\"data\": \"0x{}\"}}", hex::encode(data));

        let client = Client::new();
        let response = client
            .post(url)
            .body(body)
            .header("content-type", "application/json")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let text = response.text().await?;
                let text = text.trim_start_matches("0x");
                let bytes = hex::decode(text).map_err(SignerError::FromHexError)?;
                Ok(Bytes::from(bytes))
            }
            StatusCode::NOT_FOUND => Err(SignerError::Web3SignerError(
                "Private key not found in web3signer server".to_string(),
            )),
            StatusCode::BAD_REQUEST => Err(SignerError::Web3SignerError(
                "Bad request format".to_string(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(SignerError::Web3SignerError(
                "Internal server error".to_string(),
            )),
            _ => Err(SignerError::Web3SignerError(format!(
                "Unknown error {}",
                response.status().as_str(),
            ))),
        }
    }

    async fn health(&self) -> serde_json::Value {
        let Ok(url) = self.url.join("/healthcheck") else {
            return serde_json::Value::String(format!("Failed to create url from {}", self.url));
        };

        let client = Client::new();
        match client.get(url.clone()).send().await {
            Err(e) => serde_json::Value::String(format!("GET {} returned an error: {e}", url)),
            Ok(ok) => match ok.status() {
                StatusCode::OK | StatusCode::SERVICE_UNAVAILABLE => ok
                    .json::<serde_json::Value>()
                    .await
                    .unwrap_or_else(|e| serde_json::Value::String(e.to_string())),
                status => {
                    serde_json::Value::String(format!("GET {} returned an error: {}", url, status))
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("Url Parse Error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Failed with a reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse value: {0}")]
    FromHexError(#[from] FromHexError),
    #[error("Tried to sign Privileged L2 transaction")]
    PrivilegedL2TxUnsupported,
    #[error("Web3signer error: {0}")]
    Web3SignerError(String),
}

pub trait Signable {
    fn sign(
        &self,
        signer: &Signer,
    ) -> impl std::future::Future<Output = Result<Self, SignerError>> + Send
    where
        Self: Sized + Sync + Send + Clone,
    {
        async {
            let mut signable = self.clone();
            signable.sign_inplace(signer).await?;
            Ok(signable)
        }
    }

    fn sign_inplace(
        &mut self,
        signer: &Signer,
    ) -> impl std::future::Future<Output = Result<(), SignerError>> + Send;
}

impl Signable for Transaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        match self {
            Transaction::LegacyTransaction(tx) => tx.sign_inplace(signer).await,
            Transaction::EIP2930Transaction(tx) => tx.sign_inplace(signer).await,
            Transaction::EIP1559Transaction(tx) => tx.sign_inplace(signer).await,
            Transaction::EIP4844Transaction(tx) => tx.sign_inplace(signer).await,
            Transaction::EIP7702Transaction(tx) => tx.sign_inplace(signer).await,
            Transaction::PrivilegedL2Transaction(_) => Err(SignerError::PrivilegedL2TxUnsupported), // Privileged Transactions are not signed
            Transaction::FeeTokenTransaction(tx) => tx.sign_inplace(signer).await,
        }
    }
}

impl Signable for LegacyTransaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let chain_id = self.v.as_u64();
        let mut payload = Vec::new();
        Encoder::new(&mut payload)
            .encode_field(&self.nonce)
            .encode_field(&self.gas_price)
            .encode_field(&self.gas)
            .encode_field(&self.to)
            .encode_field(&self.value)
            .encode_field(&self.data)
            .encode_field(&chain_id)
            .encode_field(&0u8)
            .encode_field(&0u8)
            .finish();
        let signature = signer.sign(Bytes::from(payload)).await?;
        self.sig = signature;

        Ok(())
    }
}

impl Signable for EIP1559Transaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let mut payload = vec![TxType::EIP1559 as u8];
        payload.append(self.encode_payload_to_vec().as_mut());

        let signature = signer.sign(payload.into()).await?;
        self.sig = signature;
        self.v = U256::from(self.chain_id);

        Ok(())
    }
}

impl Signable for EIP2930Transaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let mut payload = vec![TxType::EIP2930 as u8];
        payload.append(self.encode_payload_to_vec().as_mut());

        let signature = signer.sign(payload.into()).await?;
        self.sig = signature;
        self.v = U256::from(self.chain_id);

        Ok(())
    }
}

impl Signable for EIP4844Transaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let mut payload = vec![TxType::EIP4844 as u8];
        payload.append(self.encode_payload_to_vec().as_mut());

        let signature = signer.sign(payload.into()).await?;
        self.sig = signature;
        self.v = U256::from(self.chain_id);

        Ok(())
    }
}

impl Signable for EIP7702Transaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let mut payload = vec![TxType::EIP7702 as u8];
        payload.append(self.encode_payload_to_vec().as_mut());

        let signature = signer.sign(payload.into()).await?;
        self.sig = signature;
        self.v = U256::from(self.chain_id);

        Ok(())
    }
}

impl Signable for FeeTokenTransaction {
    async fn sign_inplace(&mut self, signer: &Signer) -> Result<(), SignerError> {
        let mut payload = vec![TxType::FeeToken as u8];
        payload.append(self.encode_payload_to_vec().as_mut());

        let signature = signer.sign(payload.into()).await?;
        self.sig = signature;
        self.v = U256::from(self.chain_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethrex_crypto::slh_dsa::{SlhSignature, generate_slh_key, slh_verify};

    #[test]
    fn sign_message_and_verify() {
        let (sk, _) = generate_slh_key();
        let signer = LocalSigner::new(sk);
        let data = Bytes::from_static(b"test-message");
        let sig_bytes = signer.sign(data.clone());
        let sig = match SlhSignature::from_bytes(&sig_bytes) {
            Ok(value) => value,
            Err(err) => {
                assert!(false, "signature bytes should be valid: {err}");
                return;
            }
        };
        let hash = keccak(data);
        assert!(slh_verify(&hash.0, &sig));
    }
}
