#!/usr/bin/env bash
set -euo pipefail

RPC_URL="${RPC_URL:-http://127.0.0.1:8545}"
PUBLIC_IP="${PUBLIC_IP:-}"

call() {
  local method="$1"
  local params="${2:-[]}"
  curl -sS "${RPC_URL}" \
    -H "Content-Type: application/json" \
    --data "{\"jsonrpc\":\"2.0\",\"method\":\"${method}\",\"params\":${params},\"id\":1}"
}

echo "RPC_URL=${RPC_URL}"
echo
echo "admin_nodeInfo"
call "admin_nodeInfo"
echo
echo "admin_peers"
call "admin_peers"
echo
echo "net_version"
call "net_version"
echo
echo "eth_chainId"
call "eth_chainId"
echo
echo "eth_blockNumber"
call "eth_blockNumber"
echo
echo "eth_getBlockByNumber(0x0)"
call "eth_getBlockByNumber" "[\"0x0\", false]"

if [[ -n "${PUBLIC_IP}" ]]; then
  echo
  echo "NOTE: replace 127.0.0.1 in enode with PUBLIC_IP=${PUBLIC_IP} for bootnodes."
fi
