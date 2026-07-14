use worth_store_layout_indexes::BaselineLsmLookupAbsence;

fn worth() -> BaselineLsmLookupAbsence {
    BaselineLsmLookupAbsence {
        probe_sequence: 1,
        tombstone_blocks_older: false,
    }
}

fn main() {}
