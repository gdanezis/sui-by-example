# Example 04: Transaction with parameters including owned and shared objects

In example 03 we show how to define a contract and how to execute a transaction on a test network with no parameters. In this example we call other function of the same contract that take as parameters both a shared object and an owned object.

## Key concepts

* Transactions take parameters that can either by *object references* in the Sui blockchain or *pure arguments* that pass in data into the execution. 

* Owned objects need to be specified by their *full object reference* which includes a triples or their object ID, their current version and the current object digest. It is common to query a full node you trust (such as your own) to get the latest reference of an object by object ID.

* Shared object inputs are specified only by *object ID and the initial version* they were shared at. A more exact version or digest may not be available as they might be chaining as the transaction is being constructed.

## Setup

* This example assumes you have ran the example 03 on a test network and you have now a shared `TrophyStation` and you own one or more `SuiDevTrophy` objects in a local Sui network. Or that you have modified the example 03 to generate your own `SuiDevTrophy` object on mainnet.

* You will need to modify the example to include the package ID of the contact in your own network, as well as the object ID and initial version at which the `TrophyStation` was shared. Or you may use the given (but commented out constant for the Sui mainnet).

* You will also need the object ID of a `SuiDevTrophy` you want to stamp with a sequence number. The example takes two command line arguments: (1) your local key file (see example 03) and (2) the object ID of the object to stamp.
```
$ cargo run --bin 04_shared_object_transaction ~/.sui/sui_config/sui.keystore 0x0d69a64f09fd9587588bf744345da5b84c0c42a4f7d73e4b79348c4f8be9e721
```

## Code details

Let's talk about the functions `stamp_trophy` and `drop_trophy` in Move package and module `hello_dev_trophy::dev_trophy` from example 03.

* The function `stamp_trophy` takes the singleton `TrophyStation` as a first argument, and an owned object of type `SuiDevTrophy` and updates its fields with a sequence number. This can happen only once so it uses an `assert!( ... )` to abort if there is already a sequence number. Note that both objects are passed as mutable references ie. `&mut ...` so the function can change their contents but not drop them.

* Note that the module defines an event of type `AwardEvent` that is emitted when a valid stamp is awarded to a trophy using the system function `event::emit`. We discuss events and their used in the next example. 

* The function `drop_trophy` cleans up unwanted and unloved `SuiDevTrophy` objects you may own. Note that `SuiDevTrophy` has no `drop` ability due to `UID` not having the drop ability. This is a typical issue for all objects with `UID`. So to allow users to drop it we have to make a specific function that deconstructs the object, and manually calls the destructor of `UID`:

```
    public fun drop_trophy(trophy : SuiDevTrophy) {
        let SuiDevTrophy { id, seq_from_station: _ , trophy_sender: _ } = trophy;
        // This deletes the UID which otherwise cannot be dropped.
        object::delete(id);
    }
```

This example Rust code follows the setup of example 03, and with a few notable exceptions when it comes to calling the function `stamp_trophy`. 

* We use a call to `.read_api().get_object_with_options( ... )`, with minimal default options to look up the `SuiDevTrophy` object with the given object ID. Then we use `trophy_object.object_ref()` to get the reference corresponding to the latest version of the object in the test Sui network. It is typical to at least refresh the references to local copies of the objects from the network before a transactions. IT IS IMPERATIVE TO NEVER CONCURRENTLY USE THE SAME OBJECT REFERENCE IN DIFFERENT TRANSACTION.

* We invoke the `obj` method of `ProgrammableTransactionBuilder` with either `ObjectArg::ImmOrOwnedObject` for owned objects or `ObjectArg::SharedObject` to construct arguments to the Move call. Then we use the `command` method to define a command that takes these arguments in the order the `stamp_trophy` expects them:

```
    ptb.command(Command::move_call(
        package_id,
        Identifier::new("dev_trophy")?,
        Identifier::new("stamp_trophy")?,
        vec![],
        vec![trophy_station_input, trophy_input],
    ));
```

* This is a Programmable Transaction Block (PTB) with one command. In future examples we will see richer examples with multiple commands, but note that you can add more calls to stamp more objects by repeating this command, using different references for the owned object but the same reference for the shared object.

## Security concerns

As before the reads from the full node on the read API are not authenticated.

## Further resources and docs

* The very first `SuiDevTrophy` stamped on Sui mainnet is here:
https://suiexplorer.com/object/0xe8f0b5294f051b6dace6a9241bf1024c96973d3215ba238604ce9974c39b3461
