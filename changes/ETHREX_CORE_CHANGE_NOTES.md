# Core Execution Changes (Quranium vs geth v1.15.0)

Scope: core execution/signing/crypto/p2p/params/keystore only. Tests, CI, and tooling excluded.
Baseline: `refs/tags/v1.15.0`.
Target: current `HEAD`.

Each entry includes what changed, why (inferred from code), and how it was achieved so you can port to Ethrex.

---

## Crypto

### `crypto/crypto.go`
- What changed:
  - Added SLHDSA integration (`crypto/slhdsa`) and renamed ECDSA constants.
  - Introduced `ECSignatureLength` and set `SignatureLength` to `49920` (SLHDSA signature size).
  - Added SLHDSA key helpers: `ToSLHDSA`, `ToSLHDSAUnsafe`, `FromSLHDSA`, `HexToSLHDSA`, `LoadSLHDSA`, `SaveSLHDSA`.
  - Split ECDSA vs SLHDSA public key handling: `UnmarshalECPubkey` vs `UnmarshalPubkey` (SLHDSA), `FromSLHDSAPub`.
  - Replaced `GenerateKey` to generate SLHDSA keys; added `GenerateECKey` for secp256k1.
  - `ValidateSignatureValues` now takes `sig []byte` and returns `true` (ECDSA validation commented out).
  - Address derivation now uses `PubkeyToAddress(slhdsa.PublicKey)`; added `ECPubkeyToAddress`.
  - Added `SlhdsaKeyExchange` (XOR placeholder per comments).
- Why changed (inferred):
  - Switch primary signing scheme from ECDSA/secp256k1 to SLHDSA, while keeping ECDSA support for P2P and tooling.
  - Store full SLHDSA signatures (which include public key) and derive addresses from SLHDSA public keys.
- How achieved:
  - Introduced SLHDSA key size assumptions via `slhdsa.SLHSKBytes` and `slhdsa.SLHPublicKeySize`.
  - Modified key parsing/serialization to treat SLHDSA keys as raw byte blobs rather than big-int D values.
  - Bypassed ECDSA signature validation logic; `ValidateSignatureValues` now accepts any signature.
  - Address derivation now runs keccak on `pubBytes[1:]` from SLHDSA public key bytes.
  - Added EC-specific helpers to preserve secp256k1 usage where needed.

### `crypto/signature_cgo.go`
- What changed:
  - Added `Slhrecover` to recover SLHDSA pubkey from signature.
  - `SigToPub` now returns `*slhdsa.PublicKey` and calls `Slhrecover`.
  - Renamed ECDSA helpers: `ECSign`, `VerifyECSignature`, `CompressECPubkey`, `DecompressECPubkey`.
  - `Sign` and `VerifySignature` now use SLHDSA by default.
  - `DecompressPubkey`/`CompressPubkey` operate on SLHDSA public keys (no actual compression, just passthrough bytes).
- Why changed (inferred):
  - Make SLHDSA the default signing and verification path, while retaining explicit EC helpers for P2P/discovery.
- How achieved:
  - Wrapped SLHDSA C library via `crypto/slhdsa` and swapped the generic `Sign`/`VerifySignature` to use it.
  - Added explicit EC-only helpers to avoid breaking P2P/discovery code paths that still use secp256k1.

### `crypto/slhdsa/slhdsa.go` (new)
- What changed:
  - Added full SLHDSA cgo wrapper over `libslhdsa` with keygen, sign, verify, and pubkey derivation.
  - Defines `PrivateKey`, `PublicKey`, key sizes, and signature size.
  - Signature format = `sig || pubkey` (public key appended to signature).
  - Includes a custom key agreement and encrypt/decrypt helpers (AES-CTR + HMAC) and KDF utilities.
- Why changed (inferred):
  - Provide SLHDSA primitives and expose them as Go APIs used throughout core signing and precompile logic.
- How achieved:
  - Added cgo bindings with `slhdsa_gen_privatekey`, `slhdsa_gen_pubkey`, `slhdsa_sign_message`, `slhdsa_signature_verify`.
  - Implemented `Sign` to append public key to the signature and `RecoverPubkey` to extract/verify.
  - Implemented ECIES-like helpers and KDFs inside the Go wrapper (used by scwallet/securechannel).

