#!/bin/bash
set -e

if [ "$1" != "--no-fetch" ]; then
    curl -H 'Cache-Control: no-cache' https://raw.githubusercontent.com/near/nearcore/refs/heads/master/chain/jsonrpc/openapi/openapi.json > openapi.json
fi
python3 progenitor_fixes.py --spec-fix
cargo progenitor -i openapi.json -o near-openapi -n near-openapi -v 0.0.0
echo "[workspace]" >> near-openapi/Cargo.toml
cd near-openapi && cargo fmt && cd ..
python3 progenitor_fixes.py --lib-fix
cd near-openapi-client && cargo fmt && cd ..
cd near-openapi-types && cargo fmt && cd ..
