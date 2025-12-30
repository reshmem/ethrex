# Quranium vs Upstream geth v1.15.0 — Exact Change List

## Scope and Method
- Baseline: `refs/tags/v1.15.0` (upstream geth)
- Target: `HEAD`
- Commands used:
  - `git diff --shortstat refs/tags/v1.15.0..HEAD`
  - `git diff --name-status refs/tags/v1.15.0..HEAD`
  - `git diff --numstat refs/tags/v1.15.0..HEAD`

## Summary
 532 files changed, 37010 insertions(+), 1567 deletions(-)

## File Status List (A/M/D)
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

## File Line Counts (Added/Deleted)
```
1	0	.github/workflows/Basic_contract_mainnet_staging.yml
1	0	.github/workflows/Basic_contract_testnet_prod.yml
1	0	.github/workflows/RPC_Eth.yml
108	0	.github/workflows/UnitTest_Runner.yml
12	0	.gitignore
11	3	Makefile
3	3	accounts/abi/bind/auth.go
6	4	accounts/abi/bind/util_test.go
63	9	accounts/keystore/key.go
27	6	accounts/keystore/keystore.go
23	4	accounts/keystore/keystore_test.go
65	1	accounts/keystore/passphrase.go
33	2	accounts/keystore/passphrase_test.go
23	7	accounts/keystore/plain_test.go
3	3	accounts/keystore/presale.go
1	0	accounts/keystore/testdata/v1/b8833ffe337e58f7100ca92e713080f66c798bed/b8833ffe337e58f7100ca92e713080f66c798bed
0	1	accounts/keystore/testdata/v1/cb61d5a9c4896fb9658090b597ef0e7be6f7b67e/cb61d5a9c4896fb9658090b597ef0e7be6f7b67e
1	1	accounts/keystore/testdata/very-light-scrypt.json
22	3	accounts/scwallet/securechannel.go
3	3	accounts/scwallet/wallet.go
6	6	cmd/clef/consolecmd_test.go
2	2	cmd/clef/main.go
1	1	cmd/devp2p/discv4cmd.go
1	1	cmd/devp2p/dnscmd.go
4	4	cmd/devp2p/internal/ethtest/chain.go
1	1	cmd/devp2p/internal/ethtest/conn.go
1	1	cmd/devp2p/internal/ethtest/suite.go
1	1	cmd/devp2p/internal/v4test/discv4tests.go
1	1	cmd/devp2p/internal/v4test/framework.go
1	1	cmd/devp2p/internal/v5test/framework.go
1	1	cmd/devp2p/keycmd.go
1	1	cmd/devp2p/rlpxcmd.go
3	3	cmd/ethkey/generate.go
2	2	cmd/ethkey/inspect.go
1	1	cmd/ethkey/message.go
4	4	cmd/evm/internal/t8ntool/block.go
8	8	cmd/evm/internal/t8ntool/tx_iterator.go
2	2	cmd/geth/accountcmd.go
4	4	cmd/geth/accountcmd_test.go
1	1	cmd/geth/testdata/guswallet.json
2	1	cmd/utils/history_test.go
1	1	consensus/clique/clique.go
6	4	consensus/clique/clique_test.go
3	3	consensus/clique/snapshot_test.go
4	3	core/bench_test.go
2	1	core/block_validator_test.go
135	49	core/blockchain_test.go
6	5	core/chain_makers_test.go
1	1	core/rawdb/accessors_chain_test.go
2	1	core/rlp_test.go
53	28	core/state_processor_test.go
1	1	core/txindexer_test.go
22	12	core/txpool/blobpool/blobpool_test.go
1	0	core/txpool/blobpool/evictheap_test.go
1	0	core/txpool/blobpool/priority_test.go
1	0	core/txpool/blobpool/slotter_test.go
7	5	core/txpool/legacypool/legacypool2_test.go
21	21	core/txpool/legacypool/legacypool_test.go
1	1	core/types/block_test.go
17	18	core/types/gen_authorization.go
5	4	core/types/hashing_test.go
43	33	core/types/transaction.go
115	105	core/types/transaction_marshalling.go
136	81	core/types/transaction_signing.go
24	26	core/types/transaction_signing_test.go
75	53	core/types/transaction_test.go
10	12	core/types/tx_access_list.go
19	16	core/types/tx_blob.go
4	3	core/types/tx_blob_test.go
10	14	core/types/tx_dynamic_fee.go
19	13	core/types/tx_legacy.go
30	30	core/types/tx_setcode.go
1	1	core/types/types_test.go
17	16	core/verkle_witness_test.go
27	34	core/vm/contracts.go
1	1	core/vm/contracts_test.go
160	17	crypto/crypto.go
222	97	crypto/crypto_test.go
42	6	crypto/signature_cgo.go
15	13	crypto/signature_test.go
40	0	crypto/slhdsa/.gitignore
79	0	crypto/slhdsa/libslhdsa/Makefile.am
70	0	crypto/slhdsa/libslhdsa/config.h
69	0	crypto/slhdsa/libslhdsa/config.h.in
106	0	crypto/slhdsa/libslhdsa/configure.ac
2	0	crypto/slhdsa/libslhdsa/include/KeccakP-1600-AVX512-config.h
49	0	crypto/slhdsa/libslhdsa/include/KeccakP-1600-SnP-AVX512.h
46	0	crypto/slhdsa/libslhdsa/include/KeccakP-1600-SnP-reference.h
23	0	crypto/slhdsa/libslhdsa/include/KeccakP-1600-reference.h
37	0	crypto/slhdsa/libslhdsa/include/KeccakSponge-common.h
37	0	crypto/slhdsa/libslhdsa/include/KeccakSpongeWidth1600.h
3	0	crypto/slhdsa/libslhdsa/include/SIMD256-config.h
35	0	crypto/slhdsa/libslhdsa/include/adrs.h
34	0	crypto/slhdsa/libslhdsa/include/align.h
142	0	crypto/slhdsa/libslhdsa/include/brg_endian.h
563	0	crypto/slhdsa/libslhdsa/include/crypto_int64.h
18	0	crypto/slhdsa/libslhdsa/include/external.h
12	0	crypto/slhdsa/libslhdsa/include/fors.h
9	0	crypto/slhdsa/libslhdsa/include/hypertree.h
12	0	crypto/slhdsa/libslhdsa/include/internal.h
117	0	crypto/slhdsa/libslhdsa/include/params.h
25	0	crypto/slhdsa/libslhdsa/include/shake.h
101	0	crypto/slhdsa/libslhdsa/include/slhdsa.h
16	0	crypto/slhdsa/libslhdsa/include/wots.h
10	0	crypto/slhdsa/libslhdsa/include/xmss.h
10	0	crypto/slhdsa/libslhdsa/libslhdsa.pc
10	0	crypto/slhdsa/libslhdsa/libslhdsa.pc.in
8427	0	crypto/slhdsa/libslhdsa/m4/libtool.m4
437	0	crypto/slhdsa/libslhdsa/m4/ltoptions.m4
124	0	crypto/slhdsa/libslhdsa/m4/ltsugar.m4
24	0	crypto/slhdsa/libslhdsa/m4/ltversion.m4
99	0	crypto/slhdsa/libslhdsa/m4/lt~obsolete.m4
1104	0	crypto/slhdsa/libslhdsa/src/KeccakP-1600-AVX2.s
619	0	crypto/slhdsa/libslhdsa/src/KeccakP-1600-AVX512.c
417	0	crypto/slhdsa/libslhdsa/src/KeccakP-1600-reference.c
313	0	crypto/slhdsa/libslhdsa/src/KeccakSponge.inc
58	0	crypto/slhdsa/libslhdsa/src/KeccakSpongeWidth1600.c
59	0	crypto/slhdsa/libslhdsa/src/PostquantenCrypto.cbp
67	0	crypto/slhdsa/libslhdsa/src/adrs.c
183	0	crypto/slhdsa/libslhdsa/src/external.c
113	0	crypto/slhdsa/libslhdsa/src/fors.c
71	0	crypto/slhdsa/libslhdsa/src/hypertree.c
158	0	crypto/slhdsa/libslhdsa/src/internal.c
619	0	crypto/slhdsa/libslhdsa/src/keccak_selected.c
80	0	crypto/slhdsa/libslhdsa/src/params.c
111	0	crypto/slhdsa/libslhdsa/src/shake.c
146	0	crypto/slhdsa/libslhdsa/src/slhdsa.c
121	0	crypto/slhdsa/libslhdsa/src/wots.c
88	0	crypto/slhdsa/libslhdsa/src/xmss.c
1	0	crypto/slhdsa/libslhdsa/stamp-h1
422	0	crypto/slhdsa/slhdsa.go
53	0	crypto/slhdsa_test.go
2	1	eth/catalyst/api_test.go
4	3	eth/catalyst/simulated_beacon_test.go
2	1	eth/downloader/testchain_test.go
4	3	eth/filters/filter_test.go
2	1	eth/gasprice/gasprice_test.go
2	1	eth/handler_test.go
9	8	eth/protocols/eth/handler_test.go
1	1	eth/protocols/eth/protocol.go
7	6	eth/protocols/eth/protocol_test.go
1	1	eth/protocols/snap/protocol.go
12	11	eth/tracers/api_test.go
2	1	eth/tracers/internal/tracetest/calltrace_test.go
7	6	eth/tracers/internal/tracetest/supply_test.go
72	47	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/create.json
37	37	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/delegatecall.json
24	23	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/inner_instafail.json
26	26	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/inner_throw_outer_revert.json
12	12	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/oog.json
8	8	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/revert.json
18	21	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/revert_reason.json
25	24	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/selfdestruct.json
13	23	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/simple.json
12	12	eth/tracers/internal/tracetest/testdata/call_tracer_legacy/throw.json
1	1	eth/tracers/tracers_test.go
2	2	ethclient/ethclient.go
3	2	ethclient/ethclient_test.go
3	2	ethclient/gethclient/gethclient_test.go
1	1	ethclient/signer.go
6	5	ethclient/simulated/backend_test.go
2	3	ethclient/simulated/rollback_test.go
1	1	go.mod
2	2	go.sum
114	0	go_test_runner.py
41	32	graphql/graphql.go
3	2	graphql/graphql_test.go
5	4	graphql/schema.go
17	15	internal/ethapi/api.go
34	44	internal/ethapi/api_test.go
22	22	internal/ethapi/testdata/eth_getBlockByHash-hash-1.json
19	19	internal/ethapi/testdata/eth_getBlockByHash-hash-genesis.json
12	16	internal/ethapi/testdata/eth_getBlockByHash-hash-latest-1-fullTx.json
21	21	internal/ethapi/testdata/eth_getBlockByHash-hash-latest.json
2	2	internal/ethapi/testdata/eth_getBlockByNumber-number-0.json
21	21	internal/ethapi/testdata/eth_getBlockByNumber-number-1.json
1	1	internal/ethapi/testdata/eth_getBlockByNumber-number-latest-1.json
23	23	internal/ethapi/testdata/eth_getBlockByNumber-tag-latest.json
3	3	internal/ethapi/testdata/eth_getBlockByNumber-tag-pending.json
16	16	internal/ethapi/testdata/eth_getHeaderByHash-hash-0.json
5	4	internal/ethapi/testdata/eth_getHeaderByHash-hash-1.json
5	4	internal/ethapi/testdata/eth_getHeaderByHash-hash-latest-1.json
17	17	internal/ethapi/testdata/eth_getHeaderByHash-hash-latest.json
2	2	internal/ethapi/testdata/eth_getHeaderByNumber-number-0.json
4	4	internal/ethapi/testdata/eth_getHeaderByNumber-number-1.json
6	5	internal/ethapi/testdata/eth_getHeaderByNumber-tag-latest.json
4	4	internal/ethapi/testdata/eth_getTransactionReceipt-create-contract-tx.json
4	4	internal/ethapi/testdata/eth_getTransactionReceipt-dynamic-tx-with-logs.json
4	4	internal/ethapi/testdata/eth_getTransactionReceipt-normal-transfer-tx.json
7	7	internal/ethapi/testdata/eth_getTransactionReceipt-with-logs.json
5	4	miner/ordering_test.go
35	0	miner/worker.go
2	2	node/config.go
1	1	node/config_test.go
1	1	node/node_test.go
1	1	node/rpcstack.go
1	1	p2p/discover/table_test.go
2	2	p2p/discover/v4wire/v4wire.go
4	4	p2p/discover/v5wire/crypto.go
1	1	p2p/discover/v5wire/encoding_test.go
1	1	p2p/discover/v5wire/session.go
1	1	p2p/dnsdisc/client_test.go
6	6	p2p/dnsdisc/tree.go
4	4	p2p/enode/idscheme.go
2	2	p2p/enode/localnode_test.go
1	1	p2p/enode/urlv4.go
1	1	p2p/peer.go
6	6	p2p/rlpx/rlpx.go
1	1	p2p/rlpx/rlpx_test.go
1	1	p2p/server_test.go
2	2	p2p/transport_test.go
30	30	params/bootnodes.go
17	7	params/config.go
3	1	params/protocol_params.go
3	3	rpc/client.go
1	1	rpc/http.go
1	1	rpc/websocket.go
4	3	signer/core/api_test.go
2	2	signer/core/apitypes/types_test.go
5	9	signer/core/signed_data_test.go
2	2	signer/core/uiapi.go
16	0	test_config/key_constants.go
31	0	test_config/protocol_test_go/README.md
9	0	test_config/protocol_test_go/test_messages.go
0	1	tests/evm-benchmarks
39	0	tests/rpc_tests/eth_commands/Makefile
57	0	tests/rpc_tests/eth_commands/README.md
28	0	tests/rpc_tests/eth_commands/beacon/1_testConnection.js
17	0	tests/rpc_tests/eth_commands/beacon/Be_testConnection.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_blinded_block.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_blob_sidecars.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_block.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_block_attestations.js
64	0	tests/rpc_tests/eth_commands/beacon/beacon_get_block_header.js
76	0	tests/rpc_tests/eth_commands/beacon/beacon_get_block_headers.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_block_root.js
55	0	tests/rpc_tests/eth_commands/beacon/beacon_get_committees_states.js
72	0	tests/rpc_tests/eth_commands/beacon/beacon_get_fork.js
64	0	tests/rpc_tests/eth_commands/beacon/beacon_get_genesis.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_pool_attestations.js
55	0	tests/rpc_tests/eth_commands/beacon/beacon_get_randao_states.js
63	0	tests/rpc_tests/eth_commands/beacon/beacon_get_rewards.js
68	0	tests/rpc_tests/eth_commands/beacon/beacon_get_state.js
82	0	tests/rpc_tests/eth_commands/beacon/beacon_get_states_finality_checkpoints.js
55	0	tests/rpc_tests/eth_commands/beacon/beacon_get_sync_committees_states.js
70	0	tests/rpc_tests/eth_commands/beacon/beacon_get_validator_balances_states.js
66	0	tests/rpc_tests/eth_commands/beacon/beacon_get_validator_states_by_id_.js
62	0	tests/rpc_tests/eth_commands/beacon/beacon_get_validators_state.js
107	0	tests/rpc_tests/eth_commands/console_log.log
17	0	tests/rpc_tests/eth_commands/explorer/exp_testConnection.js
85	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_address.js
111	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_address_paging.js
62	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_block.js
61	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_blocks.js
86	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_search.js
61	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_transactions.js
88	0	tests/rpc_tests/eth_commands/explorer/explorer_execution_txhash.js
57	0	tests/rpc_tests/eth_commands/explorer/explorer_overview.js
271	0	tests/rpc_tests/eth_commands/generate_report.py
146	0	tests/rpc_tests/eth_commands/lib/config.js
19	0	tests/rpc_tests/eth_commands/lib/getIpcPath.js
182	0	tests/rpc_tests/eth_commands/lib/helpers.js
204	0	tests/rpc_tests/eth_commands/lib/ipcProvider.js
1366	0	tests/rpc_tests/eth_commands/lib/tests/BlockchainTests/bcRPC_API_Test.json
0	0	tests/rpc_tests/eth_commands/logs/eth_test_results.log
0	0	tests/rpc_tests/eth_commands/logs/net_test_results.log
30	0	tests/rpc_tests/eth_commands/package.json
188	0	tests/rpc_tests/eth_commands/report.html
28	0	tests/rpc_tests/eth_commands/test/1_testConnection.js
71	0	tests/rpc_tests/eth_commands/test/eth_accounts.js
68	0	tests/rpc_tests/eth_commands/test/eth_blobBaseFee.js
72	0	tests/rpc_tests/eth_commands/test/eth_blockNumber.js
75	0	tests/rpc_tests/eth_commands/test/eth_call.js
72	0	tests/rpc_tests/eth_commands/test/eth_chainId.js
79	0	tests/rpc_tests/eth_commands/test/eth_feeHistory.js
76	0	tests/rpc_tests/eth_commands/test/eth_fillTransaction.js
72	0	tests/rpc_tests/eth_commands/test/eth_gasPrice.js
71	0	tests/rpc_tests/eth_commands/test/eth_getBalance.js
101	0	tests/rpc_tests/eth_commands/test/eth_getBlockByNumber.js
63	0	tests/rpc_tests/eth_commands/test/eth_getCode.js
96	0	tests/rpc_tests/eth_commands/test/eth_getHeaderByHash.js
86	0	tests/rpc_tests/eth_commands/test/eth_getHeaderByNumber.js
85	0	tests/rpc_tests/eth_commands/test/eth_getLogs.js
70	0	tests/rpc_tests/eth_commands/test/eth_getStorageAt.js
72	0	tests/rpc_tests/eth_commands/test/eth_getTransactionByHash.js
70	0	tests/rpc_tests/eth_commands/test/eth_getTransactionCount.js
66	0	tests/rpc_tests/eth_commands/test/eth_getTransactionReceipt.js
66	0	tests/rpc_tests/eth_commands/test/eth_getUncleByBlockHashAndIndex.js
67	0	tests/rpc_tests/eth_commands/test/eth_getUncleByBlockNumberAndIndex.js
70	0	tests/rpc_tests/eth_commands/test/eth_newFilter.js
71	0	tests/rpc_tests/eth_commands/test/eth_pendingTransactions.js
55	0	tests/rpc_tests/eth_commands/test/net_listening.js
57	0	tests/rpc_tests/eth_commands/test/net_peerCount.js
58	0	tests/rpc_tests/eth_commands/test/net_version.js
29	0	tests/rpc_tests/eth_commands/validator/1_testConnection.js
17	0	tests/rpc_tests/eth_commands/validator/Va_testConnection.js
68	0	tests/rpc_tests/eth_commands/validator/validator_beacon_committee_subscriptions.js
97	0	tests/rpc_tests/eth_commands/validator/validator_contribution_and_proofs.js
61	0	tests/rpc_tests/eth_commands/validator/validator_get_attester.js
56	0	tests/rpc_tests/eth_commands/validator/validator_get_block_proposers.js
60	0	tests/rpc_tests/eth_commands/validator/validator_get_sync_committee.js
72	0	tests/rpc_tests/eth_commands/validator/validator_liveness_epoch.js
66	0	tests/rpc_tests/eth_commands/validator/validator_prepare_beacon_proposer.js
73	0	tests/rpc_tests/eth_commands/validator/validator_produce_new_block.js
76	0	tests/rpc_tests/eth_commands/validator/validator_produce_new_block_wo_sig.js
106	0	tests/rpc_tests/eth_commands/validator/validator_publish_multiple_aggregate.js
80	0	tests/rpc_tests/eth_commands/validator/validator_qroduce_attestation.js
82	0	tests/rpc_tests/eth_commands/validator/validator_sync_committee_subscriptions.js
63	0	tests/solidity/contracts/Applications/Assembly_Binary_Exponentiation.sol
151	0	tests/solidity/contracts/Applications/Bi-Directional_Payment_Channel.sol
71	0	tests/solidity/contracts/Applications/Deploy_Any_Contract.sol
60	0	tests/solidity/contracts/Applications/Dutch_Auction.sol
260	0	tests/solidity/contracts/Applications/ERC1155.sol
125	0	tests/solidity/contracts/Applications/ERC20.sol
193	0	tests/solidity/contracts/Applications/ERC721.sol
85	0	tests/solidity/contracts/Applications/English_Auction.sol
21	0	tests/solidity/contracts/Applications/Ether_Wallet.sol
175	0	tests/solidity/contracts/Applications/Gasless_Token_Transfer.sol
84	0	tests/solidity/contracts/Applications/Iterable_Mapping.sol
122	0	tests/solidity/contracts/Applications/Merkle Airdrop.sol
75	0	tests/solidity/contracts/Applications/Merkle_Tree.sol
71	0	tests/solidity/contracts/Applications/Minimal_Proxy_Contract.sol
67	0	tests/solidity/contracts/Applications/Multi Delegatecall.sol
32	0	tests/solidity/contracts/Applications/Multi_Call.sol
96	0	tests/solidity/contracts/Applications/Precompute_Contract_Address.sol
17	0	tests/solidity/contracts/Applications/RW_State_Variable.sol
56	0	tests/solidity/contracts/Applications/Simple_Bytecode_Contract.sol
148	0	tests/solidity/contracts/Applications/TimeLock.sol
97	0	tests/solidity/contracts/Applications/Uni-Directional_Payment_Channel.sol
251	0	tests/solidity/contracts/Applications/Upgradable_Proxy.sol
36	0	tests/solidity/contracts/Applications/Write_Any_Slot.sol
33	0	tests/solidity/contracts/Basic/Abi_decode.sol
44	0	tests/solidity/contracts/Basic/Abi_encode.sol
23	0	tests/solidity/contracts/Basic/AssemblyCondition.sol
13	0	tests/solidity/contracts/Basic/AssemblyError.sol
13	0	tests/solidity/contracts/Basic/AssemblyVariable.sol
44	0	tests/solidity/contracts/Basic/Assembly_math.sol
20	0	tests/solidity/contracts/Basic/Assemblyloop.sol
107	0	tests/solidity/contracts/Basic/BitwiseOperator.sol
39	0	tests/solidity/contracts/Basic/Calling_other_contract.sol
9	0	tests/solidity/contracts/Basic/Constants.sol
62	0	tests/solidity/contracts/Basic/Creating_contracts.sol
16	0	tests/solidity/contracts/Basic/Ether_wei.sol
75	0	tests/solidity/contracts/Basic/Events_advanced.sol
45	0	tests/solidity/contracts/Basic/Fallback.sol
26	0	tests/solidity/contracts/Basic/FirstApp.sol
17	0	tests/solidity/contracts/Basic/Foo.sol
24	0	tests/solidity/contracts/Basic/For_And_while_loop.sol
46	0	tests/solidity/contracts/Basic/Gasopt.sol
35	0	tests/solidity/contracts/Basic/HashFunction.sol
6	0	tests/solidity/contracts/Basic/HelloWorld.sol
25	0	tests/solidity/contracts/Basic/Ifelse.sol
53	0	tests/solidity/contracts/Basic/Library.sol
55	0	tests/solidity/contracts/Basic/Primitive_Data_Types.sol
17	0	tests/solidity/contracts/Basic/Reading_and_writing.sol
34	0	tests/solidity/contracts/Basic/UncheckedMath.sol
16	0	tests/solidity/contracts/Basic/Variables.sol
62	0	tests/solidity/contracts/Basic/array.sol
51	0	tests/solidity/contracts/Basic/call.sol
67	0	tests/solidity/contracts/Basic/callingparent.sol
52	0	tests/solidity/contracts/Basic/constructor.sol
57	0	tests/solidity/contracts/Basic/dataloc.sol
48	0	tests/solidity/contracts/Basic/delegatecall.sol
42	0	tests/solidity/contracts/Basic/enum.sol
44	0	tests/solidity/contracts/Basic/error.sol
16	0	tests/solidity/contracts/Basic/events.sol
76	0	tests/solidity/contracts/Basic/function.sol
59	0	tests/solidity/contracts/Basic/functionmod.sol
18	0	tests/solidity/contracts/Basic/functionsel.sol
12	0	tests/solidity/contracts/Basic/immutable.sol
18	0	tests/solidity/contracts/Basic/import.sol
61	0	tests/solidity/contracts/Basic/inheritance.sol
54	0	tests/solidity/contracts/Basic/interface.sol
42	0	tests/solidity/contracts/Basic/mapping.sol
38	0	tests/solidity/contracts/Basic/payable.sol
51	0	tests/solidity/contracts/Basic/sendingether.sol
25	0	tests/solidity/contracts/Basic/shadowstate.sol
51	0	tests/solidity/contracts/Basic/structs.sol
58	0	tests/solidity/contracts/Basic/trycatch.sol
111	0	tests/solidity/contracts/Basic/tstorage.sol
86	0	tests/solidity/contracts/Basic/valuetypes.sol
16	0	tests/solidity/contracts/Basic/view.sol
66	0	tests/solidity/contracts/Basic/visibility.sol
618	0	tests/solidity/contracts/EVM/EVM_Memory.sol
560	0	tests/solidity/contracts/EVM/EVM_Storage.sol
72	0	tests/solidity/contracts/Hacks/63_Gas_rule.sol
111	0	tests/solidity/contracts/Hacks/Accessing_Private.sol
67	0	tests/solidity/contracts/Hacks/Arithmetic_overflow.sol
35	0	tests/solidity/contracts/Hacks/Block_Timestamp.sol
45	0	tests/solidity/contracts/Hacks/Bypass.sol
58	0	tests/solidity/contracts/Hacks/Denial_of_service.sol
60	0	tests/solidity/contracts/Hacks/Hacks_Delegatecall.sol
82	0	tests/solidity/contracts/Hacks/Honeypot.sol
51	0	tests/solidity/contracts/Hacks/Phishing.sol
79	0	tests/solidity/contracts/Hacks/Re_entrancy.sol
59	0	tests/solidity/contracts/Hacks/Self_destruct.sol
61	0	tests/solidity/contracts/Hacks/Source_of_Randomness.sol
53	0	tests/solidity/contracts/Hacks/malicious.sol
16	15	tests/solidity/contracts/Migrations.sol
5	0	tests/solidity/migrations/10_assembly_math_migration.js
24	0	tests/solidity/migrations/11_assembly_variable_migration.js
23	0	tests/solidity/migrations/12_bitwise_operator_migration.js
7	0	tests/solidity/migrations/13_block_timestamp_migration.js
5	0	tests/solidity/migrations/14_bypass_migration.js
7	0	tests/solidity/migrations/15_calling_other_contract_migration.js
20	0	tests/solidity/migrations/16_callingparent_migration.js
7	0	tests/solidity/migrations/17_call_migration.js
21	0	tests/solidity/migrations/18_Constants_migration.js
40	0	tests/solidity/migrations/19_constructor_migration.js
8	0	tests/solidity/migrations/1_63_gas_rule_migration.js
5	0	tests/solidity/migrations/20_creating_contracts_migration.js
23	0	tests/solidity/migrations/21_dataloc_migration.js
29	0	tests/solidity/migrations/22_delegatecall_migration.js
9	0	tests/solidity/migrations/23_denial_migration.js
23	0	tests/solidity/migrations/24_deploy_any_contract_migration.js
23	0	tests/solidity/migrations/25_enum_migration.js
26	0	tests/solidity/migrations/26_error_migration.js
26	0	tests/solidity/migrations/27_events_advanced_migration.js
23	0	tests/solidity/migrations/28_events_migration.js
42	0	tests/solidity/migrations/29_evm_memory_migration.js
5	0	tests/solidity/migrations/2_abi_decode_migration.js
45	0	tests/solidity/migrations/30_evm_storage_migration.js
16	0	tests/solidity/migrations/31_fallback_migration.js
21	0	tests/solidity/migrations/32_firstapp_migration.js
5	0	tests/solidity/migrations/33_for_and_while_loop_migration.js
27	0	tests/solidity/migrations/34_function_migration.js
24	0	tests/solidity/migrations/35_functionmod_migration.js
23	0	tests/solidity/migrations/36_functionsel_migration.js
25	0	tests/solidity/migrations/37_gas_opt_migration.js
15	0	tests/solidity/migrations/38_hacks_delegatecall_migration.js
27	0	tests/solidity/migrations/39_hashfunction_migration.js
7	0	tests/solidity/migrations/3_abi_encode_migration.js
21	0	tests/solidity/migrations/40_helloWorld_migration.js
11	0	tests/solidity/migrations/41_honeypot_migration.js
26	0	tests/solidity/migrations/42_immutable_migration.js
5	0	tests/solidity/migrations/43_import_migration.js
34	0	tests/solidity/migrations/44_inheritance_migration.js
21	0	tests/solidity/migrations/45_initial_migration.js
27	0	tests/solidity/migrations/46_interface_migration.js
30	0	tests/solidity/migrations/47_library_migration.js
16	0	tests/solidity/migrations/48_malicious_migration.js
27	0	tests/solidity/migrations/49_mapping_migration.js
14	0	tests/solidity/migrations/4_accessing_private_migration.js
21	0	tests/solidity/migrations/50_opCodes_migration.js
5	0	tests/solidity/migrations/51_payable_migration.js
11	0	tests/solidity/migrations/52_phising_migration.js
21	0	tests/solidity/migrations/53_primitive_data_types_migration.js
5	0	tests/solidity/migrations/54_reading_and_writing_migration.js
9	0	tests/solidity/migrations/55_re_entrancy_migration.js
9	0	tests/solidity/migrations/56_self_destruct_migration.js
23	0	tests/solidity/migrations/57_sendingether_migration.js
30	0	tests/solidity/migrations/58_shadowstate_migration.js
18	0	tests/solidity/migrations/59_source_of_randomness_migration.js
12	0	tests/solidity/migrations/5_arithmetic_overflow_migration.js
21	0	tests/solidity/migrations/60_structs_migration.js
25	0	tests/solidity/migrations/61_trycatch_migration.js
25	0	tests/solidity/migrations/62_tstorage_migration.js
24	0	tests/solidity/migrations/63_unchecked_math_migration.js
23	0	tests/solidity/migrations/64_valuetypes_migration.js
21	0	tests/solidity/migrations/65_Variables__migrsation_migration.js
24	0	tests/solidity/migrations/66_view_migration.js
27	0	tests/solidity/migrations/67_visibility_migration.js
6	0	tests/solidity/migrations/6_array_migration.js
23	0	tests/solidity/migrations/7_assembly_conditions_migration.js
22	0	tests/solidity/migrations/8_assembly_error_migration.js
22	0	tests/solidity/migrations/9_assembly_loop_migration.js
41	0	tests/solidity/test/Basic/abi_decode.js
46	0	tests/solidity/test/Basic/abi_encode.js
57	0	tests/solidity/test/Basic/array.js
58	0	tests/solidity/test/Basic/assembly_math.js
60	0	tests/solidity/test/Basic/assemblyconditions.js
59	0	tests/solidity/test/Basic/assemblyerror.js
40	0	tests/solidity/test/Basic/assemblyloop.js
34	0	tests/solidity/test/Basic/assemblyvariable.js
76	0	tests/solidity/test/Basic/bitwiseoperator.js
42	0	tests/solidity/test/Basic/call.js
42	0	tests/solidity/test/Basic/calling_other_contract.js
41	0	tests/solidity/test/Basic/callingparent.js
40	0	tests/solidity/test/Basic/constants.js
128	0	tests/solidity/test/Basic/constructor.js
52	0	tests/solidity/test/Basic/creating_contracts.js
40	0	tests/solidity/test/Basic/dataloc.js
50	0	tests/solidity/test/Basic/delegatecall.js
49	0	tests/solidity/test/Basic/enum.js
65	0	tests/solidity/test/Basic/error.js
33	0	tests/solidity/test/Basic/ether_and_wei.js
41	0	tests/solidity/test/Basic/events.js
74	0	tests/solidity/test/Basic/events_advanced.js
28	0	tests/solidity/test/Basic/fallback.js
56	0	tests/solidity/test/Basic/firstappp.js
14	0	tests/solidity/test/Basic/for_and_while_loop.js
81	0	tests/solidity/test/Basic/function.js
55	0	tests/solidity/test/Basic/functionmod.js
38	0	tests/solidity/test/Basic/functionsel.js
26	0	tests/solidity/test/Basic/gas_and_gasprice.js
38	0	tests/solidity/test/Basic/gasopt.js
62	0	tests/solidity/test/Basic/hashfunction.js
33	0	tests/solidity/test/Basic/helloWorld.js
38	0	tests/solidity/test/Basic/ifelse.js
38	0	tests/solidity/test/Basic/immutable.js
9	0	tests/solidity/test/Basic/import.js
64	0	tests/solidity/test/Basic/inheritance.js
43	0	tests/solidity/test/Basic/interface.js
58	0	tests/solidity/test/Basic/library.js
69	0	tests/solidity/test/Basic/mapping.js
74	0	tests/solidity/test/Basic/payable.js
58	0	tests/solidity/test/Basic/primitive_data_types.js
26	0	tests/solidity/test/Basic/reading_and_writing.js
69	0	tests/solidity/test/Basic/sendingether.js
41	0	tests/solidity/test/Basic/shadowstate.js
55	0	tests/solidity/test/Basic/structs.js
60	0	tests/solidity/test/Basic/trycatch.js
51	0	tests/solidity/test/Basic/tstorage.js
47	0	tests/solidity/test/Basic/uncheckedmath.js
34	0	tests/solidity/test/Basic/valuetypes.js
44	0	tests/solidity/test/Basic/variables.js
36	0	tests/solidity/test/Basic/view.js
61	0	tests/solidity/test/Basic/visibility.js
279	0	tests/solidity/test/EVM/evm_memory.js
349	0	tests/solidity/test/EVM/evm_storage.js
45	0	tests/solidity/test/Hacks/63_gas_rule.js
114	0	tests/solidity/test/Hacks/accessing_private.js
42	0	tests/solidity/test/Hacks/block_timestamp.js
37	0	tests/solidity/test/Hacks/bypass.js
34	0	tests/solidity/test/Hacks/honeypot.js
49	0	tests/solidity/test/Hacks/malicious.js
30	0	tests/solidity/test/Hacks/re_entrancy.js
34	0	tests/solidity/test/Hacks/source_of_randomness.js
24	26	tests/solidity/test/opCodes.js
5	0	tests/solidity/truffle-config.js
4	4	tests/state_test_util.go
0	1	tests/testdata
```
