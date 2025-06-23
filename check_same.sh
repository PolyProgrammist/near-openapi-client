#!/bin/bash
set -e

repo_client_checksum=$(md5sum near-openapi-client/Cargo.toml near-openapi-client/src/lib.rs | md5sum | awk '{print $1}')
repo_types_checksum=$(md5sum near-openapi-types/Cargo.toml near-openapi-types/src/lib.rs | md5sum | awk '{print $1}')

rm -rf near-openapi-client near-openapi-types
./gen.sh

generated_client_checksum=$(md5sum near-openapi-client/Cargo.toml near-openapi-client/src/lib.rs | md5sum | awk '{print $1}')
generated_types_checksum=$(md5sum near-openapi-types/Cargo.toml near-openapi-types/src/lib.rs | md5sum | awk '{print $1}')

if [ "$repo_client_checksum" != "$generated_client_checksum" ] || [ "$repo_types_checksum" != "$generated_types_checksum" ]; then
    echo "The crates are not up to date. Please run 'cargo progenitor' to update it."
    exit 1
else
    echo "The crates are up to date."
fi