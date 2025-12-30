# Quranium vs Upstream geth v1.15.0 — Full Diff Report

## Scope
- Baseline: upstream `v1.15.0` tag from `ethereum/go-ethereum`
- Target: current `HEAD` of this repo
- Method: `git diff refs/tags/v1.15.0..HEAD`
- Note: this repo does not share a merge-base with upstream; comparison is a tree diff against the tag.

## High-Level Summary
- Added a full SLHDSA (post-quantum) crypto library and integrated it across signing, recovery, and keystore handling.
- Transaction signing, encoding, and chain-id related behavior is modified across core types and EVM precompiles.
- P2P/discovery and devp2p tools updated to align with new signature scheme.
- Extensive test and tooling additions, including large Solidity and RPC test suites, and new CI workflows.

## Module-by-Module Changes (Focused Report)

### Crypto (SLHDSA integration)
- New SLHDSA library and C sources under `crypto/slhdsa/` with build system artifacts.
- Integration into key handling, signing, and verification via `crypto/crypto.go`, `crypto/signature_cgo.go`.
- New tests for SLHDSA usage and conversion.

Key files:
- `crypto/slhdsa/`
- `crypto/crypto.go`
- `crypto/signature_cgo.go`
- `crypto/crypto_test.go`
- `crypto/signature_test.go`

### Core / Types (Transaction signing & encoding)
- Significant edits across transaction types and signing paths, consistent with new signature handling and chain-id encoding.
- Changes touch legacy, access-list, dynamic-fee, blob, setcode, and related marshalling.

Key files:
- `core/types/transaction.go`
- `core/types/transaction_signing.go`
- `core/types/transaction_marshalling.go`
- `core/types/tx_legacy.go`
- `core/types/tx_access_list.go`
- `core/types/tx_dynamic_fee.go`
- `core/types/tx_blob.go`
- `core/types/tx_setcode.go`

### EVM / Precompiles
- `core/vm/contracts.go` is modified, consistent with a signature scheme swap/extension.

### Accounts / Keystore / Signer
- Keystore formats and passphrase handling updated for new key types.
- Signer APIs/tests updated.

Key files:
- `accounts/keystore/key.go`
- `accounts/keystore/passphrase.go`
- `accounts/keystore/keystore.go`
- `signer/core/uiapi.go`
- `signer/core/signed_data_test.go`

### P2P / Discovery / devp2p tools
- Discovery wire and RLPx crypto changes, likely for new signature types.
- devp2p tools updated accordingly.

Key files:
- `p2p/discover/v4wire/v4wire.go`
- `p2p/discover/v5wire/crypto.go`
- `p2p/rlpx/rlpx.go`
- `cmd/devp2p/*`

### Protocol Params / Node / RPC
- Bootnodes and protocol params altered.
- RPC client and websocket/http updated.

Key files:
- `params/bootnodes.go`
- `params/config.go`
- `params/protocol_params.go`
- `rpc/client.go`
- `rpc/http.go`
- `rpc/websocket.go`

### Eth / Ethclient / GraphQL
- Minor protocol changes and tests updated.
- GraphQL schema and tests adjusted.

Key files:
- `eth/protocols/eth/protocol.go`
- `ethclient/ethclient.go`
- `graphql/schema.go`

### Tests and Tooling
- Large new Solidity contracts, migrations, and tests.
- New RPC test harness and generated reports.
- Added test configs and test runner script.

Key paths:
- `tests/solidity/`
- `tests/rpc_tests/eth_commands/`
- `test_config/`
- `go_test_runner.py`

### CI / Workflows
- New GitHub Actions workflows for unit testing and contract deployment.

Key files:
- `.github/workflows/UnitTest_Runner.yml`
- `.github/workflows/Basic_contract_mainnet_staging.yml`
- `.github/workflows/Basic_contract_testnet_prod.yml`