### `crypto/slhdsa/libslhdsa/*` (new)
- What changed:
  - Added complete C library sources, headers, and autotools scripts for SLHDSA and Keccak.
- Why changed (inferred):
  - Provide native SLHDSA implementation backing the Go cgo wrapper.
- How achieved:
  - Vendor/imported C library into `crypto/slhdsa/libslhdsa` with build artifacts and headers.

---

## Core Types / Signing

### `core/types/transaction.go`
- What changed:
  - `TxData` interface now exposes `v()` and uses signature blobs: `rawSignatureValues() (sig []byte)` and `setSignatureValues(chainID, V, sig)`.
  - `sanityCheckSignature` now accepts `sig []byte` and calls `crypto.ValidateSignatureValues(sig, ...)`.
  - `Protected()` now always returns `true`.
  - `RawSignatureValues` returns signature blob only.
  - `WithSignature` now expects signer to return `(sig, V)` and stores signature blob plus chain ID.
  - Added `Transaction.V()` to expose the stored V value (used as chain ID in this fork).
- Why changed (inferred):
  - SLHDSA signatures are not decomposed into `(r,s,v)`; instead a single signature blob is stored.
  - Chain ID is carried separately as `V` (no EIP-155 encoding).
- How achieved:
  - Replaced r/s/v fields in all tx variants and signers with a single `Sig []byte` field.
  - Simplified signature validation to check only signature blob presence.

### `core/types/transaction_marshalling.go`
- What changed:
  - JSON encoding now includes `sig` instead of `r`/`s`.
  - `yParity` handling is effectively disabled; `V` is used directly.
  - Unmarshal for all tx types uses `dec.V` as required and validates `dec.Sig` if present.
- Why changed (inferred):
  - Replace ECDSA `(r,s,v)` JSON representation with SLHDSA signature blob.
  - Store chain ID in `v` directly, rather than in EIP-155 parity encoding.
- How achieved:
  - Commented out `r`/`s` fields and `yParity` cross-checks, added `Sig []byte`.
  - Validation moved to `sanityCheckSignature(sig, ...)`.

### `core/types/transaction_signing.go`
- What changed:
  - Signing now uses SLHDSA: `SignTx` takes `*slhdsa.PrivateKey` and calls `slhdsa.Sign`.
  - `Signer.SignatureValues` returns `(sig []byte, V big.Int)` instead of `(r,s,v)`.
  - `decodeSignature` now returns the full signature blob; no R/S/V parsing.
  - `recoverPlain` now uses `crypto.Slhrecover` and derives address from SLHDSA pubkey.
  - All signers set `V = chainId` directly when chainId is non-zero.
- Why changed (inferred):
  - SLHDSA signature format is a blob; sender recovery uses embedded public key from signature.
  - Chain ID is stored directly instead of EIP-155 encoding.
- How achieved:
  - Replaced ECDSA `crypto.Sign` with `slhdsa.Sign` and removed `(r,s,v)` handling.
  - Updated sender recovery to call `Slhrecover` and `crypto.PubkeyToAddress(slhdsa.PublicKey{PK: pubKey})`.
  - Removed EIP-155 V decoding logic; V is treated as chain ID only.

### `core/types/tx_legacy.go`
- What changed:
  - Replaced `(V,R,S)` with `Sig []byte` and `V *big.Int`.
  - `chainID()` now returns `big.NewInt(tx.V.Int64())` (direct chain ID).
  - `rawSignatureValues` and `setSignatureValues` operate on signature blob.
- Why changed (inferred):
  - Store SLHDSA signature blob and separate chain ID.
- How achieved:
  - Removed `R`/`S` fields and related copying.
  - Stored signature blob and chain ID in `V`.

### `core/types/tx_access_list.go`
- What changed:
  - Replaced `(V,R,S)` with `Sig []byte` and `V *big.Int`.
  - Added `v()` accessor returning chain ID.
- Why changed (inferred):
  - SLHDSA signature blob and chain ID separation.
- How achieved:
  - Updated struct fields, copy logic, and signature accessors.

### `core/types/tx_dynamic_fee.go`
- What changed:
  - Replaced `(V,R,S)` with `Sig []byte` and `V *big.Int`.
  - Added `v()` accessor returning chain ID.
- Why changed (inferred):
  - SLHDSA signature blob and chain ID separation.
