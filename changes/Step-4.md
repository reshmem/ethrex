# Step 4 - Precompile 0x01 (slhrecover) (Plan)

## Understanding
- Replace the existing `ecrecover` precompile at address `0x01` with `slhrecover`.
- Input format is fixed ABI-style:
  - `hash(32) | offset(32) | siglen(32) | sig(49920)`
  - `siglen` must equal `49920` (`SIG_WITH_PUBKEY_LEN`).
- Output is the 32-byte padded address: `keccak256(pubkey)[12..]`.
- Gas constant should be `SlhRecoverGas = 3000`.
- Invalid length or invalid signature returns the precompile's failure behavior (likely empty/zeroed output).

## Approach
- Update `crates/vm/levm/src/precompiles.rs`:
  - Replace `ecrecover` handler logic with SLH-DSA verification/recovery.
  - Parse input and validate lengths.
  - Use `slh_recover(hash, sig)` then `slh_pubkey_to_address`.
  - Pad address to 32 bytes for output.
  - Set gas cost to 3000.
- Keep precompile semantics consistent with current error handling.

## Testing
- Unit tests in precompiles:
  - Valid input returns expected address.
  - Invalid length / wrong siglen returns failure (empty/zeroed).
  - Gas cost is 3000.
