use crate::utils::RpcErr;
use ethrex_common::{
    Address, H256, serde_utils,
    types::{
        BlockHash, BlockNumber, EIP1559Transaction, EIP2930Transaction, EIP7702Transaction,
        FeeTokenTransaction, LegacyTransaction, PrivilegedL2Transaction, Transaction,
        WrappedEIP4844Transaction,
    },
};
use ethrex_rlp::{decode::RLPDecode, error::RLPDecodeError};
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    #[serde(flatten)]
    pub tx: Transaction,
    #[serde(with = "serde_utils::u64::hex_str_opt")]
    block_number: Option<BlockNumber>,
    block_hash: Option<BlockHash>,
    from: Address,
    pub hash: H256,
    #[serde(with = "serde_utils::u64::hex_str_opt")]
    transaction_index: Option<u64>,
}

impl RpcTransaction {
    pub fn build(
        tx: Transaction,
        block_number: Option<BlockNumber>,
        block_hash: Option<BlockHash>,
        transaction_index: Option<usize>,
    ) -> Result<Self, RpcErr> {
        let from = tx.sender()?;
        let hash = tx.hash();
        let transaction_index = transaction_index.map(|n| n as u64);
        Ok(RpcTransaction {
            tx,
            block_number,
            block_hash,
            from,
            hash,
            transaction_index,
        })
    }
}

#[derive(Debug)]
pub enum SendRawTransactionRequest {
    Legacy(LegacyTransaction),
    EIP2930(EIP2930Transaction),
    EIP1559(EIP1559Transaction),
    EIP4844(WrappedEIP4844Transaction),
    EIP7702(EIP7702Transaction),
    PrivilegedL2(PrivilegedL2Transaction),
    FeeToken(FeeTokenTransaction),
}

impl SendRawTransactionRequest {
    pub fn to_transaction(&self) -> Transaction {
        match self {
            SendRawTransactionRequest::Legacy(t) => Transaction::LegacyTransaction(t.clone()),
            SendRawTransactionRequest::EIP1559(t) => Transaction::EIP1559Transaction(t.clone()),
            SendRawTransactionRequest::EIP2930(t) => Transaction::EIP2930Transaction(t.clone()),
            SendRawTransactionRequest::EIP4844(t) => Transaction::EIP4844Transaction(t.tx.clone()),
            SendRawTransactionRequest::EIP7702(t) => Transaction::EIP7702Transaction(t.clone()),
            SendRawTransactionRequest::PrivilegedL2(t) => {
                Transaction::PrivilegedL2Transaction(t.clone())
            }
            SendRawTransactionRequest::FeeToken(t) => Transaction::FeeTokenTransaction(t.clone()),
        }
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, RLPDecodeError> {
        // Look at the first byte to check if it corresponds to a TransactionType
        match bytes.first() {
            // First byte is a valid TransactionType https://eips.ethereum.org/EIPS/eip-2718#transactiontype-only-goes-up-to-0x7f
            Some(tx_type) if *tx_type <= 0x7f => {
                // Decode tx based on type
                let tx_bytes = &bytes[1..];

                match *tx_type {
                    // Legacy
                    0x0 => {
                        LegacyTransaction::decode(tx_bytes).map(SendRawTransactionRequest::Legacy)
                    }
                    // EIP2930
                    0x1 => {
                        EIP2930Transaction::decode(tx_bytes).map(SendRawTransactionRequest::EIP2930)
                    }
                    // EIP1559
                    0x2 => {
                        EIP1559Transaction::decode(tx_bytes).map(SendRawTransactionRequest::EIP1559)
                    }
                    // EIP4844
                    0x3 => WrappedEIP4844Transaction::decode(tx_bytes)
                        .map(SendRawTransactionRequest::EIP4844),
                    // EIP7702
                    0x4 => {
                        EIP7702Transaction::decode(tx_bytes).map(SendRawTransactionRequest::EIP7702)
                    }
                    // FeeTokenTransaction
                    0x7d => FeeTokenTransaction::decode(tx_bytes)
                        .map(SendRawTransactionRequest::FeeToken),
                    // PrivilegedL2Transaction
                    0x7e => PrivilegedL2Transaction::decode(tx_bytes)
                        .map(SendRawTransactionRequest::PrivilegedL2),
                    ty => Err(RLPDecodeError::Custom(format!(
                        "Invalid transaction type: {ty}"
                    ))),
                }
            }
            // LegacyTransaction
            _ => LegacyTransaction::decode(bytes).map(SendRawTransactionRequest::Legacy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use ethrex_common::{Address, U256};
    use ethrex_common::types::{EIP1559Transaction, Transaction, TxKind, TxType};
    use ethrex_crypto::slh_dsa::{generate_slh_key, slh_sign};
    use ethrex_rlp::encode::PayloadRLPEncode;

    #[test]
    fn decode_canonical_accepts_slh_signature() {
        let mut tx = EIP1559Transaction {
            nonce: 1,
            max_fee_per_gas: 100,
            max_priority_fee_per_gas: 2,
            gas_limit: 21_000,
            to: TxKind::Call(Address::zero()),
            value: U256::zero(),
            data: Bytes::new(),
            access_list: Vec::new(),
            chain_id: 1,
            v: U256::from(1u64),
            sig: Bytes::new(),
            ..Default::default()
        };
        let mut payload = vec![TxType::EIP1559 as u8];
        payload.append(tx.encode_payload_to_vec().as_mut());
        let (sk, _pk) = generate_slh_key();
        let sig = slh_sign(ethrex_common::utils::keccak(&payload).as_bytes(), &sk)
            .expect("sign");
        tx.sig = Bytes::from(sig.as_bytes().to_vec());

        let raw = Transaction::EIP1559Transaction(tx.clone()).encode_canonical_to_vec();
        let decoded = SendRawTransactionRequest::decode_canonical(&raw)
            .expect("decode canonical should succeed");
        match decoded {
            SendRawTransactionRequest::EIP1559(decoded_tx) => {
                assert_eq!(decoded_tx.sig, tx.sig);
            }
            _ => panic!("unexpected tx type"),
        }
    }
}
