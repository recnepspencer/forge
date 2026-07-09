use super::{
    BaselineLsmCounterObservation, BaselineLsmExecutionWitness, BaselineLsmLookupDisposition,
};

#[test]
fn baseline_lsm_execution_selects_memtable_before_sorted_run() {
    let witness = BaselineLsmExecutionWitness::seeded();

    assert_eq!(
        witness.lookup_disposition_for(43),
        BaselineLsmLookupDisposition::Memtable
    );
    assert_eq!(
        witness.lookup_disposition_for(42),
        BaselineLsmLookupDisposition::SortedRun
    );
    assert_eq!(
        witness.lookup_disposition_for(99),
        BaselineLsmLookupDisposition::NotFound
    );
}

#[test]
fn baseline_lsm_execution_observes_named_strategy_lanes() {
    let witness = BaselineLsmExecutionWitness::seeded();

    assert_eq!(
        witness.execute_lookup_latest_visible_record(43).counters(),
        BaselineLsmCounterObservation::new(1, 1, 0, 0, 0)
    );
    assert_eq!(
        witness.execute_manifest_publication().counters(),
        BaselineLsmCounterObservation::new(0, 0, 0, 2, 2)
    );
    assert_eq!(
        witness.execute_replay_wal_tail().counters(),
        BaselineLsmCounterObservation::new(0, 0, 1, 0, 1)
    );
}
