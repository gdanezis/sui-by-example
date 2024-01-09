# Example 06: Clock and Multi Transaction PTBs

Sui provides facilities to get the time a transaction is being executed. It also allows multiple commands within a single PTB as well as having pure arguments with data to calls. This example presents a simple smart contract that allows a caller to timestamps some data with the current time.

## Key concepts

* There is a global `Clock` type and singleton object (address `0x6` initial version 1). This object can be included as a function argument as read-only in order to read the current commit time.

* Passing the Clock as read-only allows multiple transactions using time to all run in parallel. Clock time is updated with every commit using the time reported by the leader for the commit in the consensus. It is guaranteed to be between two correct times. 

* Programmable Transaction Blocks allows multiple commands that use the same or different inputs to be executed atomically. This allows bulk execution at low latency and low complexity. In this example the clock object is shared between commands, but the second argument is unique to each.

* In example 05 we saw how to pass object reference (owned or shared) as arguments. This example shows how to pass *pure arguments* as parameters, that just represent input data.

## Setup

Follow the instructions of example 03 to setup a test network, a client connected to the test network, some test coins from the faucet and to upload the contract `timestamp_hash`.

You execute the example with:
```
$cargo run --bin 06_clock ~/.sui/sui_config/sui.keystore README.md 
```

The first argument is the key store of the wallet. The second (and optionally further ones) are files that are timestamped.

## Code details

On the Move side see `src/06_clock/contract/timestamp_hash/` for the smart contract:

* Note that the `commit_hash` function takes `clock: &sui::clock::Clock` as an argument and then we call `clock::timestamp_ms(clock)` to get the current time in milliseconds.

On the Rust side:

* This `commit_hash` function expects a `vector<u8>` and this is created as an argument in rust defined as `let item_argument = ptb.pure(&item_hash[..])?;`. The `pure` function on PTB structures is the way we define and then pass in pure arguments to Move commands.

## Security concerns

Time is not exact: it is the time of the leader that led to a commit in the consensus and its guaranteed to be between two correct timestamps. However, the exact time between two correct timestamp is down to the validator that choses it. So you must never rely on time as being entirely random or overly exact.

## Further resources and docs

* Documentation for Clock facilities: https://docs.sui.io/guides/developer/sui-101/access-time