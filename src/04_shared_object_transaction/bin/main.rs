use shared_crypto::intent::Intent;
use std::env;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponseOptions},
    types::{base_types::ObjectID, transaction::ObjectArg, Identifier},
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Command, Transaction, TransactionData},
    },
    SuiClientBuilder,
};

// Call using:
// $ cargo run --bin 04_shared_object_transaction ~/.sui/sui_config/sui.keystore 0x0d69a64f09fd9587588bf744345da5b84c0c42a4f7d73e4b79348c4f8be9e721

const PACKAGE_ID_CONST: &str = "0x74246e5987c3b20aeed158bd01620f86b50602c4970a33c3b3570018b74286a9";
const TROPHY_STATION_ID: &str =
    "0x3d987e06da12d9e20bc8c09dae2f04e14df96d75ecf0542c910f026c5842670c";
const TROPHY_STATION_VERSION: u64 = 3;

// The Sui mainnet address of the package and trophy station by the way is:
// const PACKAGE_ID_CONST: &str = "0x279525274aa623ef31a25ad90e3b99f27c8dbbad636a6454918855c81d625abc";
// const TROPHY_STATION_ID: &str = "0xea590d6ad1322f8245b84019441cf4d3b438032cb0bc7857d7892adc267ff401";
// const TROPHY_STATION_VERSION: u64 = 30622213;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Read the local key store as provided by the path
    let args: Vec<String> = env::args().collect();
    let sui_keystore = &args[1];
    let keystore = FileBasedKeystore::new(&sui_keystore.into())?;

    // Sui local network
    let sui_local = SuiClientBuilder::default()
        .build("http://127.0.0.1:9000")
        .await?;
    println!("Sui local version: {}", sui_local.api_version());

    let package_id = PACKAGE_ID_CONST.parse()?;

    let my_address = keystore.addresses()[0];
    println!("My address: {}", my_address);

    // Get all my own objects

    let coins_response = &sui_local
        .read_api()
        .get_owned_objects(
            my_address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new().with_type(),
            )),
            None,
            None,
        )
        .await?;

    // Find a coin to use
    let coin = coins_response
        .data
        .iter()
        .find(|obj| obj.data.as_ref().unwrap().is_gas_coin())
        .unwrap();
    let coin = coin.data.as_ref().unwrap();

    // Get the latest reference for the trophy object
    let trophy_object_id: ObjectID = args[2].parse()?;
    let trophy_object = sui_local
        .read_api()
        .get_object_with_options(trophy_object_id, SuiObjectDataOptions::default())
        .await?
        .data
        .unwrap();

    // Make a Programmable Transaction Block with 1 transaction inside

    let mut ptb = ProgrammableTransactionBuilder::new();

    let trophy_station_input = ptb.obj(ObjectArg::SharedObject {
        id: TROPHY_STATION_ID.parse()?,
        initial_shared_version: TROPHY_STATION_VERSION.into(),
        mutable: true,
    })?;

    let trophy_input = ptb.obj(ObjectArg::ImmOrOwnedObject(trophy_object.object_ref()))?;

    ptb.command(Command::move_call(
        package_id,
        Identifier::new("dev_trophy")?,
        Identifier::new("stamp_trophy")?,
        vec![],
        vec![trophy_station_input, trophy_input],
    ));

    let builder = ptb.finish();

    // Build the transaction data

    let gas_budget = 5_000_000;
    let gas_price = sui_local.read_api().get_reference_gas_price().await?;

    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        my_address,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    // Sign the transaction
    let signature = keystore.sign_secure(&my_address, &tx_data, Intent::sui_transaction())?;

    // Submit the transaction
    let transaction_response = sui_local
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\nTransaction information: ");
    println!("{:?}", transaction_response.digest);

    Ok(())
}
