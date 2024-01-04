use shared_crypto::intent::Intent;
use std::{any::Any, env};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponseOptions},
    types::{
        base_types::{ObjectID, SequenceNumber},
        transaction::ObjectArg,
        Identifier,
    },
    types::{
        object::Owner,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Argument, Command, Transaction, TransactionData},
    },
    SuiClientBuilder,
};

const PACKAGE_ID_CONST: &str = "0x28c13a4d04b02febc5713d67ab8709aa3d40232c043646e22bc58447379d2e20";
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
    println!("My address: {}", my_address);
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

    let existing_station = coins_response.data.iter().find(|obj| {
        let type_name = obj
            .data
            .as_ref()
            .unwrap()
            .type_
            .as_ref()
            .unwrap()
            .to_string();
        type_name == format!("{}::timestamp::TimestampStation", package_id)
    });

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

    // Create a new timestamp station, or use existing one

    let (station, save) = if let Some(existing_object) = existing_station {
        let object = existing_object.data.as_ref().unwrap();
        (
            ptb.obj(ObjectArg::ImmOrOwnedObject(object.object_ref()))?,
            false,
        )
    } else {
        (
            ptb.command(Command::move_call(
                package_id,
                Identifier::new("timestamp")?,
                Identifier::new("create_timestamp_station")?,
                vec![],
                vec![],
            )),
            true,
        )
    };

    // Timestamp hash 1

    let hash1 = ptb.pure(vec![0xAAu8; 32])?;

    ptb.command(Command::move_call(
        package_id,
        Identifier::new("timestamp")?,
        Identifier::new("commit_hash")?,
        vec![],
        vec![clock_input, station, hash1],
    ));

    // Timestamp hash 2

    let hash2 = ptb.pure(vec![0xBBu8; 32])?;

    ptb.command(Command::move_call(
        package_id,
        Identifier::new("timestamp")?,
        Identifier::new("commit_hash")?,
        vec![],
        vec![clock_input, station, hash2],
    ));

    // Send the new timestamp station to ourselves for future use

    if save {
        let recipient_address = ptb.pure(my_address)?;
        ptb.command(Command::TransferObjects(vec![station], recipient_address));
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
