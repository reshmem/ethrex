# Step 6 — Keystore / Wallet / Account (Understanding + Plan)

## Understanding
- The goal is to switch account key handling from secp256k1 to SLH‑DSA for wallet/account flows, while preserving secp256k1 compatibility for legacy keys.
- SLH‑DSA private keys are 128 bytes; public keys are 64 bytes; address derivation is `keccak256(pubkey[1..])[12..]`.
- P2P/ENR/discovery keys must remain secp256k1 (covered in Step 8), so Step 6 should not touch those paths.
- In this repo, the “wallet/account” surfaces primarily live in L2 tooling and signer utilities:
  - `crates/l2/networking/rpc/signer.rs` (Local/Remote signer + signing)
  - `crates/l2/common/src/utils.rs` (`get_address_from_secret_key`)
  - CLI/private‑key parsing in `cmd/ethrex/utils.rs` and L2 CLI/options (private key parsing helpers)
  - L2 SDK/monitor/tests that use `SecretKey` and address derivation helpers
- There is no explicit keystore implementation today; keys are passed as raw hex or stored in plain files. Step 6 likely needs a minimal keystore for SLH key bytes plus a compatibility path to accept legacy secp keys.

## Plan
1) **Introduce SLH key handling for wallet/account flows**
   - Add a new key type (e.g., `AccountPrivateKey`) that can hold either:
     - SLH private key bytes (128 bytes)
     - Legacy secp256k1 private key bytes (32 bytes)
   - Implement address derivation based on key kind:
     - SLH: derive pubkey with `slh_dsa` and use `slh_pubkey_to_address`
     - Legacy secp: keep current `keccak(pubkey[1..])[12..]`

2) **Local signer update**
   - Update `crates/l2/networking/rpc/signer.rs`:
     - `LocalSigner` should accept the new `AccountPrivateKey`.
     - If SLH: sign using `slh_sign` and set `sig` accordingly.
     - If legacy secp: keep current ECDSA path.
   - `RemoteSigner` stays secp256k1 (Web3Signer API). That’s the compatibility path for legacy EC keys.

3) **Key parsing and storage**
   - Add parsing helpers that accept either 32‑byte (legacy) or 128‑byte (SLH) hex strings.
   - Implement a minimal keystore file format for SLH keys in the datadir (e.g., `account_key.json`), storing encrypted SLH key bytes.
   - If the keystore is missing but a legacy secp key is provided, keep it as legacy.

4) **Address derivation helper**
   - Replace `get_address_from_secret_key` usages to call a new `get_address_from_account_key` that dispatches to SLH or secp based on key kind.
   - Update L2 utilities/tests to use the new helper (avoiding accidental secp‑only derivation).

## Tests / Verification
- Unit: `encrypt_decrypt_roundtrip_slh` (keystore) — store + load SLH key bytes.
- Unit: `sign_message_and_verify` (wallet/signer) — sign with SLH key and verify via `slh_verify`.
- Unit: address derivation uses SLH pk path.
- Run targeted tests for affected crates (likely `ethrex-l2-rpc`, `ethrex-l2-common`, and any new keystore module tests).

## Open Questions
- Where should the keystore live and what format do you want (simple encrypted JSON, or a standard Ethereum keystore format)?
- Should we also update L2 CLI/options to accept SLH private keys via flags/env, or keep CLI inputs secp‑only for now?
