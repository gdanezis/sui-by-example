# Example 03: First Move smart contract and Sui transaction

In this example we look at our first Move smart contract and how to execute a simple transaction on the Sui network. As part of this example we will see how to operate a local command line Sui client, a local test network, how to get test Sui coins, how to direct the explorer to the local network and how to execute transactions on it.

## Key concepts

* *Smart contracts* define the structure of objects in the Sui database and are written in the Move programming language. Move was specifically designed to write smart contracts in a safe but expressive way. A smart contract defines modules with structures of objects on chain, as well as events that a contract can emit. Contracts also define functions that create, mutate or delete objects of the types defined, and other object types in the Sui database. 

* *Transactions* call functions of smart contract modules, optionally with parameters that include existing objects owned or other call data. In our example here we do not pass any parameters to keep things simple but examples later will illustrate call parameters. 

* *Owned objects* are owned by one address and may only be used in transactions signed by that address. Coin objects used to pay for gas are owned objects. *Shared objects* can be used in transactions by anyone.

* All transactions cost a small amount of *gas* to run, paid using a Sui coin associated with a transactions. On mainnet you need real Sui coins to execute transactions (that is the purpose of SUI!) but on the local testnet you can request some test gas to be sent to an address using a gas *faucet*.


## Setup

* To initialize a local wallet and local network we follow the [instructions in the docs to install Sui](https://docs.sui.io/guides/developer/getting-started/sui-install) and [start a local network](https://docs.sui.io/guides/developer/getting-started/local-network). Broadly speaking we need to [clone the `mainnet` branch of the sui repository](https://github.com/MystenLabs/sui/tree/mainnet), and then start a local network through the command:
```
$ RUST_LOG="off,sui_node=info" cargo run --bin sui-test-validator

...

Fullnode RPC URL: http://127.0.0.1:9000
Faucet URL: http://127.0.0.1:9123

```
* After compilation a local test Sui network runs with 4 nodes. The full node RPC local address can be used to configure the [Sui explorer](https://suiexplorer.com/) (top right for the network selector) to get data from the local network. You should be also to see checkpoints being created with system transactions every few seconds.

* The Sui local client can be accessed after Sui was installed. Configure it to point to the local test network and check your own address:
```
$ sui client new-env --alias local --rpc http://127.0.0.1:9000
$ sui client active-address
0xaa14199277e2376f40727c829e4f5809d285dd4ec3dc6329250098465d01e9bc
```

* Your local address can be used to get test coins from the faucet:
```
$ curl --location --request POST 'http://127.0.0.1:9123/gas' \
--header 'Content-Type: application/json' \
--data-raw '{
    "FixedAmountRequest": {
        "recipient": "0xaa14199277e2376f40727c829e4f5809d285dd4ec3dc6329250098465d01e9bc"
    }
}'
```
* Check whether you received coins via `$ sui client gas`.

* You can the smart contract on the test network by doing into the `sui-by-example/src/03_simple_transaction/contract/hello_dev_trophy` directory, and using the `sui move build` than `sui publish` commands. Full details on how to publish packages can be found in this [guide on publishing modules](https://docs.sui.io/guides/developer/first-app/publish).
```
$ sui client publish --gas-budget 2000000000

│ Created Objects:                                                                                              │
│  ┌──                                                                                                          │
│  │ ObjectID: 0x3d987e06da12d9e20bc8c09dae2f04e14df96d75ecf0542c910f026c5842670c                               │
│  │ Sender: 0xaa14199277e2376f40727c829e4f5809d285dd4ec3dc6329250098465d01e9bc                                 │
│  │ Owner: Shared                                                                                              │
│  │ ObjectType: 0x74246e5987c3b20aeed158bd01620f86b50602c4970a33c3b3570018b74286a9::dev_trophy::TrophyStation  │
│  │ Version: 3                                                                                                 │
│  │ Digest: FQVQRrcNcufk9vnbxPqAWHirXQaryG9idrja6SuFofts                                                       │
│  └──                                                                                                          │

...

│ Published Objects:                                                                                            │
│  ┌──                                                                                                          │
│  │ PackageID: 0x74246e5987c3b20aeed158bd01620f86b50602c4970a33c3b3570018b74286a9                              │
│  │ Version: 1                                                                                                 │
│  │ Digest: J7mKMDoeAKLdRybtw5gcnSgsG3Pp6Bass2cm7tUZGzRh                                                       │
│  | Modules: dev_trophy                                                                                        │
│  └──                      
```

* The package ID (here `0x74246e5987c3b20aeed158bd01620f86b50602c4970a33c3b3570018b74286a9`) defines this smart contract, and has to be used in the code file for the variable `PACKAGE_ID_CONST`.


* Note that the local sui client stores keys in `~/.sui/sui_config/sui.keystore` which we will use as a command line argument.

* Once the package ID variable is updated in the code you can run the example via:
```
$ cargo run --bin 03_simple_transaction ~/.sui/sui_config/sui.keystore
```


## Code details

This is the first Move smart contract we are looking at, so lets start with the move code under `contracts`.

* First off what is key idea of the contract: (1) anyone can call `self_award_trophy` and make a `SuiDevTrophy` object that is owned by their account. Then there exists a singleton shared object of type `TrophyStation`, and if you use the function `stamp_trophy` you can stamp your `SuiDevTrophy` with a sequence number, and imprint your address in it. You can only stamp a trophy once, and when this happens an event `AwardEvent` is emitted that contains the ever increasing sequence number and your address. Needless to say having your address associated with a small number is very cool (and I got zero!).

* The module name and package name are at the start at `module hello_dev_trophy::dev_trophy`. However, module names are substituted when the module is published by the object ID in which the module lives, so you will never again see the nice name `hello_dev_trophy`. On Sui mainnet it is instead `0x279525274aa623ef31a25ad90e3b99f27c8dbbad636a6454918855c81d625abc`.

* The `struct` definitions for `SuiDevTrophy`, `TrophyStation` and `AwardEvent` are not surprising in terms of the data they contain. You can find out more about `UID`, which is guaranteed to be globally unique and cannot be dropped in the [docs for `object`](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/object.md). The `key`, `store`, `copy`, `drop` are abilities that are [documented here](https://move-language.github.io/move/abilities.html). 

* The `init` function is executed once when the package is published, and makes a singleton `TrophyStation` object, which is then made into a shared object using `transfer::share_object`. Since no other constructor exists this is the only one there will ever be, and since its not drop nor there is a destructor it will live for ever!

* The `self_award_trophy` takes no parameters, creates a `SuiDevTrophy` with the sender address inside and sends it to the sender. Note that it would be much better to just return it and use a *Programmable Transaction Block* to create it, then transfer it. But in this example we want to just use simple move calls (which are PTBs with 1 command only). This is the function we call in the rust code.

* We will discuss `stamp_trophy` and `drop_trophy` in a later example.

The rust code calls the `self_award_trophy` to award itself a `SuiDevTrophy`. It illustrates a number of important concepts.

* The code takes a path to the key store as a command line argument, usually `~/.sui/sui_config/sui.keystore` and opens it as a `FileBasedKeystore`. This is not entirely documented, but works.

* We then make a `SuiClient` using `SuiClientBuilder`, but this time we point it to `http://127.0.0.1:9000` which is the full node of the local test network we are running with the contract published (see setup above).

* The `.read_api().get_owned_objects` is used to get all objects owned by our local client, and then we filter out the first Sui coin object that can be used as a gas object using the handy `is_gas_coin`. To make this work we have to download the types of the objects using `SuiObjectDataOptions::new().with_type()` above.

* This is where things get exciting: we define a TPB using `ProgrammableTransactionBuilder::new()` and then use a single call with no type arguments or call arguments:

```
    ptb.move_call(
        package_id,
        Identifier::new("dev_trophy")?,
        Identifier::new("self_award_trophy")?,
        vec![],
        vec![],
    )?;
```

* We then add some metadata about the gas cost, gas coin (as identified above) and sign the transaction. The gas price is just the reference gas price given by `read_api().get_reference_gas_price()`. We sign the transaction using the local client key `keystore.sign_secure( ... )`.

* The finale involves executing the transaction using the quorum driver API, using `sui_local.quorum_driver_api().execute_transaction_block( ... )`. The `ExecuteTransactionRequestType::WaitForLocalExecution` option is the most conservative and waits for local execution to be finished before returning (rather than returning as soon as the transaction is final). The [Rust SDK docs for Quorum Driver](https://mystenlabs.github.io/sui/sui_sdk/apis/struct.QuorumDriverApi.html) say a little more about it.

## Security concerns

As before the reads from the full node on the read API are not authenticated.

## Further resources and docs


* How to initialize Move packages is documented on the [guide to witting a first app](https://docs.sui.io/guides/developer/first-app/write-package), but really it came down to this:
```
$ sui move new hello_dev_trophy
```

* Best way to start learning Sui move is the Sui foundation tutorial: https://github.com/sui-foundation/sui-move-intro-course

* Move documentation is very handy:
  - The base Move book: https://move-language.github.io/move/
  - Basic move tutorial: https://github.com/move-language/move/blob/main/language/documentation/tutorial/README.md
  - The Sui Move book by example: https://examples.sui.io/
  - The Sui Move framework reference is here: https://github.com/MystenLabs/sui/tree/main/crates/sui-framework/docs

* Docs on running a local network and a client to connect to it, and getting faucet coins: 
https://docs.sui.io/guides/developer/getting-started/local-network

* The package and module `hello_dev_trophy::dev_trophy` runs on mainnet today. The [transaction that uploaded](https://suiexplorer.com/txblock/7qhRBU33k7JqoCceQRReTb98kbUyRpajEraaJ16BFduP?network=mainnet) it is here.can you get a `SuiDevTrophy` under your address on mainnet?