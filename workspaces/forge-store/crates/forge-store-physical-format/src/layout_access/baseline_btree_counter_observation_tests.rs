use super::{
    BaselineBTreeCounterObservation, BaselineBTreeExecutionWitness, BaselineBTreeLookupBranch,
};

#[test]
fn baseline_btree_execution_directs_lookup_by_separator_branch() {
    let witness = BaselineBTreeExecutionWitness::seeded();

    assert_eq!(
        witness
            .execute_separator_directed_lookup(crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .branch(),
        BaselineBTreeLookupBranch::Left
    );
    assert_eq!(
        witness
            .execute_separator_directed_lookup(crate::PhysicalRecordSlot::from_raw(12).unwrap())
            .branch(),
        BaselineBTreeLookupBranch::Right
    );
}

#[test]
fn baseline_btree_execution_observes_named_strategy_lanes() {
    let witness = BaselineBTreeExecutionWitness::seeded();

    assert_eq!(
        witness
            .execute_separator_directed_lookup(crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .counters(),
        BaselineBTreeCounterObservation::new(1, 1, 0, 0)
    );
    assert_eq!(
        witness.execute_replay_recovery().counters(),
        BaselineBTreeCounterObservation::new(0, 0, 0, 1)
    );
}