- How achieved:
  - Updated struct fields, copy logic, and signature accessors.

### `core/types/tx_blob.go`
- What changed:
  - Replaced `(V,R,S)` with `Sig []byte` and `V *uint256.Int`.
  - Added `v()` accessor returning chain ID.
- Why changed (inferred):
  - SLHDSA signature blob and chain ID separation for blob txs.
- How achieved:
  - Updated struct fields, copy logic, and signature accessors.

### `core/types/tx_setcode.go` and `core/types/gen_authorization.go`
- What changed:
  - SetCode tx and authorization objects now carry `Sig []byte` instead of `R/S/V`.
  - Authorization includes `V` field as chain ID.
  - `SignSetCode` expects SLHDSA private key and stores `Sig`.
  - `Authority()` validates signature via `crypto.Slhrecover`.
- Why changed (inferred):
  - Align SetCode authorization with SLHDSA signatures and chain ID storage.
- How achieved:
  - Removed R/S/V serialization in JSON and replaced with `sig`.
  - Recovering signer uses `Slhrecover` on the signature blob.

---

## EVM Precompiles

### `core/vm/contracts.go`
- What changed:
  - Replaced precompile address `0x01` from `ecrecover` to `slhrecover` across all fork sets.
  - `slhrecover` input format changed: expects `hash (32) || offset (32) || siglen (32) || signature (49920)` (total 96+SignatureLength).
  - Uses `crypto.Slhrecover` and keccak over the recovered pubkey bytes.
  - Gas constant changed to `params.SlhrecoverGas`.
- Why changed (inferred):
  - Replace ECDSA recovery precompile with SLHDSA recovery and a larger signature payload.
- How achieved:
  - Implemented new `slhrecover` precompile and updated precompile maps.
  - Added input length/sig length validation and pubkey recovery via SLHDSA.

---

## P2P / Discovery / RLPx (EC-only)

These changes keep P2P/discovery on secp256k1 while the rest of the system uses SLHDSA.

### `p2p/discover/v4wire/v4wire.go`
- What changed:
  - `sigSize` now uses `crypto.ECSignatureLength`.
  - Packet signing uses `crypto.ECSign` (ECDSA).
- Why changed (inferred):
  - Keep discv4 signatures in secp256k1 format despite global SLHDSA defaults.
- How achieved:
  - Swapped `crypto.Sign` for `crypto.ECSign` and signature size constant.

### `p2p/discover/v5wire/crypto.go`
- What changed:
  - Uses `CompressECPubkey`/`DecompressECPubkey`.
  - ID nonce signatures use `ECSign` and verify with `VerifyECSignature`.
- Why changed (inferred):
  - Preserve secp256k1-based discv5 identity scheme.
- How achieved:
  - Replaced generic crypto helpers with EC-specific ones.

### `p2p/discover/v5wire/session.go`
- What changed:
  - Ephemeral key generation uses `crypto.GenerateECKey`.
- Why changed (inferred):
  - Ensure ephemeral keys are secp256k1, not SLHDSA.
- How achieved:
  - Swapped `GenerateKey` with `GenerateECKey`.

### `p2p/enode/idscheme.go`
- What changed:
  - ENR signing uses `ECSign` and verification uses `VerifyECSignature`.
  - Public key RLP encoding uses `CompressECPubkey`/`DecompressECPubkey`.
- Why changed (inferred):
  - Keep ENR identity scheme on secp256k1.
- How achieved:
  - Replaced default crypto helpers with EC-specific ones.

### `p2p/dnsdisc/tree.go`
- What changed:
  - Tree signatures use `ECSign` and verify with `VerifyECSignature`.
  - Signature length checks use `ECSignatureLength`.
  - Links encode/decode EC compressed pubkeys.
- Why changed (inferred):
  - DNS discovery signatures remain secp256k1.
- How achieved:
  - Swapped generic crypto helpers with EC-specific equivalents.

### `p2p/rlpx/rlpx.go`
- What changed:
  - Handshake signature length uses `ECSignatureLength`.
  - Auth message signatures use `ECSign`.
  - Public key import uses `UnmarshalECPubkey`.
- Why changed (inferred):
  - RLPx handshake remains secp256k1-based.
- How achieved:
  - Swapped generic crypto helpers with EC-specific equivalents.

---

