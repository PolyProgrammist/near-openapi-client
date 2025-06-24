use client::CryptoHash;
use client::Client;
use near_crypto::{InMemorySigner, Signer};
use near_openapi_client as client;
use near_primitives::transaction::{Action, Transaction, TransactionV0, TransferAction};
use std::error::Error;
use tokio::time::{sleep, Duration};

const NEAR_RPC_URL_LOCAL: &str = "http://127.0.0.1:3040";
const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";

#[tokio::test]
async fn test_openapi_client() -> Result<(), Box<dyn Error>> {
    let (signer, mut sandbox_node, client_local, client_remote) = prepare_sandbox().await.unwrap();
    let (sender_account_id, block_final_hash, base64_signed_tx, sent_tx_hash, executed_receipt_id, later_block_hash) =
        prepare_blockchain(&signer, client_local.clone()).await?;

    test_block(&client_local, block_final_hash.clone()).await?;
    test_status(&client_local).await?;

    test_broadcast_async(&client_local, base64_signed_tx.clone()).await?;
    test_broadcast_commit(&client_local, base64_signed_tx.clone()).await?;
    test_chunk(&client_local, block_final_hash.clone()).await?;
    test_gas_price_with_block(&client_local, block_final_hash.clone()).await?;
    test_gas_price_without_block(&client_local).await?;
    test_health(&client_local).await?;
    test_light_client_proof(
        &client_local,
        later_block_hash.clone(),
        sender_account_id.clone(),
        sent_tx_hash.clone(),
    )
    .await?;
    test_next_light_client_block(&client_local, block_final_hash.clone()).await?;
    test_network_info(&client_local).await?;
    test_send_tx(&client_local, base64_signed_tx.clone()).await?;
    test_status(&client_local).await?;
    test_validators(&client_local).await?;
    test_client_config(&client_local).await?;
    test_experimental_changes(
        &client_local,
        block_final_hash.clone(),
        sender_account_id.clone(),
    )
    .await?;
    test_experimental_changes_in_block(&client_local, block_final_hash.clone()).await?;
    test_experimental_congestion_level(&client_local, block_final_hash.clone()).await?;
    test_experimental_genesis_config(&client_local).await?;
    test_experimental_light_client_proof(
        &client_local,
        later_block_hash.clone(),
        sender_account_id.clone(),
        sent_tx_hash.clone(),
    )
    .await?;
    test_experimental_light_client_block(&client_local, block_final_hash.clone()).await?;
    test_experimental_protocol_config(&client_local, block_final_hash.clone()).await?;
    test_experimental_receipt(&client_local, executed_receipt_id.clone()).await?;
    test_experimental_tx_status(
        &client_local,
        sent_tx_hash.clone(),
        sender_account_id.clone(),
    )
    .await?;
    test_experimental_validators_ordered(&client_local).await?;
    test_experimental_maintenance_windows(&client_remote, sender_account_id.clone()).await?;
    test_experimental_split_storage_info(&client_local).await?;
    test_query_account(&client_local, sender_account_id.clone()).await?;
    test_function_call(&client_local, sender_account_id.clone()).await?;

    sandbox_node.kill().await?;

    Ok(())
}

