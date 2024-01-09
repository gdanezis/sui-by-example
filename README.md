# Sui by Example

## Introduction

The repo contains a few examples of Sui applications written using the Rust SDK. The examples come with `README.md` files explaining step-by-step key Sui and blockchain contexts, references to more documentation, as well as instructions for configuring the environments and running them.

## Outline

- [01](/src/01_read_latest_checkpoint/): How to read basic data from the Sui network.
- [02](/src/02_read_latest_object/): How to read object meta-data and contents.
- [03](/src/03_simple_transaction/): How to write a Move smart contract, and execute a transaction on it.
- [04](/src/04_shared_object_transaction/): How to pass arguments to transactions and use shared objects.
- [05](/src/05_reading_events/): How to use and read events emitted by smart contracts.
- [06](/src/06_clock/): How to use the clock to get real time and make a timestamp service.
- ... TBD ...

## Installation

You will need a working rust installation with cargo and all. You probably also need an [installation of Sui](https://docs.sui.io/guides/developer/getting-started/sui-install) to follow after example 03.