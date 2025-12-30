# Step 3 - Transaction Signatures & Encoding (Plan)

## Understanding
- Replace ECDSA `(r,s,v)` handling in transaction structs with a single `sig: Vec<u8>` and `v` as chain ID (no EIP-155 parity encoding).
- Update signing so `sign_tx(hash)` uses SLH-DSA, stores `sig`, and sets `v = chain_id`.
- Update sender recovery to `slh_recover(hash, sig)` and derive address from recovered pubkey.
- Update JSON and RLP encoding/decoding:
  - include `sig` in tx JSON and RLP
  - treat `v` as chain ID
  - remove/ignore `r/s/yParity`
- Add tests:
  - `tx_sign_and_recover_sender`
  - `tx_json_roundtrip_with_sig`
  - `tx_chain_id_v_field`
  - regression for legacy decoding with new fields

