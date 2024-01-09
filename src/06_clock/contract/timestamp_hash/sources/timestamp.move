module timestamp_hash::timestamp {

    use sui::clock::{Self, Clock};
    use sui::event;

    struct TimestampEvent has copy, drop {
        wall_clock_time_ms: u64,
        document_hash: vector<u8>,
    }

    public fun commit_hash(clock: &Clock, document_hash: vector<u8>) {
        let event = TimestampEvent {
            wall_clock_time_ms: clock::timestamp_ms(clock),
            document_hash: document_hash,
        };
        event::emit(event);
    }

}