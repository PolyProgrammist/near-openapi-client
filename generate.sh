#!/bin/bash
set -e

repo_client_checksum=$(md5sum near-openapi-client/Cargo.toml near-openapi-client/src/lib.rs | md5sum | awk '{print $1}')
repo_types_checksum=$(md5sum near-openapi-types/Cargo.toml near-openapi-types/src/lib.rs | md5sum | awk '{print $1}')

curl https://raw.githubusercontent.com/near/nearcore/refs/heads/master/chain/jsonrpc/openapi/progenitor.json > openapi.json
python3 progenitor_fixes.py --spec-fix
cargo progenitor -i openapi.json -o near-openapi -n near-openapi -v 0.0.0
echo "[workspace]" >> near-openapi/Cargo.toml
cd near-openapi && cargo fmt && cd ..
python3 progenitor_fixes.py --lib-fix

generated_client_checksum=$(md5sum near-openapi-client/Cargo.toml near-openapi-client/src/lib.rs | md5sum | awk '{print $1}')
generated_types_checksum=$(md5sum near-openapi-types/Cargo.toml near-openapi-types/src/lib.rs | md5sum | awk '{print $1}')

if [ "$repo_client_checksum" != "$generated_client_checksum" ] || [ "$repo_types_checksum" != "$generated_types_checksum" ]; then
    echo "The crates are not up to date. Please run './generate.sh' to update it."
    exit 1
else
    echo "The crates are up to date."
fi