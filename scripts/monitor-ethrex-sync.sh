#!/usr/bin/env bash
set -euo pipefail

RPC_URL="${RPC_URL:-http://127.0.0.1:8545}"
INTERVAL_SEC="${INTERVAL_SEC:-5}"
CURL_TIMEOUT="${CURL_TIMEOUT:-5}"
CONNECT_TIMEOUT="${CONNECT_TIMEOUT:-2}"
RETRIES="${RETRIES:-2}"
PYTHON_BIN="${PYTHON_BIN:-}"

if [[ -z "${PYTHON_BIN}" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    PYTHON_BIN="python3"
  elif command -v python >/dev/null 2>&1; then
    PYTHON_BIN="python"
  else
    echo "ERROR: python3 or python is required (set PYTHON_BIN to override)." >&2
    exit 1
  fi
fi

rpc() {
  local method="$1"
  local params="${2:-[]}"
  curl -sS --max-time "${CURL_TIMEOUT}" --connect-timeout "${CONNECT_TIMEOUT}" --retry "${RETRIES}" \
    "${RPC_URL}" \
    -H "Content-Type: application/json" \
    --data "{\"jsonrpc\":\"2.0\",\"method\":\"${method}\",\"params\":${params},\"id\":1}"
}

json_result() {
  "${PYTHON_BIN}" -c 'import json,sys; print(json.load(sys.stdin)["result"])'
}

hex_to_dec() {
  "${PYTHON_BIN}" -c 'import sys; print(int(sys.argv[1], 16))' "$1"
}

sync_status_line() {
  "${PYTHON_BIN}" - <<'PY'
import json, sys
data = sys.stdin.read().strip()
if not data:
    print("syncing=error reason=empty", end="")
    sys.exit(0)
try:
    res = json.loads(data).get("result")
except Exception:
    print("syncing=error reason=parse", end="")
    sys.exit(0)
if res is False:
    print("syncing=false", end="")
else:
    if not isinstance(res, dict):
        print("syncing=error reason=type", end="")
        sys.exit(0)
    current = int(res.get("currentBlock", "0x0"), 16)
    highest = int(res.get("highestBlock", "0x0"), 16)
    pulled = res.get("pulledStates")
    known = res.get("knownStates")
    parts = [f"syncing=true current={current}", f"highest={highest}"]
    if pulled is not None:
        parts.append(f"pulledStates={int(pulled,16)}")
    if known is not None:
        parts.append(f"knownStates={int(known,16)}")
    print(" ".join(parts), end="")
PY
}

echo "Monitoring ethrex at ${RPC_URL} every ${INTERVAL_SEC}s (Ctrl+C to stop)"

while true; do
  ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  chain_id_hex="$(rpc eth_chainId | json_result || true)"
  block_hex="$(rpc eth_blockNumber | json_result || true)"
  peers_hex="$(rpc net_peerCount | json_result || true)"
  set +e
  sync_json="$(
    curl -sS --max-time "${CURL_TIMEOUT}" --connect-timeout "${CONNECT_TIMEOUT}" --retry "${RETRIES}" \
      "${RPC_URL}" \
      -H "Content-Type: application/json" \
      --data "{\"jsonrpc\":\"2.0\",\"method\":\"eth_syncing\",\"params\":[],\"id\":1}"
  )"
  sync_rc="$?"
  set -e

  chain_id_dec="$(hex_to_dec "${chain_id_hex:-0x0}")"
  block_dec="$(hex_to_dec "${block_hex:-0x0}")"
  peers_dec="$(hex_to_dec "${peers_hex:-0x0}")"

  printf "%s chain_id=%s block=%s peers=%s " "${ts}" "${chain_id_dec}" "${block_dec}" "${peers_dec}"
  if [[ -z "${sync_json}" ]]; then
    printf "syncing=error(rc=%s)\n" "${sync_rc}"
  else
    printf "%s\n" "$(printf "%s" "${sync_json}" | sync_status_line)"
  fi
  sleep "${INTERVAL_SEC}"
done
