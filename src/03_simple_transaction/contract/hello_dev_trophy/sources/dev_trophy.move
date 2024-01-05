module hello_dev_trophy::dev_trophy {

    // Imports
    
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use std::option::{Self, Option};
    use sui::event;

    const ASSERT_ERR: u64 = 400;

    // Structures

    // A trophy you can own. It starts empty, but you can stamp it 
    // with a sequence number and the sender address using the trophy station.
    struct SuiDevTrophy has key, store {
        id: UID,
        seq_from_station: Option<u64>,
        trophy_sender: Option<address>,
    }

    // A single trophy station. It stamps trophies with sequence numbers and sender addresses.
    // Low sequence numbers are very cool.
    struct TrophyStation has key, store {
        id: UID,
        next_tropy_seq: u64,
    }

    // An event that is emitted when a trophy is stamped.
    struct AwardEvent has copy, drop {
        trophy_seq: u64,
        trophy_sender: address,
    }

    // Initialization

    // This function is called when the contract is initialized.
    // It creates a TrophyStation object and makes it into a shared object.
    fun init(ctx: &mut TxContext) {
        let station = TrophyStation {
            id: object::new(ctx),
            next_tropy_seq: 0,
        };

        transfer::share_object(station);
    }

    // Functions

    // Everyone can call this function to award a trophy to himself.
    public fun self_award_trophy(ctx: &mut TxContext) {
        let trophy = SuiDevTrophy {
            id: object::new(ctx),
            seq_from_station: option::none(),
            trophy_sender: option::none(),
        };

        // This sends the trophy to the caller.
        transfer::transfer(trophy, tx_context::sender(ctx));
    }

    // Stamp a trophy with a sequence number and the sender address.
    public fun stamp_trophy(station: &mut TrophyStation, trophy: &mut SuiDevTrophy, ctx: &mut TxContext) {
        let TrophyStation { id: _ , next_tropy_seq } = station;
        let SuiDevTrophy { id: _ , seq_from_station, trophy_sender } = trophy;

        // This checks that the trophy has not been stamped yet.
        assert!(option::is_none(seq_from_station) , ASSERT_ERR);

        *seq_from_station = option::some(*next_tropy_seq);
        *trophy_sender = option::some(tx_context::sender(ctx));

        event::emit(AwardEvent {
            trophy_sender: tx_context::sender(ctx),
            trophy_seq: *next_tropy_seq,
        });

        *next_tropy_seq = *next_tropy_seq + 1;
    }

    // This function drops an unwanted trothy to reclaim storage fee.
    public fun drop_trophy(trophy : SuiDevTrophy) {
        let SuiDevTrophy { id, seq_from_station: _ , trophy_sender: _ } = trophy;
        // This deletes the UID which otherwise cannot be dropped.
        object::delete(id);
    }

}