## Full File List With Stats
```
 .../Basic_contract_mainnet_staging.yml (new)       |    1 +
 .../Basic_contract_testnet_prod.yml (new)          |    1 +
 .github/workflows/RPC_Eth.yml (new)                |    1 +
 .github/workflows/UnitTest_Runner.yml (new)        |  108 +
 .gitignore                                         |   12 +
 Makefile                                           |   14 +-
 accounts/abi/bind/auth.go                          |    6 +-
 accounts/abi/bind/util_test.go                     |   10 +-
 accounts/keystore/key.go                           |   72 +-
 accounts/keystore/keystore.go                      |   33 +-
 accounts/keystore/keystore_test.go                 |   27 +-
 accounts/keystore/passphrase.go                    |   66 +-
 accounts/keystore/passphrase_test.go               |   35 +-
 accounts/keystore/plain_test.go                    |   30 +-
 accounts/keystore/presale.go                       |    6 +-
 .../b8833ffe337e58f7100ca92e713080f66c798bed (new) |    1 +
 ...cb61d5a9c4896fb9658090b597ef0e7be6f7b67e (gone) |    1 -
 accounts/keystore/testdata/very-light-scrypt.json  |    2 +-
 accounts/scwallet/securechannel.go                 |   25 +-
 accounts/scwallet/wallet.go                        |    6 +-
 cmd/clef/consolecmd_test.go                        |   12 +-
 cmd/clef/main.go                                   |    4 +-
 cmd/devp2p/discv4cmd.go                            |    2 +-
 cmd/devp2p/dnscmd.go                               |    2 +-
 cmd/devp2p/internal/ethtest/chain.go               |    8 +-
 cmd/devp2p/internal/ethtest/conn.go                |    2 +-
 cmd/devp2p/internal/ethtest/suite.go               |    2 +-
 cmd/devp2p/internal/v4test/discv4tests.go          |    2 +-
 cmd/devp2p/internal/v4test/framework.go            |    2 +-
 cmd/devp2p/internal/v5test/framework.go            |    2 +-
 cmd/devp2p/keycmd.go                               |    2 +-
 cmd/devp2p/rlpxcmd.go                              |    2 +-
 cmd/ethkey/generate.go                             |    6 +-
 cmd/ethkey/inspect.go                              |    4 +-
 cmd/ethkey/message.go                              |    2 +-
 cmd/evm/internal/t8ntool/block.go                  |    8 +-
 cmd/evm/internal/t8ntool/tx_iterator.go            |   16 +-
 cmd/geth/accountcmd.go                             |    4 +-
 cmd/geth/accountcmd_test.go                        |    8 +-
 cmd/geth/testdata/guswallet.json                   |    2 +-
 cmd/utils/history_test.go                          |    3 +-
 consensus/clique/clique.go                         |    2 +-
 consensus/clique/clique_test.go                    |   10 +-
 consensus/clique/snapshot_test.go                  |    6 +-
 core/bench_test.go                                 |    7 +-
 core/block_validator_test.go                       |    3 +-
 core/blockchain_test.go                            |  184 +-
 core/chain_makers_test.go                          |   11 +-
 core/rawdb/accessors_chain_test.go                 |    2 +-
 core/rlp_test.go                                   |    3 +-
 core/state_processor_test.go                       |   81 +-
 core/txindexer_test.go                             |    2 +-
 core/txpool/blobpool/blobpool_test.go              |   34 +-
 core/txpool/blobpool/evictheap_test.go             |    1 +
 core/txpool/blobpool/priority_test.go              |    1 +
 core/txpool/blobpool/slotter_test.go               |    1 +
 core/txpool/legacypool/legacypool2_test.go         |   12 +-
 core/txpool/legacypool/legacypool_test.go          |   42 +-
 core/types/block_test.go                           |    2 +-
 core/types/gen_authorization.go                    |   35 +-
 core/types/hashing_test.go                         |    9 +-
 core/types/transaction.go                          |   76 +-
 core/types/transaction_marshalling.go              |  220 +-
 core/types/transaction_signing.go                  |  217 +-
 core/types/transaction_signing_test.go             |   50 +-
 core/types/transaction_test.go                     |  128 +-
 core/types/tx_access_list.go                       |   22 +-
 core/types/tx_blob.go                              |   35 +-
 core/types/tx_blob_test.go                         |    7 +-
 core/types/tx_dynamic_fee.go                       |   24 +-
 core/types/tx_legacy.go                            |   32 +-
 core/types/tx_setcode.go                           |   60 +-
 core/types/types_test.go                           |    2 +-
 core/verkle_witness_test.go                        |   33 +-
 core/vm/contracts.go                               |   61 +-
 core/vm/contracts_test.go                          |    2 +-
 crypto/crypto.go                                   |  177 +-
 crypto/crypto_test.go                              |  319 +-
 crypto/signature_cgo.go                            |   48 +-
 crypto/signature_test.go                           |   28 +-
 crypto/slhdsa/.gitignore (new)                     |   40 +
 crypto/slhdsa/libslhdsa/Makefile.am (new)          |   79 +
 crypto/slhdsa/libslhdsa/config.h (new)             |   70 +
 crypto/slhdsa/libslhdsa/config.h.in (new)          |   69 +
 crypto/slhdsa/libslhdsa/configure.ac (new)         |  106 +
 .../include/KeccakP-1600-AVX512-config.h (new)     |    2 +
 .../include/KeccakP-1600-SnP-AVX512.h (new)        |   49 +
 .../include/KeccakP-1600-SnP-reference.h (new)     |   46 +
 .../include/KeccakP-1600-reference.h (new)         |   23 +
 .../libslhdsa/include/KeccakSponge-common.h (new)  |   37 +
 .../include/KeccakSpongeWidth1600.h (new)          |   37 +
 .../libslhdsa/include/SIMD256-config.h (new)       |    3 +
 crypto/slhdsa/libslhdsa/include/adrs.h (new)       |   35 +
 crypto/slhdsa/libslhdsa/include/align.h (new)      |   34 +
 crypto/slhdsa/libslhdsa/include/brg_endian.h (new) |  142 +
 .../slhdsa/libslhdsa/include/crypto_int64.h (new)  |  563 ++
 crypto/slhdsa/libslhdsa/include/external.h (new)   |   18 +
 crypto/slhdsa/libslhdsa/include/fors.h (new)       |   12 +
 crypto/slhdsa/libslhdsa/include/hypertree.h (new)  |    9 +
 crypto/slhdsa/libslhdsa/include/internal.h (new)   |   12 +
 crypto/slhdsa/libslhdsa/include/params.h (new)     |  117 +
 crypto/slhdsa/libslhdsa/include/shake.h (new)      |   25 +
 crypto/slhdsa/libslhdsa/include/slhdsa.h (new)     |  101 +
 crypto/slhdsa/libslhdsa/include/wots.h (new)       |   16 +
 crypto/slhdsa/libslhdsa/include/xmss.h (new)       |   10 +
 crypto/slhdsa/libslhdsa/libslhdsa.pc (new)         |   10 +
 crypto/slhdsa/libslhdsa/libslhdsa.pc.in (new)      |   10 +
 crypto/slhdsa/libslhdsa/m4/libtool.m4 (new)        | 8427 ++++++++++++++++++++
 crypto/slhdsa/libslhdsa/m4/ltoptions.m4 (new)      |  437 +
 crypto/slhdsa/libslhdsa/m4/ltsugar.m4 (new)        |  124 +
 crypto/slhdsa/libslhdsa/m4/ltversion.m4 (new)      |   24 +
 crypto/slhdsa/libslhdsa/m4/lt~obsolete.m4 (new)    |   99 +
 .../slhdsa/libslhdsa/src/KeccakP-1600-AVX2.s (new) | 1104 +++
 .../libslhdsa/src/KeccakP-1600-AVX512.c (new)      |  619 ++
 .../libslhdsa/src/KeccakP-1600-reference.c (new)   |  417 +
 crypto/slhdsa/libslhdsa/src/KeccakSponge.inc (new) |  313 +
 .../libslhdsa/src/KeccakSpongeWidth1600.c (new)    |   58 +
 .../libslhdsa/src/PostquantenCrypto.cbp (new)      |   59 +
 crypto/slhdsa/libslhdsa/src/adrs.c (new)           |   67 +
 crypto/slhdsa/libslhdsa/src/external.c (new)       |  183 +
 crypto/slhdsa/libslhdsa/src/fors.c (new)           |  113 +
 crypto/slhdsa/libslhdsa/src/hypertree.c (new)      |   71 +
 crypto/slhdsa/libslhdsa/src/internal.c (new)       |  158 +
 .../slhdsa/libslhdsa/src/keccak_selected.c (new)   |  619 ++
 crypto/slhdsa/libslhdsa/src/params.c (new)         |   80 +
 crypto/slhdsa/libslhdsa/src/shake.c (new)          |  111 +
 crypto/slhdsa/libslhdsa/src/slhdsa.c (new)         |  146 +
 crypto/slhdsa/libslhdsa/src/wots.c (new)           |  121 +
 crypto/slhdsa/libslhdsa/src/xmss.c (new)           |   88 +
 crypto/slhdsa/libslhdsa/stamp-h1 (new)             |    1 +
 crypto/slhdsa/slhdsa.go (new)                      |  422 +
 crypto/slhdsa_test.go (new)                        |   53 +
 eth/catalyst/api_test.go                           |    3 +-
 eth/catalyst/simulated_beacon_test.go              |    7 +-
 eth/downloader/testchain_test.go                   |    3 +-
 eth/filters/filter_test.go                         |    7 +-
 eth/gasprice/gasprice_test.go                      |    3 +-
 eth/handler_test.go                                |    3 +-
 eth/protocols/eth/handler_test.go                  |   17 +-
 eth/protocols/eth/protocol.go                      |    2 +-
 eth/protocols/eth/protocol_test.go                 |   13 +-
 eth/protocols/snap/protocol.go                     |    2 +-
 eth/tracers/api_test.go                            |   23 +-
 eth/tracers/internal/tracetest/calltrace_test.go   |    3 +-
 eth/tracers/internal/tracetest/supply_test.go      |   13 +-
 .../testdata/call_tracer_legacy/create.json        |  119 +-
 .../testdata/call_tracer_legacy/delegatecall.json  |   74 +-
 .../call_tracer_legacy/inner_instafail.json        |   47 +-
 .../inner_throw_outer_revert.json                  |   52 +-
 .../tracetest/testdata/call_tracer_legacy/oog.json |   24 +-
 .../testdata/call_tracer_legacy/revert.json        |   16 +-
 .../testdata/call_tracer_legacy/revert_reason.json |   39 +-
 .../testdata/call_tracer_legacy/selfdestruct.json  |   49 +-
 .../testdata/call_tracer_legacy/simple.json        |   36 +-
 .../testdata/call_tracer_legacy/throw.json         |   24 +-
 eth/tracers/tracers_test.go                        |    2 +-
 ethclient/ethclient.go                             |    4 +-
 ethclient/ethclient_test.go                        |    5 +-
 ethclient/gethclient/gethclient_test.go            |    5 +-
 ethclient/signer.go                                |    2 +-
 ethclient/simulated/backend_test.go                |   11 +-
 ethclient/simulated/rollback_test.go               |    5 +-
 go.mod                                             |    2 +-
 go.sum                                             |    4 +-
 go_test_runner.py (new)                            |  114 +
 graphql/graphql.go                                 |   73 +-
 graphql/graphql_test.go                            |    5 +-
 graphql/schema.go                                  |    9 +-
 internal/ethapi/api.go                             |   32 +-
 internal/ethapi/api_test.go                        |   78 +-
 .../ethapi/testdata/eth_getBlockByHash-hash-1.json |   44 +-
 .../testdata/eth_getBlockByHash-hash-genesis.json  |   38 +-
 .../eth_getBlockByHash-hash-latest-1-fullTx.json   |   28 +-
 .../testdata/eth_getBlockByHash-hash-latest.json   |   42 +-
 .../testdata/eth_getBlockByNumber-number-0.json    |    4 +-
 .../testdata/eth_getBlockByNumber-number-1.json    |   42 +-
 .../eth_getBlockByNumber-number-latest-1.json      |    2 +-
 .../testdata/eth_getBlockByNumber-tag-latest.json  |   46 +-
 .../testdata/eth_getBlockByNumber-tag-pending.json |    6 +-
 .../testdata/eth_getHeaderByHash-hash-0.json       |   32 +-
 .../testdata/eth_getHeaderByHash-hash-1.json       |    9 +-
 .../eth_getHeaderByHash-hash-latest-1.json         |    9 +-
 .../testdata/eth_getHeaderByHash-hash-latest.json  |   34 +-
 .../testdata/eth_getHeaderByNumber-number-0.json   |    4 +-
 .../testdata/eth_getHeaderByNumber-number-1.json   |    8 +-
 .../testdata/eth_getHeaderByNumber-tag-latest.json |   11 +-
 ...h_getTransactionReceipt-create-contract-tx.json |    8 +-
 ...getTransactionReceipt-dynamic-tx-with-logs.json |    8 +-
 ...h_getTransactionReceipt-normal-transfer-tx.json |    8 +-
 .../eth_getTransactionReceipt-with-logs.json       |   14 +-
 miner/ordering_test.go                             |    9 +-
 miner/worker.go                                    |   35 +
 node/config.go                                     |    4 +-
 node/config_test.go                                |    2 +-
 node/node_test.go                                  |    2 +-
 node/rpcstack.go                                   |    2 +-
 p2p/discover/table_test.go                         |    2 +-
 p2p/discover/v4wire/v4wire.go                      |    4 +-
 p2p/discover/v5wire/crypto.go                      |    8 +-
 p2p/discover/v5wire/encoding_test.go               |    2 +-
 p2p/discover/v5wire/session.go                     |    2 +-
 p2p/dnsdisc/client_test.go                         |    2 +-
 p2p/dnsdisc/tree.go                                |   12 +-
 p2p/enode/idscheme.go                              |    8 +-
 p2p/enode/localnode_test.go                        |    4 +-
 p2p/enode/urlv4.go                                 |    2 +-
 p2p/peer.go                                        |    2 +-
 p2p/rlpx/rlpx.go                                   |   12 +-
 p2p/rlpx/rlpx_test.go                              |    2 +-
 p2p/server_test.go                                 |    2 +-
 p2p/transport_test.go                              |    4 +-
 params/bootnodes.go                                |   60 +-
 params/config.go                                   |   24 +-
 params/protocol_params.go                          |    4 +-
 rpc/client.go                                      |    6 +-
 rpc/http.go                                        |    2 +-
 rpc/websocket.go                                   |    2 +-
 signer/core/api_test.go                            |    7 +-
 signer/core/apitypes/types_test.go                 |    4 +-
 signer/core/signed_data_test.go                    |   14 +-
 signer/core/uiapi.go                               |    4 +-
 test_config/key_constants.go (new)                 |   16 +
 test_config/protocol_test_go/README.md (new)       |   31 +
 .../protocol_test_go/test_messages.go (new)        |    9 +
 tests/evm-benchmarks (gone)                        |    1 -
 tests/rpc_tests/eth_commands/Makefile (new +x)     |   39 +
 tests/rpc_tests/eth_commands/README.md (new +x)    |   57 +
 .../eth_commands/beacon/1_testConnection.js (new)  |   28 +
 .../eth_commands/beacon/Be_testConnection.js (new) |   17 +
 .../beacon/beacon_get_blinded_block.js (new)       |   63 +
 .../beacon/beacon_get_blob_sidecars.js (new)       |   63 +
 .../eth_commands/beacon/beacon_get_block.js (new)  |   63 +
 .../beacon/beacon_get_block_attestations.js (new)  |   63 +
 .../beacon/beacon_get_block_header.js (new)        |   64 +
 .../beacon/beacon_get_block_headers.js (new)       |   76 +
 .../beacon/beacon_get_block_root.js (new)          |   63 +
 .../beacon/beacon_get_committees_states.js (new)   |   55 +
 .../eth_commands/beacon/beacon_get_fork.js (new)   |   72 +
 .../beacon/beacon_get_genesis.js (new)             |   64 +
 .../beacon/beacon_get_pool_attestations.js (new)   |   63 +
 .../beacon/beacon_get_randao_states.js (new)       |   55 +
 .../beacon/beacon_get_rewards.js (new)             |   63 +
 .../eth_commands/beacon/beacon_get_state.js (new)  |   68 +
 ...beacon_get_states_finality_checkpoints.js (new) |   82 +
 .../beacon_get_sync_committees_states.js (new)     |   55 +
 .../beacon_get_validator_balances_states.js (new)  |   70 +
 .../beacon_get_validator_states_by_id_.js (new)    |   66 +
 .../beacon/beacon_get_validators_state.js (new)    |   62 +
 tests/rpc_tests/eth_commands/console_log.log (new) |  107 +
 .../explorer/exp_testConnection.js (new)           |   17 +
 .../explorer/explorer_execution_address.js (new)   |   85 +
 .../explorer_execution_address_paging.js (new)     |  111 +
 .../explorer/explorer_execution_block.js (new)     |   62 +
 .../explorer/explorer_execution_blocks.js (new)    |   61 +
 .../explorer/explorer_execution_search.js (new)    |   86 +
 .../explorer_execution_transactions.js (new)       |   61 +
 .../explorer/explorer_execution_txhash.js (new)    |   88 +
 .../explorer/explorer_overview.js (new)            |   57 +
 .../eth_commands/generate_report.py (new)          |  271 +
 .../rpc_tests/eth_commands/lib/config.js (new +x)  |  146 +
 .../rpc_tests/eth_commands/lib/getIpcPath.js (new) |   19 +
 tests/rpc_tests/eth_commands/lib/helpers.js (new)  |  182 +
 .../eth_commands/lib/ipcProvider.js (new)          |  204 +
 .../BlockchainTests/bcRPC_API_Test.json (new)      | 1366 ++++
 .../eth_commands/logs/eth_test_results.log (new)   |    0
 .../eth_commands/logs/net_test_results.log (new)   |    0
 tests/rpc_tests/eth_commands/package.json (new)    |   30 +
 tests/rpc_tests/eth_commands/report.html (new)     |  188 +
 .../eth_commands/test/1_testConnection.js (new)    |   28 +
 .../eth_commands/test/eth_accounts.js (new)        |   71 +
 .../eth_commands/test/eth_blobBaseFee.js (new)     |   68 +
 .../eth_commands/test/eth_blockNumber.js (new)     |   72 +
 .../rpc_tests/eth_commands/test/eth_call.js (new)  |   75 +
 .../eth_commands/test/eth_chainId.js (new)         |   72 +
 .../eth_commands/test/eth_feeHistory.js (new)      |   79 +
 .../eth_commands/test/eth_fillTransaction.js (new) |   76 +
 .../eth_commands/test/eth_gasPrice.js (new)        |   72 +
 .../eth_commands/test/eth_getBalance.js (new)      |   71 +
 .../test/eth_getBlockByNumber.js (new)             |  101 +
 .../eth_commands/test/eth_getCode.js (new)         |   63 +
 .../eth_commands/test/eth_getHeaderByHash.js (new) |   96 +
 .../test/eth_getHeaderByNumber.js (new)            |   86 +
 .../eth_commands/test/eth_getLogs.js (new)         |   85 +
 .../eth_commands/test/eth_getStorageAt.js (new)    |   70 +
 .../test/eth_getTransactionByHash.js (new)         |   72 +
 .../test/eth_getTransactionCount.js (new)          |   70 +
 .../test/eth_getTransactionReceipt.js (new)        |   66 +
 .../test/eth_getUncleByBlockHashAndIndex.js (new)  |   66 +
 .../eth_getUncleByBlockNumberAndIndex.js (new)     |   67 +
 .../eth_commands/test/eth_newFilter.js (new)       |   70 +
 .../test/eth_pendingTransactions.js (new)          |   71 +
 .../eth_commands/test/net_listening.js (new)       |   55 +
 .../eth_commands/test/net_peerCount.js (new)       |   57 +
 .../eth_commands/test/net_version.js (new)         |   58 +
 .../validator/1_testConnection.js (new)            |   29 +
 .../validator/Va_testConnection.js (new)           |   17 +
 ...lidator_beacon_committee_subscriptions.js (new) |   68 +
 .../validator_contribution_and_proofs.js (new)     |   97 +
 .../validator/validator_get_attester.js (new)      |   61 +
 .../validator_get_block_proposers.js (new)         |   56 +
 .../validator_get_sync_committee.js (new)          |   60 +
 .../validator/validator_liveness_epoch.js (new)    |   72 +
 .../validator_prepare_beacon_proposer.js (new)     |   66 +
 .../validator/validator_produce_new_block.js (new) |   73 +
 .../validator_produce_new_block_wo_sig.js (new)    |   76 +
 .../validator_publish_multiple_aggregate.js (new)  |  106 +
 .../validator_qroduce_attestation.js (new)         |   80 +
 ...validator_sync_committee_subscriptions.js (new) |   82 +
 .../Assembly_Binary_Exponentiation.sol (new)       |   63 +
 .../Bi-Directional_Payment_Channel.sol (new)       |  151 +
 .../Applications/Deploy_Any_Contract.sol (new)     |   71 +
 .../contracts/Applications/Dutch_Auction.sol (new) |   60 +
 .../contracts/Applications/ERC1155.sol (new)       |  260 +
 .../contracts/Applications/ERC20.sol (new)         |  125 +
 .../contracts/Applications/ERC721.sol (new)        |  193 +
 .../Applications/English_Auction.sol (new)         |   85 +
 .../contracts/Applications/Ether_Wallet.sol (new)  |   21 +
 .../Applications/Gasless_Token_Transfer.sol (new)  |  175 +
 .../Applications/Iterable_Mapping.sol (new)        |   84 +
 .../Applications/Merkle Airdrop.sol (new)          |  122 +
 .../contracts/Applications/Merkle_Tree.sol (new)   |   75 +
 .../Applications/Minimal_Proxy_Contract.sol (new)  |   71 +
 .../Applications/Multi Delegatecall.sol (new)      |   67 +
 .../contracts/Applications/Multi_Call.sol (new)    |   32 +
 .../Precompute_Contract_Address.sol (new)          |   96 +
 .../Applications/RW_State_Variable.sol (new)       |   17 +
 .../Simple_Bytecode_Contract.sol (new)             |   56 +
 .../contracts/Applications/TimeLock.sol (new)      |  148 +
 .../Uni-Directional_Payment_Channel.sol (new)      |   97 +
 .../Applications/Upgradable_Proxy.sol (new)        |  251 +
 .../Applications/Write_Any_Slot.sol (new)          |   36 +
 .../solidity/contracts/Basic/Abi_decode.sol (new)  |   33 +
 .../solidity/contracts/Basic/Abi_encode.sol (new)  |   44 +
 .../contracts/Basic/AssemblyCondition.sol (new)    |   23 +
 .../contracts/Basic/AssemblyError.sol (new)        |   13 +
 .../contracts/Basic/AssemblyVariable.sol (new)     |   13 +
 .../contracts/Basic/Assembly_math.sol (new)        |   44 +
 .../contracts/Basic/Assemblyloop.sol (new)         |   20 +
 .../contracts/Basic/BitwiseOperator.sol (new)      |  107 +
 .../Basic/Calling_other_contract.sol (new)         |   39 +
 tests/solidity/contracts/Basic/Constants.sol (new) |    9 +
 .../contracts/Basic/Creating_contracts.sol (new)   |   62 +
 tests/solidity/contracts/Basic/Ether_wei.sol (new) |   16 +
 .../contracts/Basic/Events_advanced.sol (new)      |   75 +
 tests/solidity/contracts/Basic/Fallback.sol (new)  |   45 +
 tests/solidity/contracts/Basic/FirstApp.sol (new)  |   26 +
 tests/solidity/contracts/Basic/Foo.sol (new)       |   17 +
 .../contracts/Basic/For_And_while_loop.sol (new)   |   24 +
 tests/solidity/contracts/Basic/Gasopt.sol (new)    |   46 +
 .../contracts/Basic/HashFunction.sol (new)         |   35 +
 .../solidity/contracts/Basic/HelloWorld.sol (new)  |    6 +
 tests/solidity/contracts/Basic/Ifelse.sol (new)    |   25 +
 tests/solidity/contracts/Basic/Library.sol (new)   |   53 +
 .../contracts/Basic/Primitive_Data_Types.sol (new) |   55 +
 .../contracts/Basic/Reading_and_writing.sol (new)  |   17 +
 .../contracts/Basic/UncheckedMath.sol (new)        |   34 +
 tests/solidity/contracts/Basic/Variables.sol (new) |   16 +
 tests/solidity/contracts/Basic/array.sol (new)     |   62 +
 tests/solidity/contracts/Basic/call.sol (new)      |   51 +
 .../contracts/Basic/callingparent.sol (new)        |   67 +
 .../solidity/contracts/Basic/constructor.sol (new) |   52 +
 tests/solidity/contracts/Basic/dataloc.sol (new)   |   57 +
 .../contracts/Basic/delegatecall.sol (new)         |   48 +
 tests/solidity/contracts/Basic/enum.sol (new)      |   42 +
 tests/solidity/contracts/Basic/error.sol (new)     |   44 +
 tests/solidity/contracts/Basic/events.sol (new)    |   16 +
 tests/solidity/contracts/Basic/function.sol (new)  |   76 +
 .../solidity/contracts/Basic/functionmod.sol (new) |   59 +
 .../solidity/contracts/Basic/functionsel.sol (new) |   18 +
 tests/solidity/contracts/Basic/immutable.sol (new) |   12 +
 tests/solidity/contracts/Basic/import.sol (new)    |   18 +
 .../solidity/contracts/Basic/inheritance.sol (new) |   61 +
 tests/solidity/contracts/Basic/interface.sol (new) |   54 +
 tests/solidity/contracts/Basic/mapping.sol (new)   |   42 +
 tests/solidity/contracts/Basic/payable.sol (new)   |   38 +
 .../contracts/Basic/sendingether.sol (new)         |   51 +
 .../solidity/contracts/Basic/shadowstate.sol (new) |   25 +
 tests/solidity/contracts/Basic/structs.sol (new)   |   51 +
 tests/solidity/contracts/Basic/trycatch.sol (new)  |   58 +
 tests/solidity/contracts/Basic/tstorage.sol (new)  |  111 +
 .../solidity/contracts/Basic/valuetypes.sol (new)  |   86 +
 tests/solidity/contracts/Basic/view.sol (new)      |   16 +
 .../solidity/contracts/Basic/visibility.sol (new)  |   66 +
 tests/solidity/contracts/EVM/EVM_Memory.sol (new)  |  618 ++
 tests/solidity/contracts/EVM/EVM_Storage.sol (new) |  560 ++
 .../solidity/contracts/Hacks/63_Gas_rule.sol (new) |   72 +
 .../contracts/Hacks/Accessing_Private.sol (new)    |  111 +
 .../contracts/Hacks/Arithmetic_overflow.sol (new)  |   67 +
 .../contracts/Hacks/Block_Timestamp.sol (new)      |   35 +
 tests/solidity/contracts/Hacks/Bypass.sol (new)    |   45 +
 .../contracts/Hacks/Denial_of_service.sol (new)    |   58 +
 .../contracts/Hacks/Hacks_Delegatecall.sol (new)   |   60 +
 tests/solidity/contracts/Hacks/Honeypot.sol (new)  |   82 +
 tests/solidity/contracts/Hacks/Phishing.sol (new)  |   51 +
 .../solidity/contracts/Hacks/Re_entrancy.sol (new) |   79 +
 .../contracts/Hacks/Self_destruct.sol (new)        |   59 +
 .../contracts/Hacks/Source_of_Randomness.sol (new) |   61 +
 tests/solidity/contracts/Hacks/malicious.sol (new) |   53 +
 tests/solidity/contracts/Migrations.sol            |   31 +-
 .../migrations/10_assembly_math_migration.js (new) |    5 +
 .../11_assembly_variable_migration.js (new)        |   24 +
 .../12_bitwise_operator_migration.js (new)         |   23 +
 .../13_block_timestamp_migration.js (new)          |    7 +
 .../migrations/14_bypass_migration.js (new)        |    5 +
 .../15_calling_other_contract_migration.js (new)   |    7 +
 .../migrations/16_callingparent_migration.js (new) |   20 +
 .../solidity/migrations/17_call_migration.js (new) |    7 +
 .../migrations/18_Constants_migration.js (new)     |   21 +
 .../migrations/19_constructor_migration.js (new)   |   40 +
 .../migrations/1_63_gas_rule_migration.js (new)    |    8 +
 .../20_creating_contracts_migration.js (new)       |    5 +
 .../migrations/21_dataloc_migration.js (new)       |   23 +
 .../migrations/22_delegatecall_migration.js (new)  |   29 +
 .../migrations/23_denial_migration.js (new)        |    9 +
 .../24_deploy_any_contract_migration.js (new)      |   23 +
 .../solidity/migrations/25_enum_migration.js (new) |   23 +
 .../migrations/26_error_migration.js (new)         |   26 +
 .../27_events_advanced_migration.js (new)          |   26 +
 .../migrations/28_events_migration.js (new)        |   23 +
 .../migrations/29_evm_memory_migration.js (new)    |   42 +
 .../migrations/2_abi_decode_migration.js (new)     |    5 +
 .../migrations/30_evm_storage_migration.js (new)   |   45 +
 .../migrations/31_fallback_migration.js (new)      |   16 +
 .../migrations/32_firstapp_migration.js (new)      |   21 +
 .../33_for_and_while_loop_migration.js (new)       |    5 +
 .../migrations/34_function_migration.js (new)      |   27 +
 .../migrations/35_functionmod_migration.js (new)   |   24 +
 .../migrations/36_functionsel_migration.js (new)   |   23 +
 .../migrations/37_gas_opt_migration.js (new)       |   25 +
 .../38_hacks_delegatecall_migration.js (new)       |   15 +
 .../migrations/39_hashfunction_migration.js (new)  |   27 +
 .../migrations/3_abi_encode_migration.js (new)     |    7 +
 .../migrations/40_helloWorld_migration.js (new)    |   21 +
 .../migrations/41_honeypot_migration.js (new)      |   11 +
 .../migrations/42_immutable_migration.js (new)     |   26 +
 .../migrations/43_import_migration.js (new)        |    5 +
 .../migrations/44_inheritance_migration.js (new)   |   34 +
 .../migrations/45_initial_migration.js (new)       |   21 +
 .../migrations/46_interface_migration.js (new)     |   27 +
 .../migrations/47_library_migration.js (new)       |   30 +
 .../migrations/48_malicious_migration.js (new)     |   16 +
 .../migrations/49_mapping_migration.js (new)       |   27 +
 .../4_accessing_private_migration.js (new)         |   14 +
 .../migrations/50_opCodes_migration.js (new)       |   21 +
 .../migrations/51_payable_migration.js (new)       |    5 +
 .../migrations/52_phising_migration.js (new)       |   11 +
 .../53_primitive_data_types_migration.js (new)     |   21 +
 .../54_reading_and_writing_migration.js (new)      |    5 +
 .../migrations/55_re_entrancy_migration.js (new)   |    9 +
 .../migrations/56_self_destruct_migration.js (new) |    9 +
 .../migrations/57_sendingether_migration.js (new)  |   23 +
 .../migrations/58_shadowstate_migration.js (new)   |   30 +
 .../59_source_of_randomness_migration.js (new)     |   18 +
 .../5_arithmetic_overflow_migration.js (new)       |   12 +
 .../migrations/60_structs_migration.js (new)       |   21 +
 .../migrations/61_trycatch_migration.js (new)      |   25 +
 .../migrations/62_tstorage_migration.js (new)      |   25 +
 .../63_unchecked_math_migration.js (new)           |   24 +
 .../migrations/64_valuetypes_migration.js (new)    |   23 +
 .../65_Variables__migrsation_migration.js (new)    |   21 +
 .../solidity/migrations/66_view_migration.js (new) |   24 +
 .../migrations/67_visibility_migration.js (new)    |   27 +
 .../solidity/migrations/6_array_migration.js (new) |    6 +
 .../7_assembly_conditions_migration.js (new)       |   23 +
 .../migrations/8_assembly_error_migration.js (new) |   22 +
 .../migrations/9_assembly_loop_migration.js (new)  |   22 +
 tests/solidity/test/Basic/abi_decode.js (new)      |   41 +
 tests/solidity/test/Basic/abi_encode.js (new)      |   46 +
 tests/solidity/test/Basic/array.js (new)           |   57 +
 tests/solidity/test/Basic/assembly_math.js (new)   |   58 +
 .../test/Basic/assemblyconditions.js (new)         |   60 +
 tests/solidity/test/Basic/assemblyerror.js (new)   |   59 +
 tests/solidity/test/Basic/assemblyloop.js (new)    |   40 +
 .../solidity/test/Basic/assemblyvariable.js (new)  |   34 +
 tests/solidity/test/Basic/bitwiseoperator.js (new) |   76 +
 tests/solidity/test/Basic/call.js (new)            |   42 +
 .../test/Basic/calling_other_contract.js (new)     |   42 +
 tests/solidity/test/Basic/callingparent.js (new)   |   41 +
 tests/solidity/test/Basic/constants.js (new)       |   40 +
 tests/solidity/test/Basic/constructor.js (new)     |  128 +
 .../test/Basic/creating_contracts.js (new)         |   52 +
 tests/solidity/test/Basic/dataloc.js (new)         |   40 +
 tests/solidity/test/Basic/delegatecall.js (new)    |   50 +
 tests/solidity/test/Basic/enum.js (new)            |   49 +
 tests/solidity/test/Basic/error.js (new)           |   65 +
 tests/solidity/test/Basic/ether_and_wei.js (new)   |   33 +
 tests/solidity/test/Basic/events.js (new)          |   41 +
 tests/solidity/test/Basic/events_advanced.js (new) |   74 +
 tests/solidity/test/Basic/fallback.js (new)        |   28 +
 tests/solidity/test/Basic/firstappp.js (new)       |   56 +
 .../test/Basic/for_and_while_loop.js (new)         |   14 +
 tests/solidity/test/Basic/function.js (new)        |   81 +
 tests/solidity/test/Basic/functionmod.js (new)     |   55 +
 tests/solidity/test/Basic/functionsel.js (new)     |   38 +
 .../solidity/test/Basic/gas_and_gasprice.js (new)  |   26 +
 tests/solidity/test/Basic/gasopt.js (new)          |   38 +
 tests/solidity/test/Basic/hashfunction.js (new)    |   62 +
 tests/solidity/test/Basic/helloWorld.js (new)      |   33 +
 tests/solidity/test/Basic/ifelse.js (new)          |   38 +
 tests/solidity/test/Basic/immutable.js (new)       |   38 +
 tests/solidity/test/Basic/import.js (new)          |    9 +
 tests/solidity/test/Basic/inheritance.js (new)     |   64 +
 tests/solidity/test/Basic/interface.js (new)       |   43 +
 tests/solidity/test/Basic/library.js (new)         |   58 +
 tests/solidity/test/Basic/mapping.js (new)         |   69 +
 tests/solidity/test/Basic/payable.js (new)         |   74 +
 .../test/Basic/primitive_data_types.js (new)       |   58 +
 .../test/Basic/reading_and_writing.js (new)        |   26 +
 tests/solidity/test/Basic/sendingether.js (new)    |   69 +
 tests/solidity/test/Basic/shadowstate.js (new)     |   41 +
 tests/solidity/test/Basic/structs.js (new)         |   55 +
 tests/solidity/test/Basic/trycatch.js (new)        |   60 +
 tests/solidity/test/Basic/tstorage.js (new)        |   51 +
 tests/solidity/test/Basic/uncheckedmath.js (new)   |   47 +
 tests/solidity/test/Basic/valuetypes.js (new)      |   34 +
 tests/solidity/test/Basic/variables.js (new)       |   44 +
 tests/solidity/test/Basic/view.js (new)            |   36 +
 tests/solidity/test/Basic/visibility.js (new)      |   61 +
 tests/solidity/test/EVM/evm_memory.js (new)        |  279 +
 tests/solidity/test/EVM/evm_storage.js (new)       |  349 +
 tests/solidity/test/Hacks/63_gas_rule.js (new)     |   45 +
 .../solidity/test/Hacks/accessing_private.js (new) |  114 +
 tests/solidity/test/Hacks/block_timestamp.js (new) |   42 +
 tests/solidity/test/Hacks/bypass.js (new)          |   37 +
 tests/solidity/test/Hacks/honeypot.js (new)        |   34 +
 tests/solidity/test/Hacks/malicious.js (new)       |   49 +
 tests/solidity/test/Hacks/re_entrancy.js (new)     |   30 +
 .../test/Hacks/source_of_randomness.js (new)       |   34 +
 tests/solidity/test/opCodes.js                     |   50 +-
 tests/solidity/truffle-config.js                   |    5 +
 tests/state_test_util.go                           |    8 +-
 tests/testdata (gone)                              |    1 -
 532 files changed, 37010 insertions(+), 1567 deletions(-)
```

