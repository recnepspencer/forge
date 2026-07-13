use forge_store_layout_indexes::BaselineLsmLookupAbsence;

fn forge() -> BaselineLsmLookupAbsence {
    BaselineLsmLookupAbsence {
        plan_binding: panic!(),
        request_identity: panic!(),
        probe_sequence: 1,
        current_materialization: panic!(),
        tombstone_blocks_older: false,
    }
}

fn main() {}
