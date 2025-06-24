use near_openapi_client as client;
use std::error::Error;
use tokio::time::{sleep, Duration};

const NEAR_RPC_URL_LOCAL: &str = "http://127.0.0.1:3050";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut home_dir = std::env::temp_dir();
    home_dir.push("main_example");

    let rpc_port: u16 = 3050;
    let net_port: u16 = 3031;

    near_sandbox_utils::init(&home_dir)?
        .wait_with_output()
        .await
        .unwrap();

    let mut child = near_sandbox_utils::run_with_version(&home_dir, rpc_port, net_port, "master")?;

    sleep(Duration::from_secs(2)).await;

    let client_local = client::Client::new(NEAR_RPC_URL_LOCAL);

    let payload_block_final = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::Finality(client::types::Finality::Final),
    };

    let block_final: client::types::JsonRpcResponseForRpcBlockResponseAndRpcError =
        client_local.block(&payload_block_final).await?.into_inner();
    println!("the_response block_final: {:#?}", block_final);

    child.kill().await?;

    Ok(())
}