## Full File List With Status
```
A	.github/workflows/Basic_contract_mainnet_staging.yml
A	.github/workflows/Basic_contract_testnet_prod.yml
A	.github/workflows/RPC_Eth.yml
A	.github/workflows/UnitTest_Runner.yml
M	.gitignore
M	Makefile
M	accounts/abi/bind/auth.go
M	accounts/abi/bind/util_test.go
M	accounts/keystore/key.go
M	accounts/keystore/keystore.go
M	accounts/keystore/keystore_test.go
M	accounts/keystore/passphrase.go
M	accounts/keystore/passphrase_test.go
M	accounts/keystore/plain_test.go
M	accounts/keystore/presale.go
A	accounts/keystore/testdata/v1/b8833ffe337e58f7100ca92e713080f66c798bed/b8833ffe337e58f7100ca92e713080f66c798bed
D	accounts/keystore/testdata/v1/cb61d5a9c4896fb9658090b597ef0e7be6f7b67e/cb61d5a9c4896fb9658090b597ef0e7be6f7b67e
M	accounts/keystore/testdata/very-light-scrypt.json
M	accounts/scwallet/securechannel.go
M	accounts/scwallet/wallet.go
M	cmd/clef/consolecmd_test.go
M	cmd/clef/main.go
M	cmd/devp2p/discv4cmd.go
M	cmd/devp2p/dnscmd.go
M	cmd/devp2p/internal/ethtest/chain.go
M	cmd/devp2p/internal/ethtest/conn.go
M	cmd/devp2p/internal/ethtest/suite.go
M	cmd/devp2p/internal/v4test/discv4tests.go
M	cmd/devp2p/internal/v4test/framework.go
M	cmd/devp2p/internal/v5test/framework.go
M	cmd/devp2p/keycmd.go
M	cmd/devp2p/rlpxcmd.go
M	cmd/ethkey/generate.go
M	cmd/ethkey/inspect.go
M	cmd/ethkey/message.go
M	cmd/evm/internal/t8ntool/block.go
M	cmd/evm/internal/t8ntool/tx_iterator.go
M	cmd/geth/accountcmd.go
M	cmd/geth/accountcmd_test.go
M	cmd/geth/testdata/guswallet.json
M	cmd/utils/history_test.go
M	consensus/clique/clique.go
M	consensus/clique/clique_test.go
M	consensus/clique/snapshot_test.go
M	core/bench_test.go
M	core/block_validator_test.go
M	core/blockchain_test.go
M	core/chain_makers_test.go
M	core/rawdb/accessors_chain_test.go
M	core/rlp_test.go
M	core/state_processor_test.go
M	core/txindexer_test.go
M	core/txpool/blobpool/blobpool_test.go
M	core/txpool/blobpool/evictheap_test.go
M	core/txpool/blobpool/priority_test.go
M	core/txpool/blobpool/slotter_test.go
M	core/txpool/legacypool/legacypool2_test.go
M	core/txpool/legacypool/legacypool_test.go
M	core/types/block_test.go
M	core/types/gen_authorization.go
M	core/types/hashing_test.go
M	core/types/transaction.go
M	core/types/transaction_marshalling.go
M	core/types/transaction_signing.go
M	core/types/transaction_signing_test.go
M	core/types/transaction_test.go
M	core/types/tx_access_list.go
M	core/types/tx_blob.go
M	core/types/tx_blob_test.go
M	core/types/tx_dynamic_fee.go
M	core/types/tx_legacy.go
M	core/types/tx_setcode.go
M	core/types/types_test.go
M	core/verkle_witness_test.go
M	core/vm/contracts.go
M	core/vm/contracts_test.go
M	crypto/crypto.go
M	crypto/crypto_test.go
M	crypto/signature_cgo.go
M	crypto/signature_test.go
A	crypto/slhdsa/.gitignore
A	crypto/slhdsa/libslhdsa/Makefile.am
A	crypto/slhdsa/libslhdsa/config.h
A	crypto/slhdsa/libslhdsa/config.h.in
A	crypto/slhdsa/libslhdsa/configure.ac
A	crypto/slhdsa/libslhdsa/include/KeccakP-1600-AVX512-config.h
A	crypto/slhdsa/libslhdsa/include/KeccakP-1600-SnP-AVX512.h
A	crypto/slhdsa/libslhdsa/include/KeccakP-1600-SnP-reference.h
A	crypto/slhdsa/libslhdsa/include/KeccakP-1600-reference.h
A	crypto/slhdsa/libslhdsa/include/KeccakSponge-common.h
A	crypto/slhdsa/libslhdsa/include/KeccakSpongeWidth1600.h
A	crypto/slhdsa/libslhdsa/include/SIMD256-config.h
A	crypto/slhdsa/libslhdsa/include/adrs.h
A	crypto/slhdsa/libslhdsa/include/align.h
A	crypto/slhdsa/libslhdsa/include/brg_endian.h
A	crypto/slhdsa/libslhdsa/include/crypto_int64.h
A	crypto/slhdsa/libslhdsa/include/external.h
A	crypto/slhdsa/libslhdsa/include/fors.h
A	crypto/slhdsa/libslhdsa/include/hypertree.h
A	crypto/slhdsa/libslhdsa/include/internal.h
A	crypto/slhdsa/libslhdsa/include/params.h
A	crypto/slhdsa/libslhdsa/include/shake.h
A	crypto/slhdsa/libslhdsa/include/slhdsa.h
A	crypto/slhdsa/libslhdsa/include/wots.h
A	crypto/slhdsa/libslhdsa/include/xmss.h
A	crypto/slhdsa/libslhdsa/libslhdsa.pc
A	crypto/slhdsa/libslhdsa/libslhdsa.pc.in
A	crypto/slhdsa/libslhdsa/m4/libtool.m4
A	crypto/slhdsa/libslhdsa/m4/ltoptions.m4
A	crypto/slhdsa/libslhdsa/m4/ltsugar.m4
A	crypto/slhdsa/libslhdsa/m4/ltversion.m4
A	crypto/slhdsa/libslhdsa/m4/lt~obsolete.m4
A	crypto/slhdsa/libslhdsa/src/KeccakP-1600-AVX2.s
A	crypto/slhdsa/libslhdsa/src/KeccakP-1600-AVX512.c
A	crypto/slhdsa/libslhdsa/src/KeccakP-1600-reference.c
A	crypto/slhdsa/libslhdsa/src/KeccakSponge.inc
A	crypto/slhdsa/libslhdsa/src/KeccakSpongeWidth1600.c
A	crypto/slhdsa/libslhdsa/src/PostquantenCrypto.cbp
A	crypto/slhdsa/libslhdsa/src/adrs.c
A	crypto/slhdsa/libslhdsa/src/external.c
A	crypto/slhdsa/libslhdsa/src/fors.c
A	crypto/slhdsa/libslhdsa/src/hypertree.c
A	crypto/slhdsa/libslhdsa/src/internal.c
A	crypto/slhdsa/libslhdsa/src/keccak_selected.c
A	crypto/slhdsa/libslhdsa/src/params.c
A	crypto/slhdsa/libslhdsa/src/shake.c
A	crypto/slhdsa/libslhdsa/src/slhdsa.c
A	crypto/slhdsa/libslhdsa/src/wots.c
A	crypto/slhdsa/libslhdsa/src/xmss.c
A	crypto/slhdsa/libslhdsa/stamp-h1
A	crypto/slhdsa/slhdsa.go
A	crypto/slhdsa_test.go
M	eth/catalyst/api_test.go
M	eth/catalyst/simulated_beacon_test.go
M	eth/downloader/testchain_test.go
M	eth/filters/filter_test.go
M	eth/gasprice/gasprice_test.go
M	eth/handler_test.go
M	eth/protocols/eth/handler_test.go
M	eth/protocols/eth/protocol.go
M	eth/protocols/eth/protocol_test.go
M	eth/protocols/snap/protocol.go
M	eth/tracers/api_test.go
M	eth/tracers/internal/tracetest/calltrace_test.go
M	eth/tracers/internal/tracetest/supply_test.go
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/create.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/delegatecall.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/inner_instafail.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/inner_throw_outer_revert.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/oog.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/revert.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/revert_reason.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/selfdestruct.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/simple.json
M	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/throw.json
M	eth/tracers/tracers_test.go
M	ethclient/ethclient.go
M	ethclient/ethclient_test.go
M	ethclient/gethclient/gethclient_test.go
M	ethclient/signer.go
M	ethclient/simulated/backend_test.go
M	ethclient/simulated/rollback_test.go
M	go.mod
M	go.sum
A	go_test_runner.py
M	graphql/graphql.go
M	graphql/graphql_test.go
M	graphql/schema.go
M	internal/ethapi/api.go
M	internal/ethapi/api_test.go
M	internal/ethapi/testdata/eth_getBlockByHash-hash-1.json
M	internal/ethapi/testdata/eth_getBlockByHash-hash-genesis.json
M	internal/ethapi/testdata/eth_getBlockByHash-hash-latest-1-fullTx.json
M	internal/ethapi/testdata/eth_getBlockByHash-hash-latest.json
M	internal/ethapi/testdata/eth_getBlockByNumber-number-0.json
M	internal/ethapi/testdata/eth_getBlockByNumber-number-1.json
M	internal/ethapi/testdata/eth_getBlockByNumber-number-latest-1.json
M	internal/ethapi/testdata/eth_getBlockByNumber-tag-latest.json
M	internal/ethapi/testdata/eth_getBlockByNumber-tag-pending.json
M	internal/ethapi/testdata/eth_getHeaderByHash-hash-0.json
M	internal/ethapi/testdata/eth_getHeaderByHash-hash-1.json
M	internal/ethapi/testdata/eth_getHeaderByHash-hash-latest-1.json
M	internal/ethapi/testdata/eth_getHeaderByHash-hash-latest.json
M	internal/ethapi/testdata/eth_getHeaderByNumber-number-0.json
M	internal/ethapi/testdata/eth_getHeaderByNumber-number-1.json
M	internal/ethapi/testdata/eth_getHeaderByNumber-tag-latest.json
M	internal/ethapi/testdata/eth_getTransactionReceipt-create-contract-tx.json
M	internal/ethapi/testdata/eth_getTransactionReceipt-dynamic-tx-with-logs.json
M	internal/ethapi/testdata/eth_getTransactionReceipt-normal-transfer-tx.json
M	internal/ethapi/testdata/eth_getTransactionReceipt-with-logs.json
M	miner/ordering_test.go
M	miner/worker.go
M	node/config.go
M	node/config_test.go
M	node/node_test.go
M	node/rpcstack.go
M	p2p/discover/table_test.go
M	p2p/discover/v4wire/v4wire.go
M	p2p/discover/v5wire/crypto.go
M	p2p/discover/v5wire/encoding_test.go
M	p2p/discover/v5wire/session.go
M	p2p/dnsdisc/client_test.go
M	p2p/dnsdisc/tree.go
M	p2p/enode/idscheme.go
M	p2p/enode/localnode_test.go
M	p2p/enode/urlv4.go
M	p2p/peer.go
M	p2p/rlpx/rlpx.go
M	p2p/rlpx/rlpx_test.go
M	p2p/server_test.go
M	p2p/transport_test.go
M	params/bootnodes.go
M	params/config.go
M	params/protocol_params.go
M	rpc/client.go
M	rpc/http.go
M	rpc/websocket.go
M	signer/core/api_test.go
M	signer/core/apitypes/types_test.go
M	signer/core/signed_data_test.go
M	signer/core/uiapi.go
A	test_config/key_constants.go
A	test_config/protocol_test_go/README.md
A	test_config/protocol_test_go/test_messages.go
D	tests/evm-benchmarks
A	tests/rpc_tests/eth_commands/Makefile
A	tests/rpc_tests/eth_commands/README.md
A	tests/rpc_tests/eth_commands/beacon/1_testConnection.js
A	tests/rpc_tests/eth_commands/beacon/Be_testConnection.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_blinded_block.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_blob_sidecars.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_block.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_block_attestations.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_block_header.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_block_headers.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_block_root.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_committees_states.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_fork.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_genesis.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_pool_attestations.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_randao_states.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_rewards.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_state.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_states_finality_checkpoints.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_sync_committees_states.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_validator_balances_states.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_validator_states_by_id_.js
A	tests/rpc_tests/eth_commands/beacon/beacon_get_validators_state.js
A	tests/rpc_tests/eth_commands/console_log.log
A	tests/rpc_tests/eth_commands/explorer/exp_testConnection.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_address.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_address_paging.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_block.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_blocks.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_search.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_transactions.js
A	tests/rpc_tests/eth_commands/explorer/explorer_execution_txhash.js
A	tests/rpc_tests/eth_commands/explorer/explorer_overview.js
A	tests/rpc_tests/eth_commands/generate_report.py
A	tests/rpc_tests/eth_commands/lib/config.js
A	tests/rpc_tests/eth_commands/lib/getIpcPath.js
A	tests/rpc_tests/eth_commands/lib/helpers.js
A	tests/rpc_tests/eth_commands/lib/ipcProvider.js
A	tests/rpc_tests/eth_commands/lib/tests/BlockchainTests/bcRPC_API_Test.json
A	tests/rpc_tests/eth_commands/logs/eth_test_results.log
A	tests/rpc_tests/eth_commands/logs/net_test_results.log
A	tests/rpc_tests/eth_commands/package.json
A	tests/rpc_tests/eth_commands/report.html
A	tests/rpc_tests/eth_commands/test/1_testConnection.js
A	tests/rpc_tests/eth_commands/test/eth_accounts.js
A	tests/rpc_tests/eth_commands/test/eth_blobBaseFee.js
A	tests/rpc_tests/eth_commands/test/eth_blockNumber.js
A	tests/rpc_tests/eth_commands/test/eth_call.js
A	tests/rpc_tests/eth_commands/test/eth_chainId.js
A	tests/rpc_tests/eth_commands/test/eth_feeHistory.js
A	tests/rpc_tests/eth_commands/test/eth_fillTransaction.js
A	tests/rpc_tests/eth_commands/test/eth_gasPrice.js
A	tests/rpc_tests/eth_commands/test/eth_getBalance.js
A	tests/rpc_tests/eth_commands/test/eth_getBlockByNumber.js
A	tests/rpc_tests/eth_commands/test/eth_getCode.js
A	tests/rpc_tests/eth_commands/test/eth_getHeaderByHash.js
A	tests/rpc_tests/eth_commands/test/eth_getHeaderByNumber.js
A	tests/rpc_tests/eth_commands/test/eth_getLogs.js
A	tests/rpc_tests/eth_commands/test/eth_getStorageAt.js
A	tests/rpc_tests/eth_commands/test/eth_getTransactionByHash.js
A	tests/rpc_tests/eth_commands/test/eth_getTransactionCount.js
A	tests/rpc_tests/eth_commands/test/eth_getTransactionReceipt.js
A	tests/rpc_tests/eth_commands/test/eth_getUncleByBlockHashAndIndex.js
A	tests/rpc_tests/eth_commands/test/eth_getUncleByBlockNumberAndIndex.js
A	tests/rpc_tests/eth_commands/test/eth_newFilter.js
A	tests/rpc_tests/eth_commands/test/eth_pendingTransactions.js
A	tests/rpc_tests/eth_commands/test/net_listening.js
A	tests/rpc_tests/eth_commands/test/net_peerCount.js
A	tests/rpc_tests/eth_commands/test/net_version.js
A	tests/rpc_tests/eth_commands/validator/1_testConnection.js
A	tests/rpc_tests/eth_commands/validator/Va_testConnection.js
A	tests/rpc_tests/eth_commands/validator/validator_beacon_committee_subscriptions.js
A	tests/rpc_tests/eth_commands/validator/validator_contribution_and_proofs.js
A	tests/rpc_tests/eth_commands/validator/validator_get_attester.js
A	tests/rpc_tests/eth_commands/validator/validator_get_block_proposers.js
A	tests/rpc_tests/eth_commands/validator/validator_get_sync_committee.js
A	tests/rpc_tests/eth_commands/validator/validator_liveness_epoch.js
A	tests/rpc_tests/eth_commands/validator/validator_prepare_beacon_proposer.js
A	tests/rpc_tests/eth_commands/validator/validator_produce_new_block.js
A	tests/rpc_tests/eth_commands/validator/validator_produce_new_block_wo_sig.js
A	tests/rpc_tests/eth_commands/validator/validator_publish_multiple_aggregate.js
A	tests/rpc_tests/eth_commands/validator/validator_qroduce_attestation.js
A	tests/rpc_tests/eth_commands/validator/validator_sync_committee_subscriptions.js
A	tests/solidity/contracts/Applications/Assembly_Binary_Exponentiation.sol
A	tests/solidity/contracts/Applications/Bi-Directional_Payment_Channel.sol
A	tests/solidity/contracts/Applications/Deploy_Any_Contract.sol
A	tests/solidity/contracts/Applications/Dutch_Auction.sol
A	tests/solidity/contracts/Applications/ERC1155.sol
A	tests/solidity/contracts/Applications/ERC20.sol
A	tests/solidity/contracts/Applications/ERC721.sol
A	tests/solidity/contracts/Applications/English_Auction.sol
A	tests/solidity/contracts/Applications/Ether_Wallet.sol
A	tests/solidity/contracts/Applications/Gasless_Token_Transfer.sol
A	tests/solidity/contracts/Applications/Iterable_Mapping.sol
A	tests/solidity/contracts/Applications/Merkle Airdrop.sol
A	tests/solidity/contracts/Applications/Merkle_Tree.sol
A	tests/solidity/contracts/Applications/Minimal_Proxy_Contract.sol
A	tests/solidity/contracts/Applications/Multi Delegatecall.sol
A	tests/solidity/contracts/Applications/Multi_Call.sol
A	tests/solidity/contracts/Applications/Precompute_Contract_Address.sol
A	tests/solidity/contracts/Applications/RW_State_Variable.sol
A	tests/solidity/contracts/Applications/Simple_Bytecode_Contract.sol
A	tests/solidity/contracts/Applications/TimeLock.sol
A	tests/solidity/contracts/Applications/Uni-Directional_Payment_Channel.sol
A	tests/solidity/contracts/Applications/Upgradable_Proxy.sol
A	tests/solidity/contracts/Applications/Write_Any_Slot.sol
A	tests/solidity/contracts/Basic/Abi_decode.sol
A	tests/solidity/contracts/Basic/Abi_encode.sol
A	tests/solidity/contracts/Basic/AssemblyCondition.sol
A	tests/solidity/contracts/Basic/AssemblyError.sol
A	tests/solidity/contracts/Basic/AssemblyVariable.sol
A	tests/solidity/contracts/Basic/Assembly_math.sol
A	tests/solidity/contracts/Basic/Assemblyloop.sol
A	tests/solidity/contracts/Basic/BitwiseOperator.sol
A	tests/solidity/contracts/Basic/Calling_other_contract.sol
A	tests/solidity/contracts/Basic/Constants.sol
A	tests/solidity/contracts/Basic/Creating_contracts.sol
A	tests/solidity/contracts/Basic/Ether_wei.sol
A	tests/solidity/contracts/Basic/Events_advanced.sol
A	tests/solidity/contracts/Basic/Fallback.sol
A	tests/solidity/contracts/Basic/FirstApp.sol
A	tests/solidity/contracts/Basic/Foo.sol
A	tests/solidity/contracts/Basic/For_And_while_loop.sol
A	tests/solidity/contracts/Basic/Gasopt.sol
A	tests/solidity/contracts/Basic/HashFunction.sol
A	tests/solidity/contracts/Basic/HelloWorld.sol
A	tests/solidity/contracts/Basic/Ifelse.sol
A	tests/solidity/contracts/Basic/Library.sol
A	tests/solidity/contracts/Basic/Primitive_Data_Types.sol
A	tests/solidity/contracts/Basic/Reading_and_writing.sol
A	tests/solidity/contracts/Basic/UncheckedMath.sol
A	tests/solidity/contracts/Basic/Variables.sol
A	tests/solidity/contracts/Basic/array.sol
A	tests/solidity/contracts/Basic/call.sol
A	tests/solidity/contracts/Basic/callingparent.sol
A	tests/solidity/contracts/Basic/constructor.sol
A	tests/solidity/contracts/Basic/dataloc.sol
A	tests/solidity/contracts/Basic/delegatecall.sol
A	tests/solidity/contracts/Basic/enum.sol
A	tests/solidity/contracts/Basic/error.sol
A	tests/solidity/contracts/Basic/events.sol
A	tests/solidity/contracts/Basic/function.sol
A	tests/solidity/contracts/Basic/functionmod.sol
A	tests/solidity/contracts/Basic/functionsel.sol
A	tests/solidity/contracts/Basic/immutable.sol
A	tests/solidity/contracts/Basic/import.sol
A	tests/solidity/contracts/Basic/inheritance.sol
A	tests/solidity/contracts/Basic/interface.sol
A	tests/solidity/contracts/Basic/mapping.sol
A	tests/solidity/contracts/Basic/payable.sol
A	tests/solidity/contracts/Basic/sendingether.sol
A	tests/solidity/contracts/Basic/shadowstate.sol
A	tests/solidity/contracts/Basic/structs.sol
A	tests/solidity/contracts/Basic/trycatch.sol
A	tests/solidity/contracts/Basic/tstorage.sol
A	tests/solidity/contracts/Basic/valuetypes.sol
A	tests/solidity/contracts/Basic/view.sol
A	tests/solidity/contracts/Basic/visibility.sol
A	tests/solidity/contracts/EVM/EVM_Memory.sol
A	tests/solidity/contracts/EVM/EVM_Storage.sol
A	tests/solidity/contracts/Hacks/63_Gas_rule.sol
A	tests/solidity/contracts/Hacks/Accessing_Private.sol
A	tests/solidity/contracts/Hacks/Arithmetic_overflow.sol
A	tests/solidity/contracts/Hacks/Block_Timestamp.sol
A	tests/solidity/contracts/Hacks/Bypass.sol
A	tests/solidity/contracts/Hacks/Denial_of_service.sol
A	tests/solidity/contracts/Hacks/Hacks_Delegatecall.sol
A	tests/solidity/contracts/Hacks/Honeypot.sol
A	tests/solidity/contracts/Hacks/Phishing.sol
A	tests/solidity/contracts/Hacks/Re_entrancy.sol
A	tests/solidity/contracts/Hacks/Self_destruct.sol
A	tests/solidity/contracts/Hacks/Source_of_Randomness.sol
A	tests/solidity/contracts/Hacks/malicious.sol
M	tests/solidity/contracts/Migrations.sol
A	tests/solidity/migrations/10_assembly_math_migration.js
A	tests/solidity/migrations/11_assembly_variable_migration.js
A	tests/solidity/migrations/12_bitwise_operator_migration.js
A	tests/solidity/migrations/13_block_timestamp_migration.js
A	tests/solidity/migrations/14_bypass_migration.js
A	tests/solidity/migrations/15_calling_other_contract_migration.js
A	tests/solidity/migrations/16_callingparent_migration.js
A	tests/solidity/migrations/17_call_migration.js
A	tests/solidity/migrations/18_Constants_migration.js
A	tests/solidity/migrations/19_constructor_migration.js
A	tests/solidity/migrations/1_63_gas_rule_migration.js
A	tests/solidity/migrations/20_creating_contracts_migration.js
A	tests/solidity/migrations/21_dataloc_migration.js
A	tests/solidity/migrations/22_delegatecall_migration.js
A	tests/solidity/migrations/23_denial_migration.js
A	tests/solidity/migrations/24_deploy_any_contract_migration.js
A	tests/solidity/migrations/25_enum_migration.js
A	tests/solidity/migrations/26_error_migration.js
A	tests/solidity/migrations/27_events_advanced_migration.js
A	tests/solidity/migrations/28_events_migration.js
A	tests/solidity/migrations/29_evm_memory_migration.js
A	tests/solidity/migrations/2_abi_decode_migration.js
A	tests/solidity/migrations/30_evm_storage_migration.js
A	tests/solidity/migrations/31_fallback_migration.js
A	tests/solidity/migrations/32_firstapp_migration.js
A	tests/solidity/migrations/33_for_and_while_loop_migration.js
A	tests/solidity/migrations/34_function_migration.js
A	tests/solidity/migrations/35_functionmod_migration.js
A	tests/solidity/migrations/36_functionsel_migration.js
A	tests/solidity/migrations/37_gas_opt_migration.js
A	tests/solidity/migrations/38_hacks_delegatecall_migration.js
A	tests/solidity/migrations/39_hashfunction_migration.js
A	tests/solidity/migrations/3_abi_encode_migration.js
A	tests/solidity/migrations/40_helloWorld_migration.js
A	tests/solidity/migrations/41_honeypot_migration.js
A	tests/solidity/migrations/42_immutable_migration.js
A	tests/solidity/migrations/43_import_migration.js
A	tests/solidity/migrations/44_inheritance_migration.js
A	tests/solidity/migrations/45_initial_migration.js
A	tests/solidity/migrations/46_interface_migration.js
A	tests/solidity/migrations/47_library_migration.js
A	tests/solidity/migrations/48_malicious_migration.js
A	tests/solidity/migrations/49_mapping_migration.js
A	tests/solidity/migrations/4_accessing_private_migration.js
A	tests/solidity/migrations/50_opCodes_migration.js
A	tests/solidity/migrations/51_payable_migration.js
A	tests/solidity/migrations/52_phising_migration.js
A	tests/solidity/migrations/53_primitive_data_types_migration.js
A	tests/solidity/migrations/54_reading_and_writing_migration.js
A	tests/solidity/migrations/55_re_entrancy_migration.js
A	tests/solidity/migrations/56_self_destruct_migration.js
A	tests/solidity/migrations/57_sendingether_migration.js
A	tests/solidity/migrations/58_shadowstate_migration.js
A	tests/solidity/migrations/59_source_of_randomness_migration.js
A	tests/solidity/migrations/5_arithmetic_overflow_migration.js
A	tests/solidity/migrations/60_structs_migration.js
A	tests/solidity/migrations/61_trycatch_migration.js
A	tests/solidity/migrations/62_tstorage_migration.js
A	tests/solidity/migrations/63_unchecked_math_migration.js
A	tests/solidity/migrations/64_valuetypes_migration.js
A	tests/solidity/migrations/65_Variables__migrsation_migration.js
A	tests/solidity/migrations/66_view_migration.js
A	tests/solidity/migrations/67_visibility_migration.js
A	tests/solidity/migrations/6_array_migration.js
A	tests/solidity/migrations/7_assembly_conditions_migration.js
A	tests/solidity/migrations/8_assembly_error_migration.js
A	tests/solidity/migrations/9_assembly_loop_migration.js
A	tests/solidity/test/Basic/abi_decode.js
A	tests/solidity/test/Basic/abi_encode.js
A	tests/solidity/test/Basic/array.js
A	tests/solidity/test/Basic/assembly_math.js
A	tests/solidity/test/Basic/assemblyconditions.js
A	tests/solidity/test/Basic/assemblyerror.js
A	tests/solidity/test/Basic/assemblyloop.js
A	tests/solidity/test/Basic/assemblyvariable.js
A	tests/solidity/test/Basic/bitwiseoperator.js
A	tests/solidity/test/Basic/call.js
A	tests/solidity/test/Basic/calling_other_contract.js
A	tests/solidity/test/Basic/callingparent.js
A	tests/solidity/test/Basic/constants.js
A	tests/solidity/test/Basic/constructor.js
A	tests/solidity/test/Basic/creating_contracts.js
A	tests/solidity/test/Basic/dataloc.js
A	tests/solidity/test/Basic/delegatecall.js
A	tests/solidity/test/Basic/enum.js
A	tests/solidity/test/Basic/error.js
A	tests/solidity/test/Basic/ether_and_wei.js
A	tests/solidity/test/Basic/events.js
A	tests/solidity/test/Basic/events_advanced.js
A	tests/solidity/test/Basic/fallback.js
A	tests/solidity/test/Basic/firstappp.js
A	tests/solidity/test/Basic/for_and_while_loop.js
A	tests/solidity/test/Basic/function.js
A	tests/solidity/test/Basic/functionmod.js
A	tests/solidity/test/Basic/functionsel.js
A	tests/solidity/test/Basic/gas_and_gasprice.js
A	tests/solidity/test/Basic/gasopt.js
A	tests/solidity/test/Basic/hashfunction.js
A	tests/solidity/test/Basic/helloWorld.js
A	tests/solidity/test/Basic/ifelse.js
A	tests/solidity/test/Basic/immutable.js
A	tests/solidity/test/Basic/import.js
A	tests/solidity/test/Basic/inheritance.js
A	tests/solidity/test/Basic/interface.js
A	tests/solidity/test/Basic/library.js
A	tests/solidity/test/Basic/mapping.js
A	tests/solidity/test/Basic/payable.js
A	tests/solidity/test/Basic/primitive_data_types.js
A	tests/solidity/test/Basic/reading_and_writing.js
A	tests/solidity/test/Basic/sendingether.js
A	tests/solidity/test/Basic/shadowstate.js
A	tests/solidity/test/Basic/structs.js
A	tests/solidity/test/Basic/trycatch.js
A	tests/solidity/test/Basic/tstorage.js
A	tests/solidity/test/Basic/uncheckedmath.js
A	tests/solidity/test/Basic/valuetypes.js
A	tests/solidity/test/Basic/variables.js
A	tests/solidity/test/Basic/view.js
A	tests/solidity/test/Basic/visibility.js
A	tests/solidity/test/EVM/evm_memory.js
A	tests/solidity/test/EVM/evm_storage.js
A	tests/solidity/test/Hacks/63_gas_rule.js
A	tests/solidity/test/Hacks/accessing_private.js
A	tests/solidity/test/Hacks/block_timestamp.js
A	tests/solidity/test/Hacks/bypass.js
A	tests/solidity/test/Hacks/honeypot.js
A	tests/solidity/test/Hacks/malicious.js
A	tests/solidity/test/Hacks/re_entrancy.js
A	tests/solidity/test/Hacks/source_of_randomness.js
M	tests/solidity/test/opCodes.js
M	tests/solidity/truffle-config.js
M	tests/state_test_util.go
D	tests/testdata
```
