use near_openapi_client::types;

#[test]
fn test_global_contract_identifier_view_from_string() {
    // This is what the RPC returns for GlobalContractIdentifierView (an AccountId)
    let json = r#""mt_receiver_global.sandbox""#;

    let result: Result<types::GlobalContractIdentifierView, _> = serde_json::from_str(json);

    println!("{:?}", result);

    assert!(
        result.is_ok(),
        "Failed to deserialize GlobalContractIdentifierView from bare string: {:?}",
        result.err()
    );
}

#[test]
fn test_deterministic_state_init_action_view() {
    // This is the actual JSON returned by the RPC for a DeterministicStateInit action
    let json = r#"{"DeterministicStateInit":{"code":"mt_receiver_global.sandbox","data":{},"deposit":"0"}}"#;

    let result: Result<types::ActionView, _> = serde_json::from_str(json);

    println!("{:?}", result);

    assert!(
        result.is_ok(),
        "Failed to deserialize DeterministicStateInit ActionView: {:?}",
        result.err()
    );
}

#[test]
fn test_global_contract_identifier_view_from_crypto_hash() {
    // CryptoHash is a 32-byte base58 encoded string
    let json = r#""11111111111111111111111111111111""#;

    let result: Result<types::GlobalContractIdentifierView, _> = serde_json::from_str(json);

    assert!(
        result.is_ok(),
        "Failed to deserialize GlobalContractIdentifierView from CryptoHash string: {:?}",
        result.err()
    );
}
