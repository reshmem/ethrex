# Quranium vs Upstream geth v1.15.0 — Component Diff Index

## Scope
- Baseline: `refs/tags/v1.15.0`
- Target: `HEAD`
- This index groups diffs by subsystem to support porting into reth/ethrex.

## Crypto / SLHDSA

### Status (A/M/D)
```
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
```

### Line Counts (Added/Deleted)
```
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
```

## Core Types & Signing

### Status (A/M/D)
```
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
```

### Line Counts (Added/Deleted)
```
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
```

## Core VM

### Status (A/M/D)
```
M	core/vm/contracts.go
M	core/vm/contracts_test.go
```

### Line Counts (Added/Deleted)
```
27	34	core/vm/contracts.go
1	1	core/vm/contracts_test.go
```

## Accounts / Keystore / Signer

### Status (A/M/D)
```
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
```

### Line Counts (Added/Deleted)
```
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
```

## Signer

### Status (A/M/D)
```
M	signer/core/api_test.go
M	signer/core/apitypes/types_test.go
M	signer/core/signed_data_test.go
M	signer/core/uiapi.go
```

### Line Counts (Added/Deleted)
```
4	3	signer/core/api_test.go
2	2	signer/core/apitypes/types_test.go
5	9	signer/core/signed_data_test.go
2	2	signer/core/uiapi.go
```

## P2P / Discovery / RLPx

### Status (A/M/D)
```
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
```

### Line Counts (Added/Deleted)
```
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
```

## Consensus (Clique)

### Status (A/M/D)
```
M	consensus/clique/clique.go
M	consensus/clique/clique_test.go
M	consensus/clique/snapshot_test.go
```

### Line Counts (Added/Deleted)
```
1	1	consensus/clique/clique.go
6	4	consensus/clique/clique_test.go
3	3	consensus/clique/snapshot_test.go
```

## Node / Params / RPC

### Status (A/M/D)
```
```

### Line Counts (Added/Deleted)
```
```

## Eth Protocol / Ethclient / GraphQL / API

### Status (A/M/D)
```
```

### Line Counts (Added/Deleted)
```
```

## Cmd Tools

### Status (A/M/D)
```
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
```

### Line Counts (Added/Deleted)
```
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
```

## Tests / Test Config / CI

### Status (A/M/D)
```
```

### Line Counts (Added/Deleted)
```
```

