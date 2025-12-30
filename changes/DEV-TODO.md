# DEV TODO — Ethrex SLH-DSA (SHAKE-256f) Port Plan

Assumptions:
- Use a standard Rust SLH-DSA library with SHAKE-256f parameter set (e.g., `pqcrypto-sphincsplus::sphincsshake256f`).
- v1 supports only SHAKE-256f.
- P2P/ENR/discovery remain secp256k1.

---

## Step 0 — Repo Mapping (Ethrex)
- Locate Ethrex modules for:
  - crypto primitives
  - transaction types/encoding
  - signature recovery
  - precompiles / EVM
  - keystore / wallet / account
  - RPC JSON
  - chain params / config
  - P2P/ENR/discovery
- Deliverable: mapping table from Quranium changes to Ethrex file paths.
- Tests: none (discovery step).

## Step 1 — SLH-DSA Crate + API Contract
- Add dependency for SHAKE-256f SLH-DSA.
- Define constants:
  - `PUBKEY_LEN = 64`
  - `SIG_LEN = 49856`
  - `SIG_WITH_PUBKEY_LEN = 49920` (sig || pubkey)
- Define types: `SlhPrivateKey`, `SlhPublicKey`, `SlhSignature`.
- Implement wrapper API:
  - `generate_slh_key()`
  - `slh_sign(hash, sk) -> sig_with_pk`
  - `slh_verify(hash, sig_with_pk) -> bool`
  - `slh_recover(hash, sig_with_pk) -> pk` (verify + return tail pk)
- Tests:
  - Unit: `keygen_sign_verify_roundtrip`
  - Unit: `signature_sizes_match`
  - Unit: `recover_pubkey_from_signature`

## Step 2 — Crypto Layer Integration
- Add SLH-DSA helpers in Ethrex crypto module:
  - `slh_pubkey_to_address(pk)` = `keccak256(pk[1..])[12..]` (Quranium style)
- Keep EC helpers for P2P:
  - `ec_sign`, `ec_verify`, `ec_compress`, `ec_decompress`, `ec_signature_len`
- Tests:
  - Unit: `address_derivation_matches_quranium_style`
  - Unit: `slh_recover_validates_signature`

## Step 3 — Transaction Signatures & Encoding
- Replace `(r,s,v)` with:
  - `sig: Vec<u8>`
  - `v: U256/BigInt` = chain ID (no EIP-155 parity encoding)
- Update signing:
  - `sign_tx(hash)` uses SLH-DSA and stores `sig`, `v = chain_id`.
- Update sender recovery:
  - `sender(tx)` uses `slh_recover(hash, sig)` and derives address.
- Update JSON/RLP:
  - include `sig` in tx JSON
  - treat `v` as chain ID
  - remove/ignore `r/s/yParity`
- Tests:
  - Unit: `tx_sign_and_recover_sender`
  - Unit: `tx_json_roundtrip_with_sig`
  - Unit: `tx_chain_id_v_field`
  - Regression: legacy tx type still decodes with new fields

## Step 4 — Precompile 0x01 (slhrecover)
- Replace `ecrecover` with `slhrecover`.
- Input format:
  - total length = `96 + SIG_WITH_PUBKEY_LEN`
  - `hash(32) | offset(32) | siglen(32) | sig(49920)`
  - validate `siglen == 49920`
- Output:
  - `keccak256(pubkey)[12..]` (32 bytes padded)
- Gas constant:
  - `SlhRecoverGas = 3000`
- Tests:
  - Precompile: valid input returns expected address
  - Precompile: invalid length errors
  - Gas: constant is 3000

## Step 5 — Chain Params + Tx/Block Cap
- Set default chain ID to `4062024` in config presets.
- Add `max_txs_per_block` param default `300`.
- Enforce in block builder/tx selection.
- Tests:
  - Unit: `config_max_txs_default`
  - Integration: block rejects >300 txs
  - Integration: block accepts <=300 txs

## Step 6 — Keystore / Wallet / Account
- Store SLH-DSA secret key bytes in keystore.
- Add EC compatibility path if legacy EC keys exist.
- Update address derivation to SLH-DSA.
- Tests:
  - Keystore: `encrypt_decrypt_roundtrip_slh`
  - Wallet: `sign_message_and_verify`
  - Address derivation uses SLH-DSA pk path

## Step 7 — RPC Surface
- Update RPC tx JSON to include `sig` and `v`.
- Remove or null `r/s/yParity` fields.
- Tests:
  - RPC: `eth_getTransactionByHash` returns `sig` + `v`
  - RPC: `eth_sendRawTransaction` accepts SLH-DSA signature

## Step 8 — P2P Safety Check
- Ensure discv4/discv5/ENR/RLPx remain secp256k1-based.
- Tests:
  - Existing P2P handshake tests pass
  - ENR signature validation unchanged