## Keystore / Accounts / Wallet

### `accounts/keystore/key.go`
- What changed:
  - `Key` now stores `*slhdsa.PrivateKey` (SLHDSA) instead of ECDSA.
  - Added `ECKey` struct for ECDSA-specific usage.
  - JSON marshal/unmarshal uses SLHDSA key hex serialization.
  - Key creation uses SLHDSA `GenerateKey` and `PubkeyToAddress`.
- Why changed (inferred):
  - Store and manage SLHDSA keys as primary account keys while retaining EC compatibility.
- How achieved:
  - Replaced ECDSA generation and address derivation with SLHDSA equivalents.

### `accounts/keystore/keystore.go`
- What changed:
  - `ImportECDSA` removed/commented; `ImportSLHDSA` added.
  - `zeroKey` now zeroes SLHDSA secret bytes.
- Why changed (inferred):
  - SLHDSA is the default account key type.
- How achieved:
  - Replaced ECDSA key handling with SLHDSA equivalents.

### `accounts/keystore/passphrase.go`
- What changed:
  - `EncryptKey` now encrypts SLHDSA private key bytes (SK).
  - Added `EncryptECKey` and `DecryptECKey` for ECDSA compatibility.
  - `DecryptKey` now produces SLHDSA keys via `crypto.ToSLHDSA`.
- Why changed (inferred):
  - Persist SLHDSA keys in keystore JSON while keeping an EC-only path.
- How achieved:
  - Swapped key serialization to SLHDSA SK bytes and added EC-specific helpers.

### `accounts/keystore/presale.go`
- What changed:
  - Presale key derivation now builds SLHDSA key from `Keccak256` output.
- Why changed (inferred):
  - Make presale import generate SLHDSA accounts instead of ECDSA.
- How achieved:
  - Replaced `ToECDSAUnsafe` with `ToSLHDSAUnsafe`.

### `accounts/scwallet/securechannel.go`
- What changed:
  - Secure channel key exchange now uses `SlhdsaKeyExchange` with SLHDSA keys.
  - Public key bytes now `FromSLHDSAPub`.
- Why changed (inferred):
  - Align smartcard channel establishment with SLHDSA key format.
- How achieved:
  - Replaced ECDSA scalar multiply with `SlhdsaKeyExchange`.

### `accounts/scwallet/wallet.go`
- What changed:
  - `initialize` uses `ToSLHDSA` and `FromSLHDSAPub`.
  - Recoverable signature uses `Slhrecover`.
- Why changed (inferred):
  - Wallet derivation and signature recovery aligned to SLHDSA.
- How achieved:
  - Replaced ECDSA helpers with SLHDSA equivalents.

---

## Chain Params

### `params/config.go`
- What changed:
  - `ChainID` for multiple configs set to `4062024`.
  - Added `MaxTxnPerBlock` to `ChainConfig` and helper `GetMaxTxnPerBlock()`.
- Why changed (inferred):
  - Set Quranium chain ID and introduce per-block transaction cap.
- How achieved:
  - Updated chain config defaults and added getter returning `MaxTransactionsPerBlock` constant.

### `params/protocol_params.go`
- What changed:
  - Renamed `EcrecoverGas` to `SlhrecoverGas` (same value).
  - Added `MaxTransactionsPerBlock` constant.
- Why changed (inferred):
  - Align protocol constants with SLHDSA precompile and transaction cap.
- How achieved:
  - Replaced constant name and added new max-tx constant.

### `params/bootnodes.go`
- What changed:
  - Default bootnodes commented out for Mainnet/Holesky/Sepolia/V5.
- Why changed (inferred):
  - Disable upstream bootnodes for a custom network.
- How achieved:
  - Commented out lists rather than deleting them.

---

## Ethrex Porting Notes (Core Execution)
- Replace all ECDSA signature handling in tx signing/encoding with SLHDSA signature blobs.
- Update transaction JSON and wire formats to carry `sig` and treat `v` as chain ID (not EIP-155 parity).
- Implement `slhrecover` precompile at address `0x01` with the new input layout and signature length checks.
- Maintain secp256k1 for P2P/discovery/ENR/RLPx by adding EC-specific crypto helpers.
- Update keystore to store SLHDSA keys and add EC compatibility path if needed.
- Update chain config to new chain ID and add per-block transaction cap.

