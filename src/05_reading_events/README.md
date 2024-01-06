# Example 05: Events and their uses

There are two distinct but related things users want to know about the Sui blockchain:

* They want to query things about the *current state* of the blockchain, ie the state within live owned and shared objects or smart contracts. We call these state properties of the chain. An example is the current balance of SUI within a coin object.
* They want to query things about the *history of the state* of the Sui blockchain. we call these trace properties of the chain. For example they want to know all the flows of SUI between accounts. 

For state properties we already saw in example 02 how to query the meta-data and content of objects. *Events* allow smart contracts to allow following trace properties of smart contract executions. When an app wants to query what happens on chain over time - either in the past or real-time - events are usually the tool of choice.

## Key concepts

* Move *events* are normal structures defined within Move modules. A smart contract can `emit` an event which appears in a log of events if the execution is successful, but does not appear in case the execution aborts (even after the `emit` call).

* Events are cheap: they do not require gas for storage costs as they are not stored as objects in the Sui state. They cannot be read or queries within smart contracts, but they can be both queried and subscribed to by apps. This example illustrates this.

## Setup

This example queries events on mainnet, so no setup or command line arguments required besides compiling the examples, and running it by:
```
cargo run --bin 05_reading_events
```

## Code details

* The key call is to the events API:
```
    let events = sui_mainnet
        .event_api().query_events(
            EventFilter::MoveModule { package: PACKAGE_ID_CONST.parse()?, module: Identifier::new("dev_trophy")? },
            None, None, false,
        ).await?;
```

* Different [filters are supported](https://docs.sui.io/guides/developer/sui-101/using-events#filtering-event-queries) but the filter presented will get all events relating to a Move module ensuring completeness. If you are building an app that needs to stay in sync with the state of the chain you probably want that.

## Security concerns

The event API relies on the full node to get events. So you need to trust the full node or run your own. The Sui light client allows you to authenticate events associated with transactions, but does not allow by default to check completeness of an event stream (you have to build this yourself, for example with a sequence number as illustrated in this example.)

## Further resources and docs

* The actual queries supported by the `query_events` API are documented here: https://docs.sui.io/guides/developer/sui-101/using-events#filtering-event-queries

