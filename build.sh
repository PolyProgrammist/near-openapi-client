curl https://raw.githubusercontent.com/PolyProgrammist/nearcore/refs/heads/jsonschema-for-rpc/chain/jsonrpc/openapi/progenitor.json > openapi.json
python3 tx.py --spec-fix
cargo progenitor -i openapi.json -o near-openapi -n near-openapi -v 0.1.0
python3 tx.py --lib-fix
cd example && cargo run
