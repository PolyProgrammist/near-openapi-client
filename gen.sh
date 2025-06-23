#!/bin/bash
set -e
curl https://raw.githubusercontent.com/near/nearcore/refs/heads/master/chain/jsonrpc/openapi/progenitor.json > openapi.json
python3 tx.py --spec-fix
cargo progenitor -i openapi.json -o near-openapi -n near-openapi -v 0.1.1
echo "[workspace]" >> near-openapi/Cargo.toml
cd near-openapi && cargo fmt && cd ..
python3 tx.py --lib-fix