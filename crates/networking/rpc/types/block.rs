use super::transaction::RpcTransaction;
use ethrex_common::{
    H256, serde_utils,
    types::{Block, BlockBody, BlockHash, BlockHeader, BlockNumber, Withdrawal},
};
use ethrex_rlp::encode::RLPEncode;

use crate::utils::RpcErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlock {
    pub hash: H256,
    #[serde(with = "serde_utils::u64::hex_str")]
    pub size: u64,
    #[serde(flatten)]
    pub header: BlockHeader,
    #[serde(flatten)]
    pub body: BlockBodyWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockBodyWrapper {
    Full(FullBlockBody),
    OnlyHashes(OnlyHashesBlockBody),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBlockBody {
    pub transactions: Vec<RpcTransaction>,
    pub uncles: Vec<H256>,
    pub withdrawals: Vec<Withdrawal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyHashesBlockBody {
    // Only tx hashes
    pub transactions: Vec<H256>,
    pub uncles: Vec<H256>,
    pub withdrawals: Vec<Withdrawal>,
}

impl TryInto<Block> for RpcBlock {
    type Error = String;

    fn try_into(self) -> Result<Block, Self::Error> {
        let block_body = if let BlockBodyWrapper::Full(body) = self.body {
            body
        } else {
            return Err("Expected full block body from RPC".to_owned());
        };

        let transactions = block_body.transactions.into_iter().map(|t| t.tx).collect();

        Ok(Block {
            header: self.header,
            body: BlockBody {
                transactions,
                ommers: Vec::new(),
                withdrawals: Some(block_body.withdrawals),
            },
        })
    }
}

impl RpcBlock {
    pub fn build(
        header: BlockHeader,
        body: BlockBody,
        hash: H256,
        full_transactions: bool,
    ) -> Result<RpcBlock, RpcErr> {
        let size = Block::new(header.clone(), body.clone()).length();
        let body_wrapper = if full_transactions {
            BlockBodyWrapper::Full(FullBlockBody::from_body(body, header.number, hash)?)
        } else {
            BlockBodyWrapper::OnlyHashes(OnlyHashesBlockBody {
                transactions: body.transactions.iter().map(|t| t.hash()).collect(),
                uncles: body.ommers.iter().map(|ommer| ommer.hash()).collect(),
                withdrawals: body.withdrawals.unwrap_or_default(),
            })
        };

        Ok(RpcBlock {
            hash,
            size: size as u64,
            header,
            body: body_wrapper,
        })
    }
}

impl FullBlockBody {
    pub fn from_body(
        body: BlockBody,
        block_number: BlockNumber,
        block_hash: BlockHash,
    ) -> Result<FullBlockBody, RpcErr> {
        let mut transactions = Vec::new();
        for (index, tx) in body.transactions.iter().enumerate() {
            transactions.push(RpcTransaction::build(
                tx.clone(),
                Some(block_number),
                Some(block_hash),
                Some(index),
            )?);
        }
        Ok(FullBlockBody {
            transactions,
            uncles: body.ommers.iter().map(|ommer| ommer.hash()).collect(),
            withdrawals: body.withdrawals.unwrap_or_default(),
        })
    }
}
#[cfg(test)]
mod test {

    use bytes::Bytes;
    use ethrex_common::utils::keccak;
    use ethrex_crypto::slh_dsa::{generate_slh_key, slh_pubkey_to_address, slh_sign};
    use ethrex_common::{
        Address, Bloom, H256, U256,
        constants::EMPTY_KECCACK_HASH,
        types::{EIP1559Transaction, Transaction, TxKind, TxType},
    };
    use ethrex_rlp::encode::PayloadRLPEncode;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn serialize_block() {
        let block_header = BlockHeader {
            parent_hash: H256::from_str(
                "0x48e29e7357408113a4166e04e9f1aeff0680daa2b97ba93df6512a73ddf7a154",
            )
            .unwrap(),
            ommers_hash: H256::from_str(
                "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            )
            .unwrap(),
            coinbase: Address::from_str("0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba").unwrap(),
            state_root: H256::from_str(
                "0x9de6f95cb4ff4ef22a73705d6ba38c4b927c7bca9887ef5d24a734bb863218d9",
            )
            .unwrap(),
            transactions_root: H256::from_str(
                "0x578602b2b7e3a3291c3eefca3a08bc13c0d194f9845a39b6f3bcf843d9fed79d",
            )
            .unwrap(),
            receipts_root: H256::from_str(
                "0x035d56bac3f47246c5eed0e6642ca40dc262f9144b582f058bc23ded72aa72fa",
            )
            .unwrap(),
            logs_bloom: Bloom::from([0; 256]),
            difficulty: U256::zero(),
            number: 1,
            gas_limit: 0x016345785d8a0000,
            gas_used: 0xa8de,
            timestamp: 0x03e8,
            extra_data: Bytes::new(),
            prev_randao: H256::zero(),
            nonce: 0x0000000000000000,
            base_fee_per_gas: Some(0x07),
            withdrawals_root: Some(
                H256::from_str(
                    "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                )
                .unwrap(),
            ),
            blob_gas_used: Some(0x00),
            excess_blob_gas: Some(0x00),
            parent_beacon_block_root: Some(H256::zero()),
            requests_hash: Some(*EMPTY_KECCACK_HASH),
            ..Default::default()
        };

        let mut tx = EIP1559Transaction {
            nonce: 0,
            max_fee_per_gas: 78,
            max_priority_fee_per_gas: 17,
            to: TxKind::Call(Address::from_slice(
                &hex::decode("6177843db3138ae69679A54b95cf345ED759450d").unwrap(),
            )),
            value: 3000000000000000_u64.into(),
            data: Bytes::from_static(b"0x1568"),
            v: U256::from(3151908u64),
            sig: Bytes::new(),
            chain_id: 3151908,
            gas_limit: 63000,
            access_list: vec![(
                Address::from_slice(
                    &hex::decode("6177843db3138ae69679A54b95cf345ED759450d").unwrap(),
                ),
                vec![],
            )],
            ..Default::default()
        };
        let mut payload = vec![TxType::EIP1559 as u8];
        payload.append(tx.encode_payload_to_vec().as_mut());
        let (sk, pk) = generate_slh_key();
        let sig = slh_sign(keccak(&payload).as_bytes(), &sk).expect("sign");
        tx.sig = Bytes::from(sig.as_bytes().to_vec());
        let expected_from = slh_pubkey_to_address(&pk);

        let block_body = BlockBody {
            transactions: vec![Transaction::EIP1559Transaction(tx)],
            ommers: vec![],
            withdrawals: Some(vec![]),
        };
        let hash = block_header.hash();

        let block = RpcBlock::build(block_header, block_body, hash, true).unwrap();
        let json = serde_json::to_value(&block).unwrap();
        let tx_json = &json["transactions"][0];
        assert_eq!(tx_json["from"], format!("{:#x}", expected_from));
        assert_eq!(tx_json["v"], "0x301824");
        assert!(tx_json["sig"].as_str().unwrap().starts_with("0x"));
        assert!(tx_json.get("r").is_none());
        assert!(tx_json.get("s").is_none());
        assert!(tx_json.get("yParity").is_none());
    }
}
