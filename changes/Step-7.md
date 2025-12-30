# Step 7 — RPC Surface (Understanding + Plan)

## Understanding
- RPC transaction JSON must expose the new SLH signature format:
  - `sig` (bytes) and `v` (chain id as hex)
- Legacy ECDSA fields (`r`, `s`, `yParity`) should be removed or set to null to avoid confusion.
- The main RPC paths to update are:
  - Transaction serialization for `eth_getTransactionByHash` and friends
  - Raw tx ingestion (`eth_sendRawTransaction`) to accept SLH signatures
- Ensure existing parsing/encoding keeps compatibility for legacy tx types while switching JSON fields to `sig` + `v`.

## Plan
1) **RPC JSON output updates**
   - Update transaction JSON types to include `sig` and `v`.
   - Remove `r`, `s`, `yParity` from RPC outputs or return them as `null` if required by the schema.
   - Confirm all transaction response endpoints use the updated struct.

2) **Raw transaction ingestion**
   - Ensure `eth_sendRawTransaction` accepts the SLH signature format (already encoded in RLP).
   - Update any signature validation or decoding assumptions in the RPC layer.

3) **Tests**
   - Add/adjust RPC tests to assert `sig` + `v` are present and `r/s/yParity` are absent/null.
   - Add a test for `eth_sendRawTransaction` with an SLH‑signed tx payload.

## Verification
- Run RPC crate tests for the updated JSON serialization and raw-tx acceptance.

## Notes / Open Questions
- Confirm whether you want `r/s/yParity` omitted entirely from JSON or present as `null`.
