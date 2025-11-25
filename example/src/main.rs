use near_openapi_client as client;
use near_sandbox::{Sandbox, SandboxConfig};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_port: u16 = 3050;

    let mut cfg = SandboxConfig::default();
    cfg.rpc_port = Some(rpc_port);

    let sandbox = Sandbox::start_sandbox_with_config(cfg).await?;
    let rpc_url = format!("http://{}", sandbox.rpc_addr);

    sleep(Duration::from_secs(2)).await;

    let client_local = client::Client::new(&rpc_url);

    let payload_block_final = client::types::JsonRpcRequestForBlock {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: client::types::JsonRpcRequestForBlockMethod::Block,
        params: client::types::RpcBlockRequest::Finality(client::types::Finality::Final),
    };

    let block_final: client::types::JsonRpcResponseForRpcBlockResponseAndRpcBlockError =
        client_local.block(&payload_block_final).await?.into_inner();
    println!("response for block_final: {:#?}", block_final);

    drop(sandbox);

    Ok(())
}
