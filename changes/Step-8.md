# Step 8 — P2P Safety Check (Understanding + Plan)

## Understanding
- We must ensure all P2P/discovery/ENR/RLPx signing stays secp256k1.
- Step 8 is a verification task: confirm no SLH keys are used in P2P code paths and existing P2P tests still pass.
- If any accidental SLH usage exists in P2P, we must revert it to secp256k1 or gate it behind explicit non‑P2P paths.

## Plan
1) **Audit P2P key usage**
   - Review discv4, discv5, ENR, and RLPx code paths to confirm they depend on secp256k1 types.
   - Verify helper utilities still generate/parse secp keys for P2P nodes.

2) **Regression tests**
   - Run existing P2P handshake and ENR tests to confirm no regressions.
   - Ensure no new SLH dependencies are pulled into P2P crates.

## Verification
- Run P2P unit tests (or full workspace if you prefer).

## Open Questions
- Do you want a hard compile‑time guard to prevent SLH usage in P2P crates (e.g., feature flags or lint checks)?
