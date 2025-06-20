curl https://raw.githubusercontent.com/near/nearcore/refs/heads/master/chain/jsonrpc/openapi/progenitor.json > openapi.json
python3 tx.py --spec-fix
cargo progenitor -i openapi.json -o near-openapi -n near-openapi -v 0.1.0
cd near-openapi && cargo fmt && cd ..
python3 tx.py --lib-fix
cd example && cargo test -- --nocapture
