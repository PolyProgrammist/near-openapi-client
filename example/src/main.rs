use std::error::Error;
use tokio::time::{sleep, Duration};
use near_crypto::InMemorySigner;

const NEAR_RPC_URL_LOCAL: &str = "http://127.0.0.1:3030";
const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";


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

    // let txprinted = print_transaction(&signer).await;
    // match txprinted {
    //     Ok(..) => {
    //         println!("hooray")
    //     }
    //     Err(err) => {
    //         println!("error {:#?}", err);
    //     }
    // }

    sleep(Duration::from_secs(100)).await;

    child.kill().await?;

    Ok(())
}
