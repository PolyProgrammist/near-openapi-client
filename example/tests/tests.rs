use client::types::CryptoHash;
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
    let payload_block = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::BlockId({
            client::types::BlockId::CryptoHash(block_hash.clone())
        }),
    };

    let block: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client.block(&payload_block).await?.into_inner();
    assert!(matches!(
        block,
        client::types::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for block: {:#?}", block);

    Ok(())
}

async fn test_broadcast_async(
    client: &Client,
    base64_signed_tx: String,
) -> Result<(), Box<dyn Error>> {
    let payload_broadcast_async = client::types::JsonRpcRequestForBroadcastTxAsync {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBroadcastTxAsyncMethod::BroadcastTxAsync,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::types::TxExecutionStatus::Executed,
        },
    };

    let broadcast_async: client::types::JsonRpcResponseForCryptoHashAndRpcError = client
        .broadcast_tx_async(&payload_broadcast_async)
        .await?
        .into_inner();
    assert!(matches!(
        broadcast_async,
        client::types::JsonRpcResponseForCryptoHashAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for broadcast_async: {:#?}", broadcast_async);

    Ok(())
}

async fn test_broadcast_commit(
    client: &Client,
    base64_signed_tx: String,
) -> Result<(), Box<dyn Error>> {
    let payload_broadcast_commit = client::types::JsonRpcRequestForBroadcastTxCommit {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBroadcastTxCommitMethod::BroadcastTxCommit,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::types::TxExecutionStatus::Executed,
        },
    };

    let broadcast_commit: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client
            .broadcast_tx_commit(&payload_broadcast_commit)
            .await?
            .into_inner();
    assert!(matches!(
        broadcast_commit,
        client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for broadcast_commit: {:#?}", broadcast_commit);

    Ok(())
}

async fn test_chunk(client: &Client, block_hash: CryptoHash) -> Result<(), Box<dyn Error>> {
    let payload_chunk = client::types::JsonRpcRequestForChunk {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForChunkMethod::Chunk,
        params: client::types::RpcChunkRequest::BlockShardId {
            block_id: client::types::BlockId::CryptoHash(block_hash.clone()),
            shard_id: client::types::ShardId(0),
        },
    };

    let chunk: client::types::JsonRpcResponseForRpcChunkResponseAndRpcError =
        client.chunk(&payload_chunk).await?.into_inner();
    assert!(matches!(
        chunk,
        client::types::JsonRpcResponseForRpcChunkResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for chunk: {:#?}", chunk);

    Ok(())
}

async fn test_gas_price_with_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_gas_price = client::types::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::types::RpcGasPriceRequest {
            block_id: Some(client::types::BlockId::CryptoHash(block_hash.clone())),
        },
    };

    let gas_price: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError =
        client.gas_price(&payload_gas_price).await?.into_inner();
    assert!(matches!(
        gas_price,
        client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for gas_price with block: {:#?}", gas_price);

    Ok(())
}

async fn test_gas_price_without_block(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_gas_price = client::types::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::types::RpcGasPriceRequest { block_id: None },
    };

    let gas_price: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError =
        client.gas_price(&payload_gas_price).await?.into_inner();
    assert!(matches!(
        gas_price,
        client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for gas_price without block: {:#?}", gas_price);

    Ok(())
}

async fn test_health(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_health = client::types::JsonRpcRequestForHealth {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForHealthMethod::Health,
        params: client::types::RpcHealthRequest(()),
    };

    let health: client::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError =
        client.health(&payload_health).await?.into_inner();
    assert!(matches!(
        health,
        client::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError::Variant0 {
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
    sender_account_id: client::types::AccountId,
    sent_tx_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_light_client_proof = client::types::JsonRpcRequestForLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForLightClientProofMethod::LightClientProof,
        params: client::types::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::types::RpcLightClientExecutionProofRequestVariant0Type::Transaction,
        },
    };

    let light_client_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client.light_client_proof(&payload_light_client_proof).await?.into_inner();
    println!("response for light_client_proof: {:#?}", light_client_proof);

    assert!(matches!(light_client_proof, client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError::Variant0 { result: _, .. }));
    Ok(())
}

async fn test_next_light_client_block(
    client: &Client,
    block_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_next_light_client_block = client::types::JsonRpcRequestForNextLightClientBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForNextLightClientBlockMethod::NextLightClientBlock,
        params: client::types::RpcLightClientNextBlockRequest {
            last_block_hash: block_hash.clone(),
        },
    };

    let next_light_client_block: client::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client.next_light_client_block(&payload_next_light_client_block).await?.into_inner();
    assert!(matches!(
        next_light_client_block,
        client::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError::Variant0 {
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
    let payload_network_info = client::types::JsonRpcRequestForNetworkInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForNetworkInfoMethod::NetworkInfo,
        params: client::types::RpcNetworkInfoRequest(()),
    };

    let network_info: client::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client
        .network_info(&payload_network_info)
        .await?
        .into_inner();
    assert!(matches!(
        network_info,
        client::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for network_info: {:#?}", network_info);

    Ok(())
}

async fn test_send_tx(client: &Client, base64_signed_tx: String) -> Result<(), Box<dyn Error>> {
    let payload_send_tx = client::types::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::types::TxExecutionStatus::Executed,
        },
    };

    let send_tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client.send_tx(&payload_send_tx).await?.into_inner();
    assert!(matches!(
        send_tx,
        client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for send_tx: {:#?}", send_tx);

    Ok(())
}

async fn test_status(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_status = client::types::JsonRpcRequestForStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForStatusMethod::Status,
        params: client::types::RpcStatusRequest(()),
    };

    let status: client::types::JsonRpcResponseForRpcStatusResponseAndRpcError =
        client.status(&payload_status).await?.into_inner();
    assert!(matches!(
        status,
        client::types::JsonRpcResponseForRpcStatusResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for status: {:#?}", status);

    Ok(())
}

async fn test_validators(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_validators = client::types::JsonRpcRequestForValidators {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForValidatorsMethod::Validators,
        params: client::types::RpcValidatorRequest::Latest,
    };

    let validators: client::types::JsonRpcResponseForRpcValidatorResponseAndRpcError =
        client.validators(&payload_validators).await?.into_inner();
    assert!(matches!(
        validators,
        client::types::JsonRpcResponseForRpcValidatorResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for validators: {:#?}", validators);

    Ok(())
}

async fn test_client_config(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_client_config = client::types::JsonRpcRequestForClientConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForClientConfigMethod::ClientConfig,
        params: client::types::RpcClientConfigRequest(()),
    };

    let client_config: client::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client
        .client_config(&payload_client_config)
        .await?
        .into_inner();
    assert!(matches!(
        client_config,
        client::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError::Variant0 {
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
    sender_account_id: client::types::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_experimental_changes = client::types::JsonRpcRequestForExperimentalChanges {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalChangesMethod::ExperimentalChanges,
        params: client::types::RpcStateChangesInBlockByTypeRequest::Variant0 {
            changes_type: client::types::RpcStateChangesInBlockByTypeRequestVariant0ChangesType::AccountChanges,
            account_ids: vec![sender_account_id],
            block_id: client::types::BlockId::CryptoHash(block_hash.clone()),
        }
    };

    let experimental_changes: client::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError = client.experimental_changes(&payload_experimental_changes).await?.into_inner();
    assert!(matches!(
        experimental_changes,
        client::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    if let client::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError::Variant0 {
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
    let payload_experimental_changes_in_block = client::types::JsonRpcRequestForExperimentalChangesInBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalChangesInBlockMethod::ExperimentalChangesInBlock,
        params: client::types::RpcStateChangesInBlockRequest::BlockId(client::types::BlockId::CryptoHash(block_hash.clone()))
    };

    let experimental_changes_in_block: client::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError = client.experimental_changes_in_block(&payload_experimental_changes_in_block).await?.into_inner();
    assert!(matches!(experimental_changes_in_block, client::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError::Variant0 { result: _, .. }));
    if let client::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError::Variant0 { result, .. } = &experimental_changes_in_block {
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
    let payload_congestion_level = client::types::JsonRpcRequestForExperimentalCongestionLevel {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalCongestionLevelMethod::ExperimentalCongestionLevel,
        params: client::types::RpcCongestionLevelRequest::BlockShardId {
            block_id: client::types::BlockId::CryptoHash(block_hash.clone()),
            shard_id: client::types::ShardId(0)
        }
    };

    let congestion_level: client::types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError =
        client
            .experimental_congestion_level(&payload_congestion_level)
            .await?
            .into_inner();
    assert!(matches!(
        congestion_level,
        client::types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for congestion_level: {:#?}", congestion_level);

    Ok(())
}

async fn test_experimental_genesis_config(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_genesis_config = client::types::JsonRpcRequestForExperimentalGenesisConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalGenesisConfigMethod::ExperimentalGenesisConfig,
        params: client::types::GenesisConfigRequest(())
    };

    let genesis_config: client::types::JsonRpcResponseForGenesisConfigAndRpcError = client
        .experimental_genesis_config(&payload_genesis_config)
        .await?
        .into_inner();
    assert!(matches!(
        genesis_config,
        client::types::JsonRpcResponseForGenesisConfigAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for genesis_config: {:#?}", genesis_config);

    Ok(())
}

async fn test_experimental_light_client_proof(
    client: &Client,
    block_hash: CryptoHash,
    sender_account_id: client::types::AccountId,
    sent_tx_hash: CryptoHash,
) -> Result<(), Box<dyn Error>> {
    let payload_exp_light_client_proof = client::types::JsonRpcRequestForExperimentalLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalLightClientProofMethod::ExperimentalLightClientProof,
        params: client::types::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::types::RpcLightClientExecutionProofRequestVariant0Type::Transaction,
        }
    };

    let exp_light_client_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client.experimental_light_client_proof(&payload_exp_light_client_proof).await?.into_inner();
    assert!(matches!(exp_light_client_proof, client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError::Variant0 { result: _, .. }));

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
    let payload_exp_light_client_block = client::types::JsonRpcRequestForExperimentalLightClientBlockProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalLightClientBlockProofMethod::ExperimentalLightClientBlockProof,
        params: client::types::RpcLightClientBlockProofRequest {
            block_hash: block_hash.clone(),
            light_client_head: block_hash.clone(),
        }
    };

    let exp_light_client_block: client::types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError = client.experimental_light_client_block_proof(&payload_exp_light_client_block).await?.into_inner();
    assert!(matches!(
        exp_light_client_block,
        client::types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError::Variant0 {
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
    let payload_protocol_config = client::types::JsonRpcRequestForExperimentalProtocolConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalProtocolConfigMethod::ExperimentalProtocolConfig,
        params: client::types::RpcProtocolConfigRequest::BlockId(client::types::BlockId::CryptoHash(block_hash.clone()))
    };

    let protocol_config: client::types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError =
        client
            .experimental_protocol_config(&payload_protocol_config)
            .await?
            .into_inner();
    assert!(matches!(
        protocol_config,
        client::types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError::Variant0 {
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
    let payload_receipt = client::types::JsonRpcRequestForExperimentalReceipt {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalReceiptMethod::ExperimentalReceipt,
        params: client::types::RpcReceiptRequest {
            receipt_id: executed_receipt_id,
        },
    };

    let receipt: client::types::JsonRpcResponseForRpcReceiptResponseAndRpcError = client
        .experimental_receipt(&payload_receipt)
        .await?
        .into_inner();
    assert!(matches!(
        receipt,
        client::types::JsonRpcResponseForRpcReceiptResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for receipt: {:#?}", receipt);

    Ok(())
}

async fn test_experimental_tx_status(
    client: &Client,
    sent_tx_hash: CryptoHash,
    sender_account_id: client::types::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_exp_tx_status = client::types::JsonRpcRequestForExperimentalTxStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalTxStatusMethod::ExperimentalTxStatus,
        params: client::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: sent_tx_hash.clone(),
            sender_account_id: sender_account_id.clone(),
            wait_until: client::types::TxExecutionStatus::None,
        },
    };

    let exp_tx_status: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client
        .experimental_tx_status(&payload_exp_tx_status)
        .await?
        .into_inner();
    assert!(matches!(
        exp_tx_status,
        client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for exp_tx_status: {:#?}", exp_tx_status);

    Ok(())
}

async fn test_experimental_validators_ordered(client: &Client) -> Result<(), Box<dyn Error>> {
    let payload_exp_validators = client::types::JsonRpcRequestForExperimentalValidatorsOrdered {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalValidatorsOrderedMethod::ExperimentalValidatorsOrdered,
        params: client::types::RpcValidatorsOrderedRequest {
            block_id: None
        }
    };

    let exp_validators: client::types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError =
        client
            .experimental_validators_ordered(&payload_exp_validators)
            .await?
            .into_inner();
    assert!(matches!(
        exp_validators,
        client::types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for exp_validators: {:#?}", exp_validators);

    Ok(())
}

async fn test_experimental_maintenance_windows(
    client_remote: &Client,
    sender_account_id: client::types::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_maintenance_windows = client::types::JsonRpcRequestForExperimentalMaintenanceWindows {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalMaintenanceWindowsMethod::ExperimentalMaintenanceWindows,
        params: client::types::RpcMaintenanceWindowsRequest {
            account_id: sender_account_id.clone(),
        }
    };

    let maintenance_windows: client::types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError =
        client_remote
            .experimental_maintenance_windows(&payload_maintenance_windows)
            .await?
            .into_inner();
    assert!(matches!(
        maintenance_windows,
        client::types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError::Variant0 {
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
    let payload_split_storage = client::types::JsonRpcRequestForExperimentalSplitStorageInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalSplitStorageInfoMethod::ExperimentalSplitStorageInfo,
        params: client::types::RpcSplitStorageInfoRequest(serde_json::Map::new())
    };

    let split_storage_info: client::types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError = client.experimental_split_storage_info(&payload_split_storage).await?.into_inner();
    assert!(matches!(
        split_storage_info,
        client::types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError::Variant0 {
            result: _,
            ..
        }
    ));

    println!("response for split_storage_info: {:#?}", split_storage_info);

    Ok(())
}

async fn test_query_account(
    client: &Client,
    sender_account_id: client::types::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_query_account = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::ViewAccountByFinality {
            account_id: sender_account_id,
            request_type: client::types::ViewAccountByFinalityRequestType::ViewAccount,
            finality: client::types::Finality::Final,
        },
    };

    let query_account: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError =
        client.query(&payload_query_account).await?.into_inner();
    assert!(matches!(
        query_account,
        client::types::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 { result: _, .. }
    ));

    println!("response for query_account: {:#?}", query_account);

    Ok(())
}

async fn test_function_call(
    client: &Client,
    sender_account_id: client::types::AccountId,
) -> Result<(), Box<dyn Error>> {
    let payload_function_call = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::CallFunctionByFinality {
            account_id: sender_account_id.clone(),
            request_type: client::types::CallFunctionByFinalityRequestType::CallFunction,
            method_name: "get_greeting".to_string(),
            args_base64: client::types::FunctionArgs("".to_string()),
            finality: client::types::Finality::Final,
        },
    };

    let function_call: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError =
        client.query(&payload_function_call).await?.into_inner();
    assert!(matches!(
        function_call,
        client::types::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 { result: _, .. }
    ));
    if let client::types::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 {
        result, ..
    } = &function_call
    {
        if let client::types::RpcQueryResponse::Variant3 { result, .. } = result {
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
        client::types::AccountId,
        CryptoHash,
        String,
        CryptoHash,
        CryptoHash,
        CryptoHash,
    ),
    Box<dyn Error>,
> {
    let sender_account_id: client::types::AccountId = "test.near".parse().unwrap();

    let payload_query_access_key = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::ViewAccessKeyByFinality {
            account_id: sender_account_id.clone(),
            public_key: client::types::PublicKey(signer.public_key().to_string()),
            request_type: client::types::ViewAccessKeyByFinalityRequestType::ViewAccessKey,
            finality: client::types::Finality::Final,
        },
    };

    let access_key: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError = client_local
        .query(&payload_query_access_key)
        .await?
        .into_inner();
    println!("response for access_key: {:#?}", access_key);

    let access_key_block_hash: CryptoHash;
    let access_key_nonce: u64;
    if let client::types::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 {
        result, ..
    } = access_key
    {
        if let client::types::RpcQueryResponse::Variant4 {
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

    let payload_send_tx = client::types::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(
                base64_signed_tx.clone(),
            ),
            wait_until: client::types::TxExecutionStatus::Executed,
        },
    };

    let send_tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError =
        client_local.send_tx(&payload_send_tx).await?.into_inner();
    println!("response for send_tx: {:#?}", send_tx);

    let sent_tx_hash: CryptoHash;
    let executed_receipt_id: CryptoHash;
    if let client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 {
        result,
        ..
    } = send_tx
    {
        if let client::types::RpcTransactionResponse::Variant1 {
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

    let payload_block_final = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::Finality(client::types::Finality::Final),
    };

    let block_final: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client_local.block(&payload_block_final).await?.into_inner();
    println!("response for block_final: {:#?}", block_final);
    let block_final_hash: CryptoHash;
    if let client::types::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 {
        result, ..
    } = block_final
    {
        block_final_hash = result.header.hash;
    } else {
        return Err("final block is not in expected format".into());
    }

    sleep(Duration::from_secs(2)).await;

    let later_block: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client_local.block(&payload_block_final).await?.into_inner();
    let later_block_hash: CryptoHash;
    if let client::types::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 {
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
    home_dir.push("test_node_a7d");

    let rpc_port: u16 = 3040;
    let net_port: u16 = 3031;

    let version = "master/a7dc91c14928974401da4cf3b8ba9341a547e6aa";

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
