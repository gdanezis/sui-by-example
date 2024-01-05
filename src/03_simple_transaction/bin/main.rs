use shared_crypto::intent::Intent;
use std::env;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponseOptions},
    types::Identifier,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Transaction, TransactionData},
    },
    SuiClientBuilder,
};

const PACKAGE_ID_CONST: &str = "0x74246e5987c3b20aeed158bd01620f86b50602c4970a33c3b3570018b74286a9";

// The Sui mainnet address of the package by the way is:
// const PACKAGE_ID_CONST: &str = "0x279525274aa623ef31a25ad90e3b99f27c8dbbad636a6454918855c81d625abc";

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

    // Make a Programmable Transaction Block with 1 transaction inside

    let mut ptb = ProgrammableTransactionBuilder::new();

    ptb.move_call(
        package_id,
        Identifier::new("dev_trophy")?,
        Identifier::new("self_award_trophy")?,
        vec![],
        vec![],
    )?;

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
