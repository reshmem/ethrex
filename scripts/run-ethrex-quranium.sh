#!/usr/bin/env bash
set -euo pipefail
ARTIFACTS_DIR=./quranium-artifacts
P2P_ADDR="${P2P_ADDR:-0.0.0.0}"
BOOTNODES="enode://f5431cbddc90873452c208f69521dfefeba343c70cbb0182aec8622ee0b724e1d83165f23ee6c56bad2a076dc669aa1d3546849b2484801fb524ae33805ab223@87.71.32.69:40404,enode://6155df6acb9a98cf72953eb06da7b5707aa569950c2b0a03cb93a99b4116c9662a43fb97bf3db8fc27c70b087a5af77ebb8a75541efa4477739b3dcd00940486@35.224.16.114:40404,enode://656353d6f48a6f65bc94f2730e8c4af5f30e4f4f3af4bc7e78beb0a3041426fa793804d4613a4da48be28c679056d20b0d84bcfde44e162fac0ebb1913db722f@34.70.202.57:40404,enode://9c1bc64dcf5f890c684790f8c9f68683557aa052f75caf0a87fbfe59440a5c1c7bac3da4ca16a580887957b655ecfe8eaf8f250ca55bb7a35236e20b6857267c@172.166.248.107:40404,enode://262d27b2a10853f9ebca544eb864bab75d4aac73bbff801fdbc151c54ecab1fea44dfc10076f6fd05033ab52aec76e6ee554b8a8a3b5b68f54e5da90dddab13c@172.167.67.136:40404,enode://d6c4eea0a45d8aea095b44706f148a6d46cc7380582d891def0555816facbae6ab7d05436fedd3d4198bf880a15c417685a1cf85096d11b207c102fa4ae3b16e@4.250.225.225:40404"

ethrex \
  --network ${ARTIFACTS_DIR}/el_cl_genesis_data/genesis.json \
  --datadir ${ARTIFACTS_DIR}/data/execution-data-2 \
  --syncmode full \
  --http.addr 0.0.0.0 \
  --http.port 8545 \
  --ws.enabled \
  --ws.addr 0.0.0.0 \
  --ws.port 8546 \
  --authrpc.addr 0.0.0.0 \
  --authrpc.port 8551 \
  --authrpc.jwtsecret ${ARTIFACTS_DIR}/jwt_file/jwtsecret \
  --p2p.addr "${P2P_ADDR}" \
  --p2p.port 40404 \
  --discovery.port 40404 \
  --p2p.target-peers 15 \
  --bootnodes "${BOOTNODES}" \
  --builder.gas-limit 30000000 \
  --mempool.maxsize 10240 \
  --metrics \
  --metrics.addr 0.0.0.0 \
  --metrics.port 7070
