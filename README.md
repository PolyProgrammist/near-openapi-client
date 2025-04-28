# near-openapi-clientс


How to generate & use:
```
curl https://raw.githubusercontent.com/PolyProgrammist/nearcore/refs/heads/jsonschema-for-rpc/chain/jsonrpc/openapi/progenitor.json > openapi.json
python3 tx.py --spec-fix
cargo install cargo-progenitor
cargo progenitor -i openapi.json -o client -n client -v 0.1.0
python3 tx.py --lib-fix
cd example && cargo run
```