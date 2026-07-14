use super::{page, segment};
use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_layout_indexes::{
    btree_lookup_execution_cases, layout_read_runtime, BTreeLookupExecutionView,
    BTreeSeparatorPartitionDenial, BaselineBTreeExecutionDenial, BaselineBTreeReadSource,
    PageLookupRequest,
};
use worth_store_physical_format::PhysicalRecordSlot;
use worth_store_security::admitted_tenant_page_security_scope_for_layout_partition_test;
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, baseline_btree_probe_slot,
    deterministic_baseline_btree_read_source, deterministic_corrupt_leaf_btree_read_source,
    deterministic_left_partition_violation_btree_read_source,
    deterministic_noncanonical_leaf_btree_read_source,
    deterministic_right_partition_violation_btree_read_source,
    deterministic_stale_child_btree_read_source,
};

#[test]
fn ordinary_runtime_observes_every_declared_btree_lookup_execution_case() {
    let observed = scenarios()
        .into_iter()
        .map(|(source, slot)| execute(source, slot).case_id().name())
        .collect::<std::collections::BTreeSet<_>>();
    let declared = btree_lookup_execution_cases()
        .map(|case| case.name())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(observed, declared);
}

#[test]
fn separator_partition_damage_is_denied_before_absence_can_be_issued() {
    for (source, slot, expected) in [
        (
            deterministic_noncanonical_leaf_btree_read_source(),
            baseline_btree_probe_slot(),
            BTreeSeparatorPartitionDenial::LeafSlotsNotCanonical,
        ),
        (
            deterministic_left_partition_violation_btree_read_source(),
            baseline_btree_probe_slot(),
            BTreeSeparatorPartitionDenial::LeftChildCrossesSeparator,
        ),
        (
            deterministic_right_partition_violation_btree_read_source(),
            physical_slot(15),
            BTreeSeparatorPartitionDenial::RightChildPrecedesSeparator,
        ),
    ] {
        assert!(matches!(
            execute(source, slot).view(),
            BTreeLookupExecutionView::Denied(
                BaselineBTreeExecutionDenial::SeparatorPartition(actual)
            ) if *actual == expected
        ));
    }
}

fn scenarios() -> [(BaselineBTreeReadSource, PhysicalRecordSlot); 7] {
    [
        (
            deterministic_baseline_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_baseline_btree_read_source(),
            physical_slot(15),
        ),
        (
            deterministic_stale_child_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_corrupt_leaf_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_noncanonical_leaf_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_left_partition_violation_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_right_partition_violation_btree_read_source(),
            physical_slot(15),
        ),
    ]
}

fn execute(
    source: BaselineBTreeReadSource,
    slot: PhysicalRecordSlot,
) -> worth_store_layout_indexes::BTreeLookupExecutionOutcome {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    layout_read_runtime()
        .execute_page_lookup(PageLookupRequest::new(
            &catalog,
            security.witnesses(),
            segment(7),
            page(9),
            slot,
            PreExecutionBudgetEnvelope::foreground_default(),
            source,
        ))
        .expect("hostile B-tree source must reach owner execution")
}

fn physical_slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}
