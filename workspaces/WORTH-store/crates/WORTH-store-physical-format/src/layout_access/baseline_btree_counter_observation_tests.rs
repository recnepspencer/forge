use super::{BaselineBTreeExecutionWitness, BaselineBTreeLookupBranch, BaselineBTreeReadShape};
use worth_store_budgets::S8PreExecutionPlanBinding;

const TEST_PLAN_BINDING: S8PreExecutionPlanBinding = S8PreExecutionPlanBinding::new(7, 11, 13, 17, 0);

#[test]
fn baseline_btree_execution_directs_lookup_by_separator_branch() {
    let witness = BaselineBTreeExecutionWitness::seeded();

    assert_eq!(
        witness
            .execute_separator_directed_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .branch(),
        BaselineBTreeLookupBranch::Left
    );
    assert_eq!(
        witness
            .execute_separator_directed_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(12).unwrap())
            .branch(),
        BaselineBTreeLookupBranch::Right
    );
}

#[test]
fn baseline_btree_execution_emits_exact_strategy_lane_classification() {
    let witness = BaselineBTreeExecutionWitness::seeded();

    assert_eq!(
        witness
            .execute_separator_directed_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .exact_counters()
            .point_lookups(),
        1
    );
    assert_eq!(
        witness
            .execute_separator_directed_range_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .exact_counters()
            .range_lookups(),
        1
    );
    assert_eq!(
        witness
            .execute_separator_directed_prefix_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
            .shape(),
        BaselineBTreeReadShape::PrefixLookup
    );
    assert_eq!(
        witness.execute_replay_recovery(TEST_PLAN_BINDING).exact_counters().maintenance_reads(),
        1
    );
}

#[test]
fn baseline_btree_execution_emits_exact_counter_witnesses_at_family_boundary() {
    let witness = BaselineBTreeExecutionWitness::seeded();
    let point = witness
        .execute_separator_directed_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
        .exact_counters();
    let range = witness
        .execute_separator_directed_range_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
        .exact_counters();
    let prefix = witness
        .execute_separator_directed_prefix_lookup(TEST_PLAN_BINDING, crate::PhysicalRecordSlot::from_raw(11).unwrap())
        .exact_counters();
    let publication = BaselineBTreeExecutionWitness::seeded()
        .execute_root_publication(TEST_PLAN_BINDING)
        .exact_counters();
    let recovery = witness.execute_replay_recovery(TEST_PLAN_BINDING).exact_counters();

    assert_eq!(point.page_touches(), 2);
    assert_eq!(point.index_probes(), 2);
    assert_eq!(point.key_comparisons(), 2);
    assert_eq!(point.bytes_read(), 8_192);
    assert_eq!(range.range_steps(), 1);
    assert_eq!(prefix.prefix_steps(), 1);
    assert_eq!(publication.publications(), 1);
    assert_eq!(publication.bytes_written(), 4_096);
    assert_eq!(recovery.maintenance_reads(), 1);
    assert_eq!(recovery.manifest_reads(), 1);
}
