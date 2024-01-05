use std::env;
use sui_sdk::{
    rpc_types::SuiObjectDataOptions,
    types::{base_types::ObjectID, object::Owner},
    SuiClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    // The first command line argument is an object ID
    let object_id: ObjectID = args[1].parse()?;

    // Sui mainnet -- https://fullnode.mainnet.sui.io:443
    let sui_mainnet = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await?;
    println!("Sui mainnet version: {}", sui_mainnet.api_version());

    // Read the object with the given ID
    let object = sui_mainnet
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions {
                show_type: true,
                show_owner: true,
                show_previous_transaction: true,
                show_display: true,
                show_content: true,
                show_bcs: true,
                show_storage_rebate: true,
            },
        )
        .await?;

    let inner_object = object.data.as_ref().unwrap();

    // Print some object meta-data
    let version = inner_object.version;
    let previous_transaction = inner_object.previous_transaction;

    println!(
        "Object version: {} Previous transaction: {:?}",
        version, previous_transaction
    );
    inner_object.owner.map(|owner| {
        if let Owner::Shared {
            initial_shared_version,
        } = owner
        {
            println!("Shared at initial version: {}", initial_shared_version);
        }

        println!("Owner: {}", owner);
    });

    inner_object.content.as_ref().map(|content| {
        println!("Content:\n{}", content);
    });

    Ok(())
}
