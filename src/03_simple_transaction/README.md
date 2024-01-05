# Example 03: First Move smart contract and Sui transaction

In this example we look at our first Move smart contract and how to execute a simple transaction on the Sui network. As part of this example we will see how to operate a local command line Sui client, a local test network, how to get test Sui coins, how to direct the explorer to the local network and how to execute transactions on it.

## Key concepts

* *Smart contracts* define the structure of objects in the Sui database and are written in the Move programming language. Move was specifically designed to write smart contracts in a safe but expressive way. A smart contract defines modules with structures of objects on chain, as well as events that a contract can emit. Contracts also define functions that create, mutate or delete objects of the types defined, and other object types in the Sui database. 

* *Transactions* call functions of smart contract modules, optionally with parameters that include existing objects owned or other call data. In our example here we do not pass any parameters to keep things simple but examples later will illustrate call parameters. 

* *Owned objects* are owned by one address and may only be used in transactions signed by that address. Coin objects used to pay for gas are owned objects. *Shared objects* can be used in transactions by anyone.


## Setup

* To initialize a local wallet and local network we follow the [instructions in the docs](https://docs.sui.io/guides/developer/getting-started/local-network). Broadly speaking we need to [clone the `mainnet` branch of the sui repository](https://github.com/MystenLabs/sui/tree/mainnet), and then start a local network through the command:
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


## Security concerns


## Further resources and docs


* How to initialize Move packages is documented on the [guide to witting a first app](https://docs.sui.io/guides/developer/first-app/write-package), but really it came down to this:
```
$ sui move new hello_dev_trophy
```

* Move documentation is very handy:
  - The base Move book: https://move-language.github.io/move/
  - The Sui Move book by example: https://examples.sui.io/

* Docs on running a local network and a client to connect to it, and getting faucet coins: 
https://docs.sui.io/guides/developer/getting-started/local-network

* The package and module `hello_dev_trophy::dev_trophy` runs on mainnet today. The [transaction that uploaded](https://suiexplorer.com/txblock/7qhRBU33k7JqoCceQRReTb98kbUyRpajEraaJ16BFduP?network=mainnet) it is here.can you get a `SuiDevTrophy` under your address on mainnet?