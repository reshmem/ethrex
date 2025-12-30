# Step 5 — Understanding & Plan

## Understanding
Step 5 has two changes to chain parameters and block production limits:

1) **Default chain ID** should become **4062024** for Ethrex’s default chain configuration. This likely affects the local/dev/test network defaults (e.g., `fixtures/genesis/*`, `Network::LocalDevnet`, or any “default” Genesis/ChainConfig used when no explicit chain is provided). It should not override public network presets (mainnet/holesky/sepolia/hoodi), unless explicitly intended.

2) **`max_txs_per_block` default = 300** should be added to the chain config and enforced in block building / transaction selection. This means the block builder or mempool selection path must cap included transactions at 300, and block validation should reject blocks exceeding this limit. This is a consensus‑critical rule and should be applied consistently wherever blocks are constructed or validated.

## Aim / Approach
- **Locate chain config / genesis defaults** and add the `max_txs_per_block` field with default 300. Ensure it is serialized/deserialized in genesis JSON and config structs.
- **Set default chain ID** to 4062024 in the default genesis/config path used for “local devnet” or “default” chain creation (not public networks).
- **Enforce limit in block builder / tx selection** and **validate** the limit in block verification or block acceptance paths.

## Testing / Verification
- **Unit test:** `config_max_txs_default` to assert default is 300.
- **Integration test:** block builder rejects >300 txs.
- **Integration test:** block builder accepts <=300 txs.
- Run `cargo test -p ethrex-common --lib` and the specific blockchain/storage tests touched by the limit enforcement.

## Open Questions
- Which exact default chain (local devnet vs. other) should be set to chain ID 4062024? I will assume **local devnet / default genesis** only, leaving public networks untouched unless you say otherwise.
- Where is the authoritative block building path in Ethrex (blockchain/mempool/payload builder)? I will locate and apply the limit at the canonical construction point and at validation.
