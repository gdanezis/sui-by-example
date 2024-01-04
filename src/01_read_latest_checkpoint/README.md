# Example 01: Hello Sui!

This first example is as simple as possible, and will help you setup your development environment, and ensure you can import the Rust Sui SDK into one of your projects. In a nutshell, it fetches the latest checkpoint sequence number from the mainnet, downloads its summary, and prints the total number of Programmable Transaction Blocks (PTBs) processed so far by the network!

## Key concepts

* The Sui blockchain outputs an ever growing sequence of *checkpoints*, that contain *transactions* executed, and their outcomes called *effects*.

* The Sui blockchain is operated by *validators* that accept and collectively execute and sequence transactions. Clients are outside entities that can observe the operation of the network, and submit transactions to be executed. All operations on Sui are transparent and can be observed, and audited.

* This example illustrates a client that observes meta-data about the latest checkpoint.

## Setup

Using the Sui Rusk SDK is easy: just include the packages from the Sui github repository you need as dependencies within your `cargo.toml` file:

```
[dependencies]
...
sui-sdk = { git = "https://github.com/mystenlabs/sui", package = "sui-sdk"}
sui-keys = { git = "https://github.com/mystenlabs/sui", package = "sui-keys"}
shared-crypto = { git = "https://github.com/mystenlabs/sui", package = "shared-crypto"}
...
```
More instructions of setting up Sui and the Rust SDK can be found on the [page of the sui-sdk crate](https://github.com/MystenLabs/sui/tree/main/crates/sui-sdk).

The first example can then be built and run as usual for Rust using `cargo run`:
```
$ cargo run --bin 01_read_latest_checkpoint
Sui mainnet version: 1.15.2
Latest mainnet checkpoint: 22682910
Network total transactions: 1038053278
```

## Code details

* The `SuiClientBuilder` allows you to build a `SuiClient` connected to either the mainnet, testnet, devnet or to a local network. Here we use a full node connected to the mainnet as an example. For reference this is `https://fullnode.mainnet.sui.io:443`.

* The Sui client supports many categories of APIs including the `ReadApi` we use here to get the sequence number of the latest checkpoint using `get_latest_checkpoint_sequence_number` and then the full summary using `get_checkpoint`. The full documentation of the Sui [Read API](https://mystenlabs.github.io/sui/sui_sdk/apis/struct.ReadApi.html) is available, as well as the [Cargo docs of the full Rust SDK](https://mystenlabs.github.io/sui/sui_sdk/index.html).

* Each checkpoint in Sui contains metadata about the current epoch as well as the network lifetime. It also contains cryptographic hashes to the previous checkpoints (hence making a blockchain), its contents, and the full state of the current Sui objects. The [documentation](https://mystenlabs.github.io/sui/sui_json_rpc_types/struct.Checkpoint.html) for the `Checkpoint` structure contains more information.

## Security concerns

Note that the reads performed in this example are from a public full node, and are not verified for correctness. This means that you need to trust the full node to trust the values read. Chose your full node provider carefully, or operate your own full node if you need the highest assurance. 

Note that Sui supports light nodes that read and verify read values using cryptographic signatures and hashes.

## Further resources and docs

* Rust SDK [documentation](https://docs.sui.io/references/rust-sdk). 
* Rust SDK [cargo docs reference](https://mystenlabs.github.io/sui/sui_sdk/index.html).
* Sui-sdk [crate](https://github.com/MystenLabs/sui/tree/main/crates/sui-sdk)