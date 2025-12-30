use anyhow::Error;
use bytes::Bytes;
use ethrex_common::types::Block;
use ethrex_rlp::decode::RLPDecode as _;
use std::{
    fs::File,
    io::{BufReader, Read as _},
};
pub fn jwtsecret_file(file: &mut File) -> Bytes {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read jwt secret file");
    contents = contents
        .strip_prefix("0x")
        .unwrap_or(&contents)
        .trim_end_matches('\n')
        .to_string();
    hex::decode(contents)
        .expect("Secret should be hex encoded")
        .into()
}
pub fn chain_file(file: File) -> Result<Vec<Block>, Error> {
    let mut chain_rlp_reader = BufReader::new(file);
    let mut buf = vec![];
    chain_rlp_reader.read_to_end(&mut buf)?;
    let mut buf = buf.as_slice();
    let mut blocks = Vec::new();
    while !buf.is_empty() {
        let (item, rest) = Block::decode_unfinished(buf)?;
        blocks.push(item);
        buf = rest;
    }
    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use crate::decode::chain_file;
    use ethrex_common::{
        constants::DEFAULT_OMMERS_HASH,
        types::{
            compute_receipts_root, compute_transactions_root, compute_withdrawals_root, Block,
            BlockBody, BlockHeader,
        },
        Address, Bloom, H256, U256,
    };
    use ethrex_rlp::encode::RLPEncode as _;
    use std::{
        fs::File,
        io::Write as _,
        path::PathBuf,
    };

    fn build_test_chain(count: usize) -> Vec<Block> {
        let mut blocks = Vec::with_capacity(count);
        let mut parent_hash = H256::zero();

        for number in 1..=count {
            let body = BlockBody {
                transactions: Vec::new(),
                ommers: Vec::new(),
                withdrawals: Some(Vec::new()),
            };
            let transactions_root = compute_transactions_root(&body.transactions);
            let receipts_root = compute_receipts_root(&[]);
            let withdrawals_root = Some(compute_withdrawals_root(
                body.withdrawals.as_ref().expect("withdrawals set"),
            ));

            let header = BlockHeader {
                hash: Default::default(),
                parent_hash,
                ommers_hash: *DEFAULT_OMMERS_HASH,
                coinbase: Address::zero(),
                state_root: H256::zero(),
                transactions_root,
                receipts_root,
                logs_bloom: Bloom::default(),
                difficulty: U256::zero(),
                number: number as u64,
                gas_limit: 0,
                gas_used: 0,
                timestamp: number as u64,
                extra_data: Default::default(),
                prev_randao: H256::zero(),
                nonce: 0,
                base_fee_per_gas: Some(0),
                withdrawals_root,
                blob_gas_used: None,
                excess_blob_gas: None,
                parent_beacon_block_root: None,
                requests_hash: None,
            };

            let block = Block::new(header, body);
            parent_hash = block.hash();
            blocks.push(block);
        }

        blocks
    }

    #[test]
    fn decode_chain_file() {
        let expected_blocks = build_test_chain(20);
        let mut path = PathBuf::from(std::env::temp_dir());
        path.push("ethrex-test-chain.rlp");

        {
            let mut file = File::create(&path).expect("Failed to create chain file");
            for block in &expected_blocks {
                file.write_all(&block.encode_to_vec())
                    .expect("Failed to write chain file");
            }
        }

        let file = File::open(&path).expect("Failed to open chain file");
        let blocks = chain_file(file).expect("Failed to decode chain file");
        let _ = std::fs::remove_file(&path);

        assert_eq!(20, blocks.len(), "There should be 20 blocks in chain file");
        assert_eq!(
            1,
            blocks.first().unwrap().header.number,
            "first block should be number 1"
        );
        assert_eq!(
            expected_blocks.first().unwrap().hash(),
            blocks.first().unwrap().hash(),
            "First block hash does not match"
        );
        assert_eq!(
            expected_blocks.get(1).unwrap().hash(),
            blocks.get(1).unwrap().hash(),
            "Second block hash does not match"
        );
        assert_eq!(
            expected_blocks.last().unwrap().hash(),
            blocks.last().unwrap().hash(),
            "Last block hash does not match"
        );
    }

    #[test]
    #[ignore = "one-off fixture regeneration"]
    fn update_chain_fixture() {
        let blocks = build_test_chain(20);
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../fixtures/blockchain/chain.rlp");

        let mut file = File::create(&path).expect("Failed to create chain file");
        for block in &blocks {
            file.write_all(&block.encode_to_vec())
                .expect("Failed to write chain file");
        }
    }
}
