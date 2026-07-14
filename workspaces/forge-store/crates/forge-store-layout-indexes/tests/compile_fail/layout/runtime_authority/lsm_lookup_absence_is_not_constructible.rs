use forge_store_layout_indexes::BaselineLsmLookupAbsence;

fn forge() -> BaselineLsmLookupAbsence {
    BaselineLsmLookupAbsence {
        probe_sequence: 1,
        tombstone_blocks_older: false,
    }
}

fn main() {}
