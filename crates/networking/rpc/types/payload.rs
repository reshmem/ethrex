use bytes::Bytes;
use ethrex_rlp::error::RLPDecodeError;
use serde::{Deserialize, Serialize};

use ethrex_common::{
    Address, Bloom, H256, U256,
    constants::DEFAULT_OMMERS_HASH,
    serde_utils,
    types::{
        BlobsBundle, Block, BlockBody, BlockHash, BlockHeader, Transaction, Withdrawal,
        compute_transactions_root, compute_withdrawals_root, requests::EncodedRequests,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayload {
    parent_hash: H256,
    fee_recipient: Address,
    state_root: H256,
    receipts_root: H256,
    logs_bloom: Bloom,
    prev_randao: H256,
    #[serde(with = "serde_utils::u64::hex_str")]
    pub block_number: u64,
    #[serde(with = "serde_utils::u64::hex_str")]
    gas_limit: u64,
    #[serde(with = "serde_utils::u64::hex_str")]
    gas_used: u64,
    #[serde(with = "serde_utils::u64::hex_str")]
    pub timestamp: u64,
    #[serde(with = "serde_utils::bytes")]
    extra_data: Bytes,
    #[serde(with = "serde_utils::u64::hex_str")]
    base_fee_per_gas: u64,
    pub block_hash: H256,
    transactions: Vec<EncodedTransaction>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub withdrawals: Option<Vec<Withdrawal>>,
    // ExecutionPayloadV3 fields. Optional since we support V2 too
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::u64::hex_str_opt",
        default
    )]
    pub blob_gas_used: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_utils::u64::hex_str_opt",
        default
    )]
    pub excess_blob_gas: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct EncodedTransaction(pub Bytes);

impl<'de> Deserialize<'de> for EncodedTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(EncodedTransaction(serde_utils::bytes::deserialize(
            deserializer,
        )?))
    }
}

impl Serialize for EncodedTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_utils::bytes::serialize(&self.0, serializer)
    }
}

impl EncodedTransaction {
    /// Based on [EIP-2718]
    /// Transactions can be encoded in the following formats:
    /// A) `TransactionType || Transaction` (Where Transaction type is an 8-bit number between 0 and 0x7f, and Transaction is an rlp encoded transaction of type TransactionType)
    /// B) `LegacyTransaction` (An rlp encoded LegacyTransaction)
    fn decode(&self) -> Result<Transaction, RLPDecodeError> {
        Transaction::decode_canonical(self.0.as_ref())
    }

    fn encode(tx: &Transaction) -> Self {
        Self(Bytes::from(tx.encode_canonical_to_vec()))
    }
}

impl ExecutionPayload {
    /// Converts an `ExecutionPayload` into a block (aka a BlockHeader and BlockBody)
    /// using the parentBeaconBlockRoot received along with the payload in the rpc call `engine_newPayloadV2/V3`
    pub fn into_block(
        self,
        parent_beacon_block_root: Option<H256>,
        requests_hash: Option<H256>,
    ) -> Result<Block, RLPDecodeError> {
        let body = BlockBody {
            transactions: self
                .transactions
                .iter()
                .map(|encoded_tx| encoded_tx.decode())
                .collect::<Result<Vec<_>, RLPDecodeError>>()?,
            ommers: vec![],
            withdrawals: self.withdrawals,
        };
        let header = BlockHeader {
            parent_hash: self.parent_hash,
            ommers_hash: *DEFAULT_OMMERS_HASH,
            coinbase: self.fee_recipient,
            state_root: self.state_root,
            transactions_root: compute_transactions_root(&body.transactions),
            receipts_root: self.receipts_root,
            logs_bloom: self.logs_bloom,
            difficulty: 0.into(),
            number: self.block_number,
            gas_limit: self.gas_limit,
            gas_used: self.gas_used,
            timestamp: self.timestamp,
            extra_data: self.extra_data,
            prev_randao: self.prev_randao,
            nonce: 0,
            base_fee_per_gas: Some(self.base_fee_per_gas),
            withdrawals_root: body
                .withdrawals
                .as_ref()
                .map(|w| compute_withdrawals_root(w)),
            blob_gas_used: self.blob_gas_used,
            excess_blob_gas: self.excess_blob_gas,
            parent_beacon_block_root,
            // TODO: set the value properly
            requests_hash,
            ..Default::default()
        };

        Ok(Block::new(header, body))
    }

