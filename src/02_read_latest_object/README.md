# Example 02: Read a Sui object

The Sui blockchain state is a set of objects that are either owned by particular addresses or shared and may be used by anyone. A basic operation is reading the latest state of an object, finding the version it is at, its owner and seeing its contents in JSON.

## Key concepts

* In Sui the database is a set of *objects*. Each object has an *owner*, that is the only one that may use it; or is *shared* meaning that anyone may use it. All objects have an *object ID* which Sui ensures is unique (actually collision resistant).

* All objects are at a current *version* and every time they are involved in a transaction (besides explicit read-only operations) the version increases. Owned objects have to be referred by their current version when used in transactions, but shared objects are referred by the initial version at which they were shared (also recorded in their owner field).

* The owner is usually an *address* representing a public key, and owned objects may be used as part of transactions signed by the owner keys. More complex owners include multi-sig accounts where more than one address needs to sign, or zklogin accounts where a zero-knowledge proof of possession of an OAuth token is needed.

* All objects have a Move *type* that describes their layout, in terms of field types and names. These types come from the move system libraries, or from user-define smart contacts in the Move language. Printing an object in JSON requires its type, but the full nodes can do this for you.

* Objects contain the reference to the *previous transaction* that last modified them or created them.

## Setup

Running the example involves simply running the binary `02_read_latest_object` with one command line argument representing the ID of the object to look up.

```
$ cargo run --bin 02_read_latest_object 0x0001ecf89b1d54c4360d25d400da92db0a8a7f1f6f03ebacce5472cc752cee1a
Sui mainnet version: 1.15.2
Object version: 0x36e8682 Previous transaction: Some(TransactionDigest(6bA7E9bB39AMDtsCLZQCyjCwekgBDRCJHyvRAswJ7Zmp))
Owner: Account Address ( 0x00878369f475a454939af7b84cdd981515b1329f159a1aeb9bf0f8899e00083a )
Content:
type: 0x2::coin::Coin<0x2::sui::SUI>
balance: 1910320191
id: 0x0001ecf89b1d54c4360d25d400da92db0a8a7f1f6f03ebacce5472cc752cee1a
```

Note that we display the initial version at which a shared object was shared:

```
$ cargo run --bin 02_read_latest_object 0x6
Sui mainnet version: 1.15.2
Object version: 0x15b9e7c Previous transaction: Some(TransactionDigest(3bEdQhsTZv1Qz96YHELhM9UnPWRxNTLkNEYc2eESa8gH))
Shared at initial version: 0x1
Owner: Shared
Content:
type: 0x2::clock::Clock
id: 0x0000000000000000000000000000000000000000000000000000000000000006
timestamp_ms: 1704486884812
```

The above is the system clock object which is created (and shared) at genesis.

## Code details

This is a simple modification of the first example that calls the read API function `.read_api().get_object_with_options( ... )`. This function takes extensive parameters requesting different types of information about the object from the full node. Here due to laziness we ask for everything, but you probably want to be more selective in your own production code.
```
            SuiObjectDataOptions {
                show_type: true,
                show_owner: true,
                show_previous_transaction: true,
                show_display: true,
                show_content: true,
                show_bcs: true,
                show_storage_rebate: true,
            },
```

At the end we print a rendition of the object fields returned from the JSON representation.

## Security concerns

Note that the object or the rendition of the content are produced by the full node using the object and type information there. If the full node is malicious it can change them arbitrarily. This problem is mitigated by using a Sui light client.

## Further resources and docs

* The Rust [Read API](https://mystenlabs.github.io/sui/sui_sdk/apis/struct.ReadApi.html) is documented here.