# Step 1 - SLH-DSA Crate + API Contract (Plan)

## Understanding
We need to add a Rust SLH-DSA dependency (`slh_dsa` from docs.rs) and expose a small, stable wrapper API inside Ethrex. The wrapper defines fixed sizes and types, and standardizes signature format as `sig || pubkey` (total 49,920 bytes).

## What I will implement
- Add the `slh_dsa` dependency to the appropriate crate (likely `crates/common/crypto` or a new crypto submodule) and wire a new module for SLH-DSA.
- Define constants:
  - `PUBKEY_LEN = 64`
  - `SIG_LEN = 49856`
  - `SIG_WITH_PUBKEY_LEN = 49920`
- Define types: `SlhPrivateKey`, `SlhPublicKey`, `SlhSignature` (newtypes around bytes with length checks).
- Implement wrapper API:
  - `generate_slh_key() -> (SlhPrivateKey, SlhPublicKey)`
  - `slh_sign(hash, sk) -> sig_with_pk` (sign + append pubkey)
  - `slh_verify(hash, sig_with_pk) -> bool`
  - `slh_recover(hash, sig_with_pk) -> pk` (verify then return pk tail)

## How we will test / check
Unit tests for the wrapper module:
1) `keygen_sign_verify_roundtrip`: generate, sign, verify.
2) `signature_sizes_match`: assert exact byte lengths.
3) `recover_pubkey_from_signature`: sign and recover pk, then compare to generated pk.

Suggested command:
- `cargo test -p ethrex-crypto` (or the crate where we add the module)