    pub fn from_block(block: Block) -> Self {
        Self {
            parent_hash: block.header.parent_hash,
            fee_recipient: block.header.coinbase,
            state_root: block.header.state_root,
            receipts_root: block.header.receipts_root,
            logs_bloom: block.header.logs_bloom,
            prev_randao: block.header.prev_randao,
            block_number: block.header.number,
            gas_limit: block.header.gas_limit,
            gas_used: block.header.gas_used,
            timestamp: block.header.timestamp,
            extra_data: block.header.extra_data.clone(),
            base_fee_per_gas: block.header.base_fee_per_gas.unwrap_or_default(),
            block_hash: block.hash(),
            transactions: block
                .body
                .transactions
                .iter()
                .map(EncodedTransaction::encode)
                .collect(),
            withdrawals: block.body.withdrawals,
            blob_gas_used: block.header.blob_gas_used,
            excess_blob_gas: block.header.excess_blob_gas,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadStatus {
    pub status: PayloadValidationStatus,
    pub latest_valid_hash: Option<H256>,
    pub validation_error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PayloadValidationStatus {
    Valid,
    Invalid,
    Syncing,
    Accepted,
}

impl PayloadStatus {
    // Convenience methods to create payload status

    pub fn invalid_with(latest_valid_hash: H256, error: String) -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Invalid,
            latest_valid_hash: Some(latest_valid_hash),
            validation_error: Some(error),
        }
    }

    /// Creates a PayloadStatus with invalid status and error message
    pub fn invalid_with_err(error: &str) -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Invalid,
            latest_valid_hash: None,
            validation_error: Some(error.to_string()),
        }
    }

    /// Creates a PayloadStatus with invalid status and latest valid hash
    pub fn invalid_with_hash(hash: BlockHash) -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Invalid,
            latest_valid_hash: Some(hash),
            validation_error: None,
        }
    }

    /// Creates a PayloadStatus with syncing status and no other info
    pub fn syncing() -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Syncing,
            latest_valid_hash: None,
            validation_error: None,
        }
    }

    /// Creates a PayloadStatus with valid status and latest valid hash
    pub fn valid_with_hash(hash: BlockHash) -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Valid,
            latest_valid_hash: Some(hash),
            validation_error: None,
        }
    }
    /// Creates a PayloadStatus with valid status and latest valid hash
    pub fn valid() -> Self {
        PayloadStatus {
            status: PayloadValidationStatus::Valid,
            latest_valid_hash: None,
            validation_error: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadBody {
    pub transactions: Vec<EncodedTransaction>,
    pub withdrawals: Option<Vec<Withdrawal>>,
}

impl From<BlockBody> for ExecutionPayloadBody {
    fn from(body: BlockBody) -> Self {
        Self {
            transactions: body
                .transactions
                .iter()
                .map(EncodedTransaction::encode)
                .collect(),
            withdrawals: body.withdrawals,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadResponse {
    pub execution_payload: ExecutionPayload,
    // Total fees consumed by the block (fees paid)
    pub block_value: U256,
    pub blobs_bundle: Option<BlobsBundle>,
    pub should_override_builder: Option<bool>, // TODO: look into this
    pub execution_requests: Option<Vec<EncodedRequests>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadResponseV2 {
    pub execution_payload: ExecutionPayload,
    // Total fees consumed by the block (fees paid)
    pub block_value: U256,
}

#[cfg(test)]
mod test {
    use super::*;
    use ethrex_common::types::{EIP1559Transaction, TxKind, TxType};
    use ethrex_crypto::slh_dsa::{generate_slh_key, slh_sign};
    use ethrex_rlp::structs::Encoder;

    #[test]
    fn deserialize_payload_into_block() {
        let (sk, _pk) = generate_slh_key();
        let mut tx = EIP1559Transaction {
            chain_id: 1,
            nonce: 0,
            max_priority_fee_per_gas: 1,
            max_fee_per_gas: 1,
            gas_limit: 21_000,
            to: TxKind::Call(Address::from_low_u64_be(1)),
            value: U256::zero(),
            data: Bytes::new(),
            access_list: Default::default(),
            v: U256::from(1),
            sig: Bytes::new(),
            ..Default::default()
        };
        let mut payload_bytes = vec![TxType::EIP1559 as u8];
        Encoder::new(&mut payload_bytes)
            .encode_field(&tx.chain_id)
            .encode_field(&tx.nonce)
            .encode_field(&tx.max_priority_fee_per_gas)
            .encode_field(&tx.max_fee_per_gas)
            .encode_field(&tx.gas_limit)
            .encode_field(&tx.to)
            .encode_field(&tx.value)
            .encode_field(&tx.data)
            .encode_field(&tx.access_list)
            .finish();
        let sig = slh_sign(ethrex_common::utils::keccak(&payload_bytes).as_bytes(), &sk)
            .expect("sign");
        tx.sig = Bytes::from(sig.as_bytes().to_vec());

        let payload = ExecutionPayload {
            parent_hash: H256::zero(),
            fee_recipient: Address::zero(),
            state_root: H256::zero(),
            receipts_root: H256::zero(),
            logs_bloom: Bloom::default(),
            prev_randao: H256::zero(),
            block_number: 1,
            gas_limit: 30_000_000,
            gas_used: 21_000,
            timestamp: 1,
            extra_data: Bytes::new(),
            base_fee_per_gas: 1,
            block_hash: H256::zero(),
            transactions: vec![EncodedTransaction::encode(&Transaction::EIP1559Transaction(tx))],
            withdrawals: Some(vec![]),
            blob_gas_used: Some(0),
            excess_blob_gas: Some(0),
        };
        assert!(payload.into_block(Some(H256::zero()), None).is_ok());
    }
}