async fn test_block(client: &Client, block_hash: CryptoHash) -> Result<(), Box<dyn Error>> {
    let payload_block = client::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForBlockMethod::Block,
        params: client::RpcBlockRequest::BlockId({
            client::BlockId::Variant1(block_hash.clone())
        }),
    };

    let block: client::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client.block(&payload_block).await?.into_inner();
    assert!(matches!(
        block,
        client::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for block: {:#?}", block);

    Ok(())
}

async fn test_broadcast_async(
    client: &Client,
    base64_signed_tx: String,
) -> Result<(), Box<dyn Error>> {
    let payload_broadcast_async = client::JsonRpcRequestForBroadcastTxAsync {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForBroadcastTxAsyncMethod::BroadcastTxAsync,
        params: client::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::TxExecutionStatus::Executed,
        },
    };

    let broadcast_async: client::JsonRpcResponseForCryptoHashAndRpcError = client
        .broadcast_tx_async(&payload_broadcast_async)
        .await?
        .into_inner();
    assert!(matches!(
        broadcast_async,
        client::JsonRpcResponseForCryptoHashAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for broadcast_async: {:#?}", broadcast_async);

    Ok(())
}

async fn test_broadcast_commit(
    client: &Client,
    base64_signed_tx: String,
) -> Result<(), Box<dyn Error>> {
    let payload_broadcast_commit = client::JsonRpcRequestForBroadcastTxCommit {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForBroadcastTxCommitMethod::BroadcastTxCommit,
        params: client::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::TxExecutionStatus::Executed,
        },
    };

    let broadcast_commit: client::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client
            .broadcast_tx_commit(&payload_broadcast_commit)
            .await?
            .into_inner();
    assert!(matches!(
        broadcast_commit,
        client::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for broadcast_commit: {:#?}", broadcast_commit);

    Ok(())
}

async fn test_chunk(client: &Client, block_hash: CryptoHash) -> Result<(), Box<dyn Error>> {
    let payload_chunk = client::JsonRpcRequestForChunk {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForChunkMethod::Chunk,
        params: client::RpcChunkRequest::Variant0 {
            block_id: client::BlockId::Variant1(block_hash.clone()),
            shard_id: client::ShardId(0),
        },
    };

    let chunk: client::JsonRpcResponseForRpcChunkResponseAndRpcError =
        client.chunk(&payload_chunk).await?.into_inner();
    assert!(matches!(
        chunk,
        client::JsonRpcResponseForRpcChunkResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for chunk: {:#?}", chunk);

    Ok(())
}

async fn test_gas_price_with_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_gas_price = client::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::RpcGasPriceRequest {
            block_id: Some(client::BlockId::Variant1(block_hash.clone())),
        },
    };

    let gas_price: client::JsonRpcResponseForRpcGasPriceResponseAndRpcError =
        client.gas_price(&payload_gas_price).await?.into_inner();
    assert!(matches!(
        gas_price,
        client::JsonRpcResponseForRpcGasPriceResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for gas_price with block: {:#?}", gas_price);

    Ok(())
}

async fn test_gas_price_without_block(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_gas_price = client::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::RpcGasPriceRequest { block_id: None },
    };

    let gas_price: client::JsonRpcResponseForRpcGasPriceResponseAndRpcError =
        client.gas_price(&payload_gas_price).await?.into_inner();
    assert!(matches!(
        gas_price,
        client::JsonRpcResponseForRpcGasPriceResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for gas_price without block: {:#?}", gas_price);

    Ok(())
}

async fn test_health(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_health = client::JsonRpcRequestForHealth {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForHealthMethod::Health,
        params: client::RpcHealthRequest(serde_json::Map::new()),
    };

    let health: client::JsonRpcResponseForNullableRpcHealthResponseAndRpcError =
        client.health(&payload_health).await?.into_inner();
    assert!(matches!(
        health,
        client::JsonRpcResponseForNullableRpcHealthResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for health: {:#?}", health);

    Ok(())
}

async fn test_light_client_proof(
    client: &Client,
    block_hash: CryptoHash,
    sender_account_id: client::AccountId,
    sent_tx_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_light_client_proof = client::JsonRpcRequestForLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForLightClientProofMethod::LightClientProof,
        params: client::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::TypeTransactionOrReceiptId::Transaction,
        },
    };

    let light_client_proof: client::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client.light_client_proof(&payload_light_client_proof).await?.into_inner();
    println!("response for light_client_proof: {:#?}", light_client_proof);

    assert!(matches!(light_client_proof, client::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError::Variant0 { result: _, .. }));
    Ok(())
}

async fn test_next_light_client_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_next_light_client_block = client::JsonRpcRequestForNextLightClientBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForNextLightClientBlockMethod::NextLightClientBlock,
        params: client::RpcLightClientNextBlockRequest {
            last_block_hash: block_hash.clone(),
        },
    };

    let next_light_client_block: client::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client.next_light_client_block(&payload_next_light_client_block).await?.into_inner();
    assert!(matches!(
        next_light_client_block,
        client::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!(
        "response for next_light_client_block: {:#?}",
        next_light_client_block
    );

    Ok(())
}

async fn test_network_info(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_network_info = client::JsonRpcRequestForNetworkInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForNetworkInfoMethod::NetworkInfo,
        params: client::RpcNetworkInfoRequest(serde_json::Map::new()),
    };

    let network_info: client::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client
        .network_info(&payload_network_info)
        .await?
        .into_inner();
    assert!(matches!(
        network_info,
        client::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for network_info: {:#?}", network_info);

    Ok(())
}

async fn test_send_tx(client: &Client, base64_signed_tx: String) -> Result<(), Box<dyn Error>> {
    let payload_send_tx = client::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::TxExecutionStatus::Executed,
        },
    };

    let send_tx: client::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client.send_tx(&payload_send_tx).await?.into_inner();
    assert!(matches!(
        send_tx,
        client::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for send_tx: {:#?}", send_tx);

    Ok(())
}

async fn test_status(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_status = client::JsonRpcRequestForStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForStatusMethod::Status,
        params: client::RpcStatusRequest(serde_json::Map::new()),
    };

    let status: client::JsonRpcResponseForRpcStatusResponseAndRpcError =
        client.status(&payload_status).await?.into_inner();
    assert!(matches!(
        status,
        client::JsonRpcResponseForRpcStatusResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for status: {:#?}", status);

    Ok(())
}

async fn test_validators(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_validators = client::JsonRpcRequestForValidators {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForValidatorsMethod::Validators,
        params: client::RpcValidatorRequest::Latest,
    };

    let validators: client::JsonRpcResponseForRpcValidatorResponseAndRpcError =
        client.validators(&payload_validators).await?.into_inner();
    assert!(matches!(
        validators,
        client::JsonRpcResponseForRpcValidatorResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for validators: {:#?}", validators);

    Ok(())
}

async fn test_client_config(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_client_config = client::JsonRpcRequestForClientConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForClientConfigMethod::ClientConfig,
        params: client::RpcClientConfigRequest(serde_json::Map::new()),
    };

    let client_config: client::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client
        .client_config(&payload_client_config)
        .await?
        .into_inner();
    assert!(matches!(
        client_config,
        client::JsonRpcResponseForRpcClientConfigResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for client_config: {:#?}", client_config);

    Ok(())
}

async fn test_experimental_changes(
    client: &Client,
    block_hash: CryptoHash,
    sender_account_id: client::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_experimental_changes = client::JsonRpcRequestForExperimentalChanges {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalChangesMethod::ExperimentalChanges,
        params: client::RpcStateChangesInBlockByTypeRequest::Variant0 {
            changes_type: client::RpcStateChangesInBlockByTypeRequestVariant0ChangesType::AccountChanges,
            account_ids: vec![sender_account_id],
            block_id: client::BlockId::Variant1(block_hash.clone()),
        }
    };

    let experimental_changes: client::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError = client.experimental_changes(&payload_experimental_changes).await?.into_inner();
    assert!(matches!(
        experimental_changes,
        client::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    if let client::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError::Variant0 {
        result,
        ..
    } = &experimental_changes
    {
        assert!(!result.changes.is_empty(), "Expected non-empty changes");
    }

    println!(
        "response for experimental_changes: {:#?}",
        experimental_changes
    );

    Ok(())
}

async fn test_experimental_changes_in_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_experimental_changes_in_block = client::JsonRpcRequestForExperimentalChangesInBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalChangesInBlockMethod::ExperimentalChangesInBlock,
        params: client::RpcStateChangesInBlockRequest::BlockId(client::BlockId::Variant1(block_hash.clone()))
    };

    let experimental_changes_in_block: client::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError = client.experimental_changes_in_block(&payload_experimental_changes_in_block).await?.into_inner();
    assert!(matches!(experimental_changes_in_block, client::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError::Variant0 { result: _, .. }));
    if let client::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError::Variant0 { result, .. } = &experimental_changes_in_block {
        assert!(!result.changes.is_empty(), "Expected non-empty changes in block");
    }

    println!(
        "response for experimental_changes_in_block: {:#?}",
        experimental_changes_in_block
    );

    Ok(())
}

async fn test_experimental_congestion_level(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_congestion_level = client::JsonRpcRequestForExperimentalCongestionLevel {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalCongestionLevelMethod::ExperimentalCongestionLevel,
        params: client::RpcCongestionLevelRequest::Variant0 {
            block_id: client::BlockId::Variant1(block_hash.clone()),
            shard_id: client::ShardId(0)
        }
    };

    let congestion_level: client::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError =
        client
            .experimental_congestion_level(&payload_congestion_level)
            .await?
            .into_inner();
    assert!(matches!(
        congestion_level,
        client::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for congestion_level: {:#?}", congestion_level);

    Ok(())
}

async fn test_experimental_genesis_config(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_genesis_config = client::JsonRpcRequestForExperimentalGenesisConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalGenesisConfigMethod::ExperimentalGenesisConfig,
        params: client::GenesisConfigRequest(serde_json::Map::new())
    };

    let genesis_config: client::JsonRpcResponseForGenesisConfigAndRpcError = client
        .experimental_genesis_config(&payload_genesis_config)
        .await?
        .into_inner();
    assert!(matches!(
        genesis_config,
        client::JsonRpcResponseForGenesisConfigAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for genesis_config: {:#?}", genesis_config);

    Ok(())
}

async fn test_experimental_light_client_proof(
    client: &Client,
    block_hash: CryptoHash,
    sender_account_id: client::AccountId,
    sent_tx_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_exp_light_client_proof = client::JsonRpcRequestForExperimentalLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalLightClientProofMethod::ExperimentalLightClientProof,
        params: client::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let exp_light_client_proof: client::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client.experimental_light_client_proof(&payload_exp_light_client_proof).await?.into_inner();
    assert!(matches!(exp_light_client_proof, client::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError::Variant0 { result: _, .. }));

    println!(
        "response for exp_light_client_proof: {:#?}",
        exp_light_client_proof
    );

    Ok(())
}

async fn test_experimental_light_client_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_exp_light_client_block = client::JsonRpcRequestForExperimentalLightClientBlockProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalLightClientBlockProofMethod::ExperimentalLightClientBlockProof,
        params: client::RpcLightClientBlockProofRequest {
            block_hash: block_hash.clone(),
            light_client_head: block_hash.clone(),
        }
    };

    let exp_light_client_block: client::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError = client.experimental_light_client_block_proof(&payload_exp_light_client_block).await?.into_inner();
    assert!(matches!(
        exp_light_client_block,
        client::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!(
        "response for exp_light_client_block: {:#?}",
        exp_light_client_block
    );

    Ok(())
}

async fn test_experimental_protocol_config(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_protocol_config = client::JsonRpcRequestForExperimentalProtocolConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalProtocolConfigMethod::ExperimentalProtocolConfig,
        params: client::RpcProtocolConfigRequest::BlockId(client::BlockId::Variant1(block_hash.clone()))
    };

    let protocol_config: client::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError =
        client
            .experimental_protocol_config(&payload_protocol_config)
            .await?
            .into_inner();
    assert!(matches!(
        protocol_config,
        client::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for protocol_config: {:#?}", protocol_config);

    Ok(())
}

async fn test_experimental_receipt(
    client: &Client,
    executed_receipt_id: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_receipt = client::JsonRpcRequestForExperimentalReceipt {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalReceiptMethod::ExperimentalReceipt,
        params: client::RpcReceiptRequest {
            receipt_id: executed_receipt_id,
        },
    };

    let receipt: client::JsonRpcResponseForRpcReceiptResponseAndRpcError = client
        .experimental_receipt(&payload_receipt)
        .await?
        .into_inner();
    assert!(matches!(
        receipt,
        client::JsonRpcResponseForRpcReceiptResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for receipt: {:#?}", receipt);

    Ok(())
}

async fn test_experimental_tx_status(
    client: &Client,
    sent_tx_hash: CryptoHash,
    sender_account_id: client::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_exp_tx_status = client::JsonRpcRequestForExperimentalTxStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalTxStatusMethod::ExperimentalTxStatus,
        params: client::RpcTransactionStatusRequest::Variant1 {
            tx_hash: sent_tx_hash.clone(),
            sender_account_id: sender_account_id.clone(),
            wait_until: client::TxExecutionStatus::None,
        },
    };

    let exp_tx_status: client::JsonRpcResponseForRpcTransactionResponseAndRpcError = client
        .experimental_tx_status(&payload_exp_tx_status)
        .await?
        .into_inner();
    assert!(matches!(
        exp_tx_status,
        client::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for exp_tx_status: {:#?}", exp_tx_status);

    Ok(())
}

async fn test_experimental_validators_ordered(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_exp_validators = client::JsonRpcRequestForExperimentalValidatorsOrdered {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalValidatorsOrderedMethod::ExperimentalValidatorsOrdered,
        params: client::RpcValidatorsOrderedRequest {
            block_id: None
        }
    };

    let exp_validators: client::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError =
        client
            .experimental_validators_ordered(&payload_exp_validators)
            .await?
            .into_inner();
    assert!(matches!(
        exp_validators,
        client::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for exp_validators: {:#?}", exp_validators);

    Ok(())
}

async fn test_experimental_maintenance_windows(
    client_remote: &Client,
    sender_account_id: client::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_maintenance_windows = client::JsonRpcRequestForExperimentalMaintenanceWindows {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalMaintenanceWindowsMethod::ExperimentalMaintenanceWindows,
        params: client::RpcMaintenanceWindowsRequest {
            account_id: sender_account_id.clone(),
        }
    };

    let maintenance_windows: client::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError =
        client_remote
            .experimental_maintenance_windows(&payload_maintenance_windows)
            .await?
            .into_inner();
    assert!(matches!(
        maintenance_windows,
        client::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!(
        "response for maintenance_windows: {:#?}",
        maintenance_windows
    );

    Ok(())
}

async fn test_experimental_split_storage_info(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_split_storage = client::JsonRpcRequestForExperimentalSplitStorageInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForExperimentalSplitStorageInfoMethod::ExperimentalSplitStorageInfo,
        params: client::RpcSplitStorageInfoRequest(serde_json::Map::new())
    };

    let split_storage_info: client::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError = client.experimental_split_storage_info(&payload_split_storage).await?.into_inner();
    assert!(matches!(
        split_storage_info,
        client::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for split_storage_info: {:#?}", split_storage_info);

    Ok(())
}

async fn test_query_account(
    client: &Client,
    sender_account_id: client::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_query_account = client::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForQueryMethod::Query,
        params: client::RpcQueryRequest::Variant8 {
            account_id: sender_account_id,
            request_type: client::RpcQueryRequestVariant8RequestType::ViewAccount,
            finality: client::Finality::Final,
        },
    };

    let query_account: client::JsonRpcResponseForRpcQueryResponseAndRpcError =
        client.query(&payload_query_account).await?.into_inner();
    assert!(matches!(
        query_account,
        client::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for query_account: {:#?}", query_account);

    Ok(())
}

async fn test_function_call(
    client: &Client,
    sender_account_id: client::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_function_call = client::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForQueryMethod::Query,
        params: client::RpcQueryRequest::Variant13 {
            account_id: sender_account_id.clone(),
            request_type: client::RpcQueryRequestVariant13RequestType::CallFunction,
            method_name: "get_greeting".to_string(),
            args_base64: "".to_string(),
            finality: client::Finality::Final,
        },
    };

    let function_call: client::JsonRpcResponseForRpcQueryResponseAndRpcError =
        client.query(&payload_function_call).await?.into_inner();
    assert!(matches!(
        function_call,
        client::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 { result: _, .. }
    ));
    if let client::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 {
        result, ..
    } = &function_call
    {
        if let client::RpcQueryResponse::Variant3 { result, .. } = result {
            assert_eq!(
                result.len(),
                6,
                "Expected function call response size to be 6 bytes"
            );
        } else {
            return Err("Unexpected response format for function call".into());
        }
    }

    println!("response for function_call: {:#?}", function_call);

    Ok(())
}

async fn prepare_blockchain(
    signer: &Signer,
    client_local: Client,
) -> Result<
    (
        client::AccountId,
        CryptoHash,
        String,
        CryptoHash,
        CryptoHash,
        CryptoHash,
    ),
    Box<dyn Error>,
> {
    let sender_account_id: client::AccountId = "test.near".parse().unwrap();

    let payload_query_access_key = client::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForQueryMethod::Query,
        params: client::RpcQueryRequest::Variant11 {
            account_id: sender_account_id.clone(),
            public_key: client::PublicKey(signer.public_key().to_string()),
            request_type: client::RpcQueryRequestVariant11RequestType::ViewAccessKey,
            finality: client::Finality::Final,
        },
    };

    let access_key: client::JsonRpcResponseForRpcQueryResponseAndRpcError = client_local
        .query(&payload_query_access_key)
        .await?
        .into_inner();
    println!("response for access_key: {:#?}", access_key);

    let access_key_block_hash: CryptoHash;
    let access_key_nonce: u64;
    if let client::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 {
        result, ..
    } = access_key
    {
        if let client::RpcQueryResponse::Variant4 {
            block_hash, nonce, ..
        } = result
        {
            access_key_block_hash = block_hash.to_string().parse().unwrap();
            access_key_nonce = nonce;
        } else {
            return Err("couldn't get access key".into());
        }
    } else {
        return Err("access key is not in expected format".into());
    }

    let code = std::fs::read("contract_rs.wasm")?;
    let deploy_contract_action = near_primitives::transaction::Action::DeployContract(
        near_primitives::transaction::DeployContractAction { code },
    );
    let function_call_action = near_primitives::transaction::Action::FunctionCall(Box::new(
        near_primitives::transaction::FunctionCallAction {
            method_name: "set_greeting".to_string(),
            args: serde_json::to_vec(&serde_json::json!({
                "greeting": "hola"
            }))?,
            gas: 300_000_000_000_000,
            deposit: 0,
        },
    ));

    let transfer_amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yocto
    let tx = Transaction::V0(TransactionV0 {
        signer_id: sender_account_id.clone(),
        public_key: signer.public_key(),
        nonce: access_key_nonce + 1,
        block_hash: access_key_block_hash.to_string().parse().unwrap(),
        receiver_id: sender_account_id.clone(),
        actions: vec![
            Action::Transfer(TransferAction {
                deposit: transfer_amount,
            }),
            deploy_contract_action,
            function_call_action,
        ],
    });
    let signed_tx = tx.sign(&signer);
    let base64_signed_tx = near_primitives::serialize::to_base64(&borsh::to_vec(&signed_tx)?);

    let payload_send_tx = client::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::TxExecutionStatus::Executed,
        },
    };

    let send_tx: client::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client_local.send_tx(&payload_send_tx).await?.into_inner();
    println!("response for send_tx: {:#?}", send_tx);

    let sent_tx_hash: CryptoHash;
    let executed_receipt_id: CryptoHash;
    if let client::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
        result,
        ..
    } = send_tx
    {
        if let client::RpcTransactionResponse::Variant1 {
            receipts_outcome,
            transaction,
            ..
        } = result
        {
            sent_tx_hash = transaction.hash;
            executed_receipt_id = receipts_outcome[1].id.clone();
        } else {
            return Err("couldn't send transaction".into());
        }
    } else {
        return Err("couldn't get transaction info".into());
    }

    let payload_block_final = client::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::JsonRpcRequestForBlockMethod::Block,
        params: client::RpcBlockRequest::Finality(client::Finality::Final),
    };

    let block_final: client::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client_local.block(&payload_block_final).await?.into_inner();
    println!("response for block_final: {:#?}", block_final);
    let block_final_hash: CryptoHash;
    if let client::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 {
        result, ..
    } = block_final
    {
        block_final_hash = result.header.hash;
    } else {
        return Err("final block is not in expected format".into());
    }

    sleep(Duration::from_secs(2)).await;

    let later_block: client::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client_local.block(&payload_block_final).await?.into_inner();
    let later_block_hash: CryptoHash;
    if let client::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 {
        result, ..
    } = later_block
    {
        later_block_hash = result.header.hash;
    } else {
        return Err("final block is not in expected format".into());
    }

    Ok((
        sender_account_id,
        block_final_hash,
        base64_signed_tx,
        sent_tx_hash,
        executed_receipt_id,
        later_block_hash,
    ))
}

async fn prepare_sandbox() -> Result<(Signer, tokio::process::Child, Client, Client), Box<dyn Error>>
{
    let mut home_dir = std::env::temp_dir();
    home_dir.push("test_node");

    let rpc_port: u16 = 3040;
    let net_port: u16 = 3031;

    let version = "master/b57299a7a8558d4a6813f51c2512c057289e70e2";

    near_sandbox_utils::init_with_version(&home_dir, version)?
        .wait_with_output()
        .await
        .unwrap();

    let child = near_sandbox_utils::run_with_version(&home_dir, rpc_port, net_port, version)?;

    sleep(Duration::from_secs(3)).await;

    let mut validator_key = home_dir.clone();
    validator_key.push("validator_key.json");
    let signer = InMemorySigner::from_file(&validator_key)?;

    let client_local = Client::new(NEAR_RPC_URL_LOCAL);
    let client_remote = Client::new(NEAR_RPC_URL_REMOTE);

    Ok((signer, child, client_local, client_remote))
}
