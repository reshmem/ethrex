# Step 0 - Ethrex Mapping (SLH-DSA Port)

This file maps the DEV-TODO.md subsystems to Ethrex code locations. It is a discovery-only map to guide later steps.

| Area | Ethrex modules / paths | Notes |
| --- | --- | --- |
| Crypto primitives | `crates/common/crypto/` (`blake2f`, `keccak`, `kzg`); `crates/common/utils.rs` | Core hashing and crypto helpers (keccak used for address derivation). |
| Transaction types + encoding | `crates/common/types/transaction.rs`; `crates/common/rlp/` | Tx structs, RLP encode/decode, JSON serde impls, chain-id handling, and signature fields. |
| Signature recovery (tx) | `crates/common/types/transaction.rs` (`recover_address`, `recover_address_from_message`) | ECDSA recovery logic and EIP-2 low-s checks live here. |
| Signature recovery (EVM precompile 0x01) | `crates/vm/levm/src/precompiles.rs` (`ECRECOVER` + `ecrecover`) | Precompile implementation and gas cost for ECRECOVER. |
| Signature recovery (EIP-7702 auth / vm utils) | `crates/vm/levm/src/utils.rs` | Uses secp256k1/k256 recovery for auth tuples. |
| EVM precompiles registry | `crates/vm/levm/src/precompiles.rs`; `crates/vm/lib.rs` | Precompile list and dispatch table. |
| Keystore / wallet / account | No dedicated keystore module found in L1 crates; L2 uses external wallet flows | L2 proof sender uses an external `Wallet` (`crates/l2/sequencer/l1_proof_sender.rs`). Local dev keys in `fixtures/keys/*`. Account types in `crates/common/types/account.rs`. |
| RPC JSON (tx + chain id) | `crates/networking/rpc/types/transaction.rs`; `crates/networking/rpc/eth/transaction.rs`; `crates/networking/rpc/eth/client.rs` | JSON structs and RPC handlers for transaction fields and chain id. |
| Chain params / config | `crates/common/types/genesis.rs` (ChainConfig); `crates/common/config/networks.rs`; `cmd/ethrex/networks/*/genesis.json` | Network defaults, chain ids, and genesis config. |
| P2P / ENR / discovery | `crates/networking/p2p/` (discv4, discv5, rlpx, types) | ENR parsing/signing, discv4/5 packet signing, RLPx handshake signatures. |

