use near_openapi_client as client;
use client::Client;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use client::types::CryptoHash;
use tokio::time::{sleep, Duration};
use near_primitives::transaction::{Action, TransferAction, Transaction, TransactionV0};
use near_crypto::{InMemorySigner, KeyType, Signer};

const NEAR_RPC_URL_LOCAL: &str = "http://127.0.0.1:3030";
const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";

async fn print_transaction(signer: &Signer) -> Result<(), Box<dyn Error>> {
    let sender_account_id: client::types::AccountId = "test.near".parse().unwrap();
    let signed_tx_base64 = "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string();

    let client_local = Client::new(NEAR_RPC_URL_LOCAL);
    let client_remote = Client::new(NEAR_RPC_URL_REMOTE);

    let payload_query_access_key = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::Variant11 { 
            account_id: "test.near".parse().unwrap(),
            public_key: client::types::PublicKey(signer.public_key().to_string()),
            request_type: client::types::RpcQueryRequestVariant11RequestType::ViewAccessKey,
            finality: client::types::Finality::Final,
        }
    }; 

    let access_key: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError = client_local.query(&payload_query_access_key).await?.into_inner();
    println!("the_response access_key: {:#?}", access_key);

    let access_key_block_hash: CryptoHash;
    let access_key_nonce: u64;
    if let client::types::JsonRpcResponseForRpcQueryResponseAndRpcError::Variant0 { id, jsonrpc, result } = access_key {
        if let client::types::RpcQueryResponse::Variant4 { block_hash, block_height, nonce, permission } = result {
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
    let function_call_action = near_primitives::transaction::Action::FunctionCall(
        Box::new(near_primitives::transaction::FunctionCallAction {
            method_name: "set_greeting".to_string(),
            args:  b"hola".to_vec(),
            gas: 300_000_000_000_000,
            deposit: 0,
        }),
    );

    let transfer_amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yocto
    let tx = Transaction::V0(TransactionV0 {
        signer_id: "test.near".parse().unwrap(),
        public_key: signer.public_key(),
        nonce: access_key_nonce + 1,
        block_hash: access_key_block_hash.to_string().parse().unwrap(),
        receiver_id: "test.near".parse().unwrap(),
        actions: vec![Action::Transfer(TransferAction { deposit: transfer_amount }), deploy_contract_action, function_call_action],
    });
    let signed_tx = tx.sign(&signer);
    let base64_signed_tx = near_primitives::serialize::to_base64(&borsh::to_vec(&signed_tx)?);

    let payloadSendTx = client::types::JsonRpcRequestForSendTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForSendTxMethod::SendTx,
        params: client::types::RpcSendTransactionRequest {
            signed_tx_base64: near_openapi_client::types::SignedTransaction(base64_signed_tx.clone()),
            wait_until: client::types::TxExecutionStatus::Executed
        }
    };

    let send_tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.send_tx(&payloadSendTx).await?.into_inner();
    println!("the_response send_tx: {:#?}", send_tx);

    let sent_tx_hash: CryptoHash;
    let executed_receipt_id: CryptoHash;
    if let client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError::Variant0 { id, jsonrpc, result } = send_tx {
        if let client::types::RpcTransactionResponse::Variant1 { final_execution_status, receipts_outcome, status, transaction, transaction_outcome } = result {
            sent_tx_hash = transaction.hash;
            executed_receipt_id = receipts_outcome[1].id.clone();
        } else {
            return Err("couldn't send transaction".into());
        }
    } else {
        return Err("couldn't get transaction info".into());
    }

    let payloadBlockFinal = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::Finality(client::types::Finality::Final)
    };

    let block_final: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client_local.block(&payloadBlockFinal).await?.into_inner();
    println!("the_response block_final: {:#?}", block_final);
    let block_final_hash: CryptoHash;
    if let client::types::JsonRpcResponseForRpcBlockResponseAndRpcError::Variant0 { id, jsonrpc, result } = block_final {
        block_final_hash = result.header.hash;
    } else {
        return Err("final block is not in expected format".into());
    }

    let payloadBlock = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::BlockId({
            client::types::BlockId::Variant1(block_final_hash.clone())
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
            block_id: client::types::BlockId::Variant1(block_final_hash.clone()),
            shard_id: client::types::ShardId(0)
        }
    };

    let payloadGasPriceWithBlock = client::types::JsonRpcRequestForGasPrice {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForGasPriceMethod::GasPrice,
        params: client::types::RpcGasPriceRequest {
            block_id: Some(client::types::BlockId::Variant1(block_final_hash.clone()))
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
            light_client_head: block_final_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::types::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let payloadNextLightClientBlock = client::types::JsonRpcRequestForNextLightClientBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForNextLightClientBlockMethod::NextLightClientBlock,
        params: client::types::RpcLightClientNextBlockRequest {
            last_block_hash: block_final_hash.clone(),
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
            account_ids: vec!["test.near".parse().unwrap()],
            block_id: client::types::BlockId::Variant1(block_final_hash.clone()),
        }
    };

    let payloadChangesInBlock = client::types::JsonRpcRequestForExperimentalChangesInBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalChangesInBlockMethod::ExperimentalChangesInBlock,
        params: client::types::RpcStateChangesInBlockRequest::BlockId(client::types::BlockId::Variant1(block_final_hash.clone()))
    };

    let payloadCongestionLevel = client::types::JsonRpcRequestForExperimentalCongestionLevel {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalCongestionLevelMethod::ExperimentalCongestionLevel,
        params: client::types::RpcCongestionLevelRequest::Variant0 {
            block_id: client::types::BlockId::Variant1(block_final_hash.clone()),
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
            light_client_head: block_final_hash.clone(),
            sender_id: sender_account_id.clone(),
            transaction_hash: sent_tx_hash.clone(),
            type_: client::types::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let payloadExpLightClientBlock = client::types::JsonRpcRequestForExperimentalLightClientBlockProof {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalLightClientBlockProofMethod::ExperimentalLightClientBlockProof,
        params: client::types::RpcLightClientBlockProofRequest {
            block_hash: block_final_hash.clone(),
            light_client_head: block_final_hash.clone(),
        }
    };

    let payloadProtocolConfig = client::types::JsonRpcRequestForExperimentalProtocolConfig {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalProtocolConfigMethod::ExperimentalProtocolConfig,
        params: client::types::RpcProtocolConfigRequest::BlockId(client::types::BlockId::Variant1(block_final_hash.clone()))
    };

    let payloadReceipt = client::types::JsonRpcRequestForExperimentalReceipt {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalReceiptMethod::ExperimentalReceipt,
        params: client::types::RpcReceiptRequest {
            receipt_id: executed_receipt_id,
        }
    };

    let payloadExpTxStatus = client::types::JsonRpcRequestForExperimentalTxStatus {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForExperimentalTxStatusMethod::ExperimentalTxStatus,
        params: client::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: sent_tx_hash.clone(),
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

    let payloadQueryAccount = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::Variant8 { 
            account_id: "test.near".parse().unwrap(),
            request_type: client::types::RpcQueryRequestVariant8RequestType::ViewAccount,
            finality: client::types::Finality::Final,
        }
    };

    let payloadFunctionCall = client::types::JsonRpcRequestForQuery {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForQueryMethod::Query,
        params: client::types::RpcQueryRequest::Variant13 { 
            account_id: "test.near".parse().unwrap(),
            request_type: client::types::RpcQueryRequestVariant13RequestType::CallFunction,
            method_name: "get_greeting".to_string(),
            args_base64: "".to_string(), //near_openapi_client::types::Base64EncodedString("eyJwYXJhbXMiOlsiYWNjX2RlcG9zaXQiLCJ0ZXN0Lm5lYXIiXX0=".to_string()),
            finality: client::types::Finality::Final,
        }
    };

    let block: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client_local.block(&payloadBlock).await?.into_inner();
    println!("the_response block: {:#?}", block);

    let broadcast_async: client::types::JsonRpcResponseForCryptoHashAndRpcError = client_local.broadcast_tx_async(&payloadBroadcastAsync).await?.into_inner();
    println!("the_response broadcast_async: {:#?}", broadcast_async);

    let broadcast_commit: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.broadcast_tx_commit(&payloadBroadcastCommit).await?.into_inner();
    println!("the_response broadcast_commit: {:#?}", broadcast_commit);
    
    let chunk: client::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client_local.chunk(&payloadChunk).await?.into_inner();
    println!("the_response chunk: {:#?}", chunk);

    let gas_price_with_block: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithBlock).await?.into_inner();
    println!("the_response gas_price_with_block: {:#?}", gas_price_with_block);

    let gas_price_without_block: client::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithoutBlock).await?.into_inner();
    println!("the_response gas_price_without_block: {:#?}", gas_price_without_block);

    let health: client::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError = client_local.health(&payloadHealth).await?.into_inner();
    println!("the_response health: {:#?}", health);

    let light_client_execution_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_local.light_client_proof(&payloadLightClientExecutionProof).await?.into_inner();
    println!("the_response light_client_execution_proof: {:#?}", light_client_execution_proof);

    let next_light_client_block: client::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client_local.next_light_client_block(&payloadNextLightClientBlock).await?.into_inner();
    println!("the_response next_light_client_block: {:#?}", next_light_client_block);

    let network_info: client::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client_local.network_info(&payloadNetworkInfo).await?.into_inner();
    println!("the_response network_info: {:#?}", network_info);

    let send_tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.send_tx(&payloadSendTx).await?.into_inner();
    println!("the_response send_tx: {:#?}", send_tx);

    let payloadTx = client::types::JsonRpcRequestForTx {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForTxMethod::Tx,
        params: client::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: sent_tx_hash.clone(),
            sender_account_id: sender_account_id.clone(),
            wait_until: client::types::TxExecutionStatus::None,
        }
    };

    let tx: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.tx(&payloadTx).await?.into_inner();
    println!("the_response tx: {:#?}", tx);

    let status = client_local.status(&payloadStatus).await?;
    println!("the_response status: {:#?}", status);

    let validators: client::types::JsonRpcResponseForRpcValidatorResponseAndRpcError = client_local.validators(&payloadValidators).await?.into_inner();
    println!("the_response validators: {:#?}", validators);

    let client_config: client::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client_local.client_config(&payloadClientConfig).await?.into_inner();
    println!("the_response client_config: {:#?}", client_config);

    let experimental_changes: client::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError = client_local.experimental_changes(&payloadStateChanges).await?.into_inner();
    println!("the_response experimental_changes: {:#?}", experimental_changes);

    let experimental_changes_in_block: client::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError = client_local.experimental_changes_in_block(&payloadChangesInBlock).await?.into_inner();
    println!("the_response experimental_changes_in_block: {:#?}", experimental_changes_in_block);

    let congestion_level: client::types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError = client_local.experimental_congestion_level(&payloadCongestionLevel).await?.into_inner();
    println!("the_response congestion_level: {:#?}", congestion_level);

    let genesis_config_local: client::types::JsonRpcResponseForGenesisConfigAndRpcError = client_local.experimental_genesis_config(&payloadGenesisConfig).await?.into_inner();
    println!("the_response genesis_config_local: {:#?}", genesis_config_local);

    let experimental_light_client_execution_proof: client::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_local.experimental_light_client_proof(&payloadExpLightClientExecutionProof).await?.into_inner();
    println!("the_response experimental_light_client_execution_proof: {:#?}", experimental_light_client_execution_proof);

    let experimental_next_light_client_block: client::types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError = client_local.experimental_light_client_block_proof(&payloadExpLightClientBlock).await?.into_inner();
    println!("the_response experimental_next_light_client_block: {:#?}", experimental_next_light_client_block);

    let experimental_protocol_config: client::types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError = client_local.experimental_protocol_config(&payloadProtocolConfig).await?.into_inner();
    println!("the_response experimental_protocol_config: {:#?}", experimental_protocol_config);

    let experimental_receipt: client::types::JsonRpcResponseForRpcReceiptResponseAndRpcError = client_local.experimental_receipt(&payloadReceipt).await?.into_inner();
    println!("the_response experimental_receipt: {:#?}", experimental_receipt);

    let experimental_tx_status: client::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_local.experimental_tx_status(&payloadExpTxStatus).await?.into_inner();
    println!("the_response experimental_tx_status: {:#?}", experimental_tx_status);

    let experimental_validators: client::types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError = client_local.experimental_validators_ordered(&payloadExpValidators).await?.into_inner();
    println!("the_response experimental_validators: {:#?}", experimental_validators);

    // TODO: setup maintenance windows in the sandbox and test it locally
    let experimental_maintenance_windows: client::types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcError = client_remote.experimental_maintenance_windows(&payloadMaintenanceWindows).await?.into_inner();
    println!("the_response experimental_maintenance_windows: {:#?}", experimental_maintenance_windows);

    let experimental_split_storage: client::types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError = client_local.experimental_split_storage_info(&payloadSplitStorage).await?.into_inner();
    println!("the_response experimental_split_storage: {:#?}", experimental_split_storage);

    let query_account: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError = client_local.query(&payloadQueryAccount).await?.into_inner();
    println!("the_response query_account: {:#?}", query_account);

    let function_call: client::types::JsonRpcResponseForRpcQueryResponseAndRpcError = client_local.query(&payloadFunctionCall).await?.into_inner();
    println!("the_response function_call: {:#?}", function_call);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut home_dir = std::env::temp_dir();
    home_dir.push("test-sandboxxx");

    let rpc_port: u16 = 3030;
    let net_port: u16 = 3031;

    let output = near_sandbox_utils::init(&home_dir)?
        .wait_with_output()
        .await
        .unwrap();

    let mut child = near_sandbox_utils::run_with_version(&home_dir, rpc_port, net_port, "master")?;

    sleep(Duration::from_secs(2)).await;

    let mut validator_key = home_dir.clone();
    validator_key.push("validator_key.json");
    let signer = InMemorySigner::from_file(&validator_key)?;

    let txprinted = print_transaction(&signer).await;
    match txprinted {
        Ok(..) => {
            println!("hooray")
        }
        Err(err) => {
            println!("error {:#?}", err);
        }
    }

    sleep(Duration::from_secs(100)).await;

    child.kill().await?;

    Ok(())
}
