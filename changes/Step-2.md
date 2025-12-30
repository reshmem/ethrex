# Step 2 - Crypto Layer Integration (Plan)

## Understanding
- Add SLH-DSA helpers to Ethrex’s crypto layer so higher layers can convert an SLH-DSA public key into an address using Quranium’s rule: `keccak256(pubkey[1..])[12..]`.
- Keep EC helpers for P2P (secp256k1) intact and separate: `ec_sign`, `ec_verify`, `ec_compress`, `ec_decompress`, `ec_signature_len`, since P2P/ENR stays ECDSA.
- Add tests:
  - `address_derivation_matches_quranium_style`
  - `slh_recover_validates_signature`

## Implementation approach
- Place SLH-DSA address derivation and helper functions in `crates/common/crypto/` alongside the SLH-DSA wrapper.
- Ensure EC helpers remain explicit and unmodified for P2P/discovery usage.
- Add focused unit tests in the same module.

