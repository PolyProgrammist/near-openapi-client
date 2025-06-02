use near_openapi_client as client;
use client::Client;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use client::types::CryptoHash;

const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";
const NEAR_RPC_URL_LOCAL: &str = "http://127.0.0.1:3030";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    let transaction_hash: CryptoHash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse().unwrap(); // Replace with your TX hash
    let block_hash: CryptoHash = "FnXmhMHHQW3CgbBqQMbkLQSn4GoVnCUKo17cs3TmyKEc".parse().unwrap();
    let sender_account_id: client::types::AccountId = "miraclx.near".parse().unwrap();
    let signed_tx_base64 = "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string();

    let client_remote = Client::new(NEAR_RPC_URL_REMOTE);
    let client_local = Client::new(NEAR_RPC_URL_LOCAL);

    let payloadBlock = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::BlockId({
            client::types::BlockId::Variant1(block_hash.clone())
        })
    };

    let payloadBroadcastAsync = client::types::JsonRpcRequestForBroadcastTxAsync {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBroadcastTxAsyncMethod::BroadcastTxAsync,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(signed_tx_base64.clone()),
            wait_until: client::types::TxExecutionStatus::Executed
        }
    };

    let payloadBroadcastCommit = client::types::JsonRpcRequestForBroadcastTxCommit {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBroadcastTxCommitMethod::BroadcastTxCommit,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(signed_tx_base64.clone()),
            wait_until: client::types::TxExecutionStatus::Executed
        }
    };

    let payloadChunk = client::types::JsonRpcRequestForChunk {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForChunkMethod::Chunk,
        params: client::types::RpcChunkRequest::Variant0{
            block_id: client::types::BlockId::Variant1(block_hash.clone()),
            shard_id: client::types::ShardId(0)
        }
    };

    let payloadGasPriceWithBlock = client::types::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::types::RpcGasPriceRequest {
            block_id: Some(client::types::BlockId::Variant1(block_hash.clone()))
        }
    };

    let payloadGasPriceWithoutBlock = client::types::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::types::RpcGasPriceRequest {
            block_id: None
        }
    };

    let payloadHealth = client::types::JsonRpcRequestForHealth {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForHealthMethod::Health,
        params: client::types::RpcHealthRequest(serde_json::Map::new())
    };

    let payloadLightClientExecutionProof = client::types::JsonRpcRequestForLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForLightClientProofMethod::LightClientProof,
        params: client::types::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: transaction_hash.clone(),
            type_: client::types::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let payloadNextLightClientBlock = client::types::JsonRpcRequestForNextLightClientBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForNextLightClientBlockMethod::NextLightClientBlock,
        params: client::types::RpcLightClientNextBlockRequest {
            last_block_hash: block_hash.clone(),
        }
    };

    let payloadNetworkInfo = client::types::JsonRpcRequestForNetworkInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForNetworkInfoMethod::NetworkInfo,
        params: client::types::RpcNetworkInfoRequest(serde_json::Map::new())
    };

    let payloadSendTx = client::types::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(signed_tx_base64.clone()),
            wait_until: client::types::TxExecutionStatus::Executed
        }
    };

    let payloadTx = client::types::JsonRpcRequestForTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForTxMethod::Tx,
        params: client::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: transaction_hash.clone(),
            sender_account_id: sender_account_id.clone(),
            wait_until: client::types::TxExecutionStatus::None,
        }
    };

    let payloadStatus = client::types::JsonRpcRequestForStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForStatusMethod::Status,
        params: client::types::RpcStatusRequest(serde_json::Map::new())
    };

    let payloadValidators = client::types::JsonRpcRequestForValidators {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForValidatorsMethod::Validators,
        params: client::types::RpcValidatorRequest::Latest
    };

    let payloadClientConfig = client::types::JsonRpcRequestForClientConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForClientConfigMethod::ClientConfig,
        params: client::types::RpcClientConfigRequest(serde_json::Map::new())
    };

    let payloadStateChanges = client::types::JsonRpcRequestForExperimentalChanges {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalChangesMethod::ExperimentalChanges,
        params: client::types::RpcStateChangesInBlockByTypeRequest::Variant0 {
            changes_type: client::types::RpcStateChangesInBlockByTypeRequestVariant0ChangesType::AccountChanges,
            account_ids: vec!["token.sweat".parse().unwrap()],
            block_id: client::types::BlockId::Variant1(block_hash.clone()),
        }
    };

    let payloadChangesInBlock = client::types::JsonRpcRequestForExperimentalChangesInBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalChangesInBlockMethod::ExperimentalChangesInBlock,
        params: client::types::RpcStateChangesInBlockRequest::BlockId(client::types::BlockId::Variant1(block_hash.clone()))
    };

    let payloadCongestionLevel = client::types::JsonRpcRequestForExperimentalCongestionLevel {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalCongestionLevelMethod::ExperimentalCongestionLevel,
        params: client::types::RpcCongestionLevelRequest::Variant0 {
            block_id: client::types::BlockId::Variant1(block_hash.clone()),
            shard_id: client::types::ShardId(0)
        }
    };

    let payloadGenesisConfig = client::types::JsonRpcRequestForExperimentalGenesisConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalGenesisConfigMethod::ExperimentalGenesisConfig,
        params: client::types::GenesisConfigRequest(serde_json::Map::new())
    };

    let payloadExpLightClientExecutionProof = client::types::JsonRpcRequestForExperimentalLightClientProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalLightClientProofMethod::ExperimentalLightClientProof,
        params: client::types::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: block_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: transaction_hash.clone(),
            type_: client::types::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let payloadExpLightClientBlock = client::types::JsonRpcRequestForExperimentalLightClientBlockProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalLightClientBlockProofMethod::ExperimentalLightClientBlockProof,
        params: client::types::RpcLightClientBlockProofRequest {
            block_hash: block_hash.clone(),
            light_client_head: block_hash.clone(),
        }
    };

    let payloadProtocolConfig = client::types::JsonRpcRequestForExperimentalProtocolConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalProtocolConfigMethod::ExperimentalProtocolConfig,
        params: client::types::RpcProtocolConfigRequest::BlockId(client::types::BlockId::Variant1(block_hash.clone()))
    };

    let payloadReceipt = client::types::JsonRpcRequestForExperimentalReceipt {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalReceiptMethod::ExperimentalReceipt,
        params: client::types::RpcReceiptRequest {
            receipt_id: "GVpXUxpyo715x7fcvFuzJMJ1zimU1vCJggVwMyGAM6oH".parse().unwrap(),
        }
    };

    let payloadExpTxStatus = client::types::JsonRpcRequestForExperimentalTxStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalTxStatusMethod::ExperimentalTxStatus,
        params: client::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: transaction_hash.clone(),
            sender_account_id: sender_account_id.clone(),
            wait_until: client::types::TxExecutionStatus::None,
        }
    };

    let payloadExpValidators = client::types::JsonRpcRequestForExperimentalValidatorsOrdered {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalValidatorsOrderedMethod::ExperimentalValidatorsOrdered,
        params: client::types::RpcValidatorsOrderedRequest {
            block_id: None
        }
    };

    let payloadMaintenanceWindows = client::types::JsonRpcRequestForExperimentalMaintenanceWindows {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalMaintenanceWindowsMethod::ExperimentalMaintenanceWindows,
        params: client::types::RpcMaintenanceWindowsRequest {
            account_id: sender_account_id.clone(),
        }
    };

    let payloadSplitStorage = client::types::JsonRpcRequestForExperimentalSplitStorageInfo {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalSplitStorageInfoMethod::ExperimentalSplitStorageInfo,
        params: client::types::RpcSplitStorageInfoRequest(serde_json::Map::new())
    };

    let block: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client_local.block(&payloadBlock).await?.into_inner();
    println!("the_response block: {:#?}", block);

    let broadcast_async: client::types::JsonRpcResponseForCryptoHashAndRpcError = client_local.broadcast_tx_async(&payloadBroadcastAsync).await?.into_inner();
    println!("the_response broadcast_async: {:#?}", broadcast_async);

    let broadcast_commit: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.broadcast_tx_commit(&payloadBroadcastCommit).await?.into_inner();
    println!("the_response broadcast_commit: {:#?}", broadcast_commit);
    
    let chunk: client::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client_local.chunk(&payloadChunk).await?.into_inner();
    println!("the_response chunk: {:#?}", chunk);

    // // local as currently accepts only array, fixed in new version
    // let gas_price_with_block: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithBlock).await?.into_inner();
    // println!("the_response gas_price_with_block: {:#?}", gas_price_with_block);

    // let gas_price_without_block: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithoutBlock).await?.into_inner();
    // println!("the_response gas_price_without_block: {:#?}", gas_price_without_block);

    let health: client::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError = client_remote.health(&payloadHealth).await?.into_inner();
    println!("the_response health: {:#?}", health);

    // let light_client_execution_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_remote.light_client_proof(&payloadLightClientExecutionProof).await?.into_inner();
    // println!("the_response light_client_execution_proof: {:#?}", light_client_execution_proof);

    // let next_light_client_block: client::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client_remote.next_light_client_block(&payloadNextLightClientBlock).await?.into_inner();
    // println!("the_response next_light_client_block: {:#?}", next_light_client_block);

    // let network_info: client::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client_remote.network_info(&payloadNetworkInfo).await?.into_inner();
    // println!("the_response network_info: {:#?}", network_info);

    // let send_tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.send_tx(&payloadSendTx).await?.into_inner();
    // println!("the_response send_tx: {:#?}", send_tx);

    // let tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.tx(&payloadTx).await?.into_inner();
    // println!("the_response tx: {:#?}", tx);

    // // local as ".version.commit" introduced recently: https://github.com/near/nearcore/pull/12722/files
    // // let status = client_local.status(&payloadStatus).await?;
    // // println!("the_response status: {:#?}", status);

    // let validators: client::types::JsonRpcResponseForRpcValidatorResponseAndRpcError = client_remote.validators(&payloadValidators).await?.into_inner();
    // println!("the_response validators: {:#?}", validators);

    // let client_config: client::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client_local.client_config(&payloadClientConfig).await?.into_inner();
    // println!("the_response client_config: {:#?}", client_config);

    // let experimental_changes: client::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError = client_remote.experimental_changes(&payloadStateChanges).await?.into_inner();
    // println!("the_response experimental_changes: {:#?}", experimental_changes);

    // let experimental_changes_in_block: client::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError = client_remote.experimental_changes_in_block(&payloadChangesInBlock).await?.into_inner();
    // println!("the_response experimental_changes_in_block: {:#?}", experimental_changes_in_block);

    // let congestion_level: client::types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError = client_remote.experimental_congestion_level(&payloadCongestionLevel).await?.into_inner();
    // println!("the_response congestion_level: {:#?}", congestion_level);

    // // let genesis_config_local: client::types::JsonRpcResponseForGenesisConfigAndRpcError = client_local.experimental_genesis_config(&payloadGenesisConfig).await?.into_inner();
    // // println!("the_response genesis_config_local: {:#?}", genesis_config_local);

    // let genesis_config_remote: client::types::JsonRpcResponseForGenesisConfigAndRpcError = client_remote.experimental_genesis_config(&payloadGenesisConfig).await?.into_inner();
    // println!("the_response genesis_config_remote: {:#?}", genesis_config_remote);

    // let experimental_light_client_execution_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_remote.experimental_light_client_proof(&payloadExpLightClientExecutionProof).await?.into_inner();
    // println!("the_response experimental_light_client_execution_proof: {:#?}", experimental_light_client_execution_proof);

    // let experimental_next_light_client_block: client::types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError = client_remote.experimental_light_client_block_proof(&payloadExpLightClientBlock).await?.into_inner();
    // println!("the_response experimental_next_light_client_block: {:#?}", experimental_next_light_client_block);

    // let experimental_protocol_config: client::types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError = client_remote.experimental_protocol_config(&payloadProtocolConfig).await?.into_inner();
    // println!("the_response experimental_protocol_config: {:#?}", experimental_protocol_config);

    // let experimental_receipt: client::types::JsonRpcResponseForRpcReceiptResponseAndRpcError = client_remote.experimental_receipt(&payloadReceipt).await?.into_inner();
    // println!("the_response experimental_receipt: {:#?}", experimental_receipt);

    // let experimental_tx_status: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.experimental_tx_status(&payloadExpTxStatus).await?.into_inner();
    // println!("the_response experimental_tx_status: {:#?}", experimental_tx_status);

    // let experimental_validators: client::types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError = client_remote.experimental_validators_ordered(&payloadExpValidators).await?.into_inner();
    // println!("the_response experimental_validators: {:#?}", experimental_validators);

    // // local as changed from tuple to struct
    // let experimental_maintenance_windows: client::types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError = client_remote.experimental_maintenance_windows(&payloadMaintenanceWindows).await?.into_inner();
    // println!("the_response experimental_maintenance_windows: {:#?}", experimental_maintenance_windows);

    // let experimental_split_storage: client::types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError = client_remote.experimental_split_storage_info(&payloadSplitStorage).await?.into_inner();
    // println!("the_response experimental_split_storage: {:#?}", experimental_split_storage);

    Ok(())
}

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let rpc_port: u16 = 3030;
    let net_port: u16 = 3031;

    let mut home_dir = std::env::temp_dir();
    home_dir.push("test-sandbox");

    let output = near_sandbox_utils::init(&home_dir)?
        .wait_with_output()
        .await
        .unwrap();
    // near_workspaces::network::set_sandbox_genesis(&home_dir)?;

    let mut child = near_sandbox_utils::run(&home_dir, rpc_port, net_port)?;

    sleep(Duration::from_secs(2)).await;

    let txprinted = print_transaction().await;
    match txprinted {
        Ok(..) => {
            println!("hooray")
        }
        Err(err) => {
            println!("error {:#?}", err);
        }
    }

    child.kill().await?;

    Ok(())
}
