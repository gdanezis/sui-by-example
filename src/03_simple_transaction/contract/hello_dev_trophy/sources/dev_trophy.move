module hello_dev_trophy::dev_trophy {

    // Part 1: Imports
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use std::option::{Self, Option};
    use sui::event;

    const ASSERT_ERR: u64 = 400;

    struct SuiDevTrophy has key, store {
        id: UID,
        seq_from_station: Option<u64>,
        trophy_sender: Option<address>,
    }

    struct TrophyStation has key, store {
        id: UID,
        next_tropy_seq: u64,
    }

    struct AwardEvent has copy, drop {
        trophy_seq: u64,
        trophy_sender: address,
    }

    fun init(ctx: &mut TxContext) {
        let station = TrophyStation {
            id: object::new(ctx),
            next_tropy_seq: 0,
        };

        transfer::share_object(station);
    }

    public fun self_award_trophy(ctx: &mut TxContext) {
        let trophy = SuiDevTrophy {
            id: object::new(ctx),
            seq_from_station: option::none(),
            trophy_sender: option::none(),
        };

        transfer::transfer(trophy, tx_context::sender(ctx));
    }

    public fun stamp_trophy(station: &mut TrophyStation, trophy: &mut SuiDevTrophy, ctx: &mut TxContext) {
        let TrophyStation { id: _ , next_tropy_seq } = station;
        let SuiDevTrophy { id: _ , seq_from_station, trophy_sender } = trophy;

        assert!(option::is_none(seq_from_station) , ASSERT_ERR);
        
        *seq_from_station = option::some(*next_tropy_seq);
        *trophy_sender = option::some(tx_context::sender(ctx));

        event::emit(AwardEvent {
            trophy_sender: tx_context::sender(ctx),
            trophy_seq: *next_tropy_seq,
        });

        *next_tropy_seq = *next_tropy_seq + 1;
    }

    public fun drop_trophy(trophy : SuiDevTrophy) {
        let SuiDevTrophy { id, seq_from_station: _ , trophy_sender: _ } = trophy;
        object::delete(id);
    }

}