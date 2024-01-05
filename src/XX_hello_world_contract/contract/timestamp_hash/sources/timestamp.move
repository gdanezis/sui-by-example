module timestamp_hash::timestamp {

    // Imports
    use sui::object::{Self, UID, ID};
    use sui::tx_context::{TxContext};

    use sui::clock::{Self, Clock};
    use sui::event;

    struct TimestampStation has key, store {
        id: UID,
        // Next available sequence number for a timestamp
        sequence_number: u64,
    }

    struct TimestampEvent has copy, drop {
        station_id: ID,
        sequence_number: u64,
        wall_clock_time_ms: u64,
        document_hash: vector<u8>,
    }

    public fun create_timestamp_station(ctx: &mut TxContext) : TimestampStation {
        let station = TimestampStation {
            id: object::new(ctx),
            sequence_number: 0,
        };
        station
    }

    public fun commit_hash(clock: &Clock, station: &mut TimestampStation, document_hash: vector<u8>) {
        let event = TimestampEvent {
            station_id: object::uid_to_inner(&station.id),
            sequence_number: station.sequence_number,
            wall_clock_time_ms: clock::timestamp_ms(clock),
            document_hash: document_hash,
        };
        event::emit(event);
        station.sequence_number = station.sequence_number + 1;
    }

}