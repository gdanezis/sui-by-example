use shared_crypto::intent::Intent;
use std::env;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponseOptions},
    types::{
        base_types::{ObjectID, SequenceNumber},
        transaction::ObjectArg,
        Identifier,
    },
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Command, Transaction, TransactionData},
    },
    SuiClientBuilder,
};

use sha2::{Sha256, Digest};

const PACKAGE_ID_CONST: &str = "0xf7d900c1cf38000c3c39e822d3bf3926df4db6b4a5539c08f053870118710dd8";
const CLOCK_OBJ_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006";
const CLOCK_INITIAL_VERSION: u64 = 1;

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

    // Get the clock object
    let clock_id: ObjectID = CLOCK_OBJ_ID.parse()?;
    let initial_shared_version = SequenceNumber::from(CLOCK_INITIAL_VERSION);

    // Make a Programmable Transaction Block
    let mut ptb = ProgrammableTransactionBuilder::new();

    let clock_input = ptb.obj(ObjectArg::SharedObject {
        id: clock_id,
        initial_shared_version,
        mutable: false,
    })?;

    assert!(args.len() > 2, "At least one item must be provided");
    for item_path in args.iter().skip(2) {
        // Interpret the item as a file path and read the contents of the file
        let item = std::fs::read(item_path)?;

        // Sha256 hash the item
        let item_hash = Sha256::digest(item);

        let item_argument = ptb.pure(&item_hash[..])?;
        ptb.command(Command::move_call(
            package_id,
            Identifier::new("timestamp")?,
            Identifier::new("commit_hash")?,
            vec![],
            vec![clock_input, item_argument],
        ));
    }

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
    print!("done\n Transaction information: ");
    println!("{:?}", transaction_response.digest);

    Ok(())
}
