use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Sui mainnet -- https://fullnode.mainnet.sui.io:443
    let sui_mainnet = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await?;
    println!("Sui mainnet version: {}", sui_mainnet.api_version());

    // Get and print the latest checkpoint
    let latest_checkpoint = sui_mainnet
        .read_api()
        .get_latest_checkpoint_sequence_number()
        .await?;
    println!("Latest mainnet checkpoint: {}", latest_checkpoint);

    // Get the checkpoint with this ID and print the total number of transactions
    let checkpoint = sui_mainnet
        .read_api()
        .get_checkpoint(latest_checkpoint.into())
        .await?;
    println!(
        "Network total transactions: {}",
        checkpoint.network_total_transactions
    );

    Ok(())
}
