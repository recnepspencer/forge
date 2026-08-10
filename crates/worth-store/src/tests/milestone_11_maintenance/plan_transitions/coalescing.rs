use super::super::*;
use super::compaction_receipt::equivalent_compaction_pair;

#[test]
fn same_equivalence_work_coalesces_in_same_lane() {
    let (mut store, batch) = build_maintenance_ready_store();
    let duplicate_id = "maintenance-compaction-coalesced";
    let duplicate_batch = duplicate_compaction_batch(&batch, duplicate_id);
    let receipt = store.admit_maintenance_batch(duplicate_batch).unwrap();
    let (_leader, duplicate) = equivalent_compaction_pair(&receipt, duplicate_id);

    let cancelled = store.start_maintenance_declaration(&duplicate).unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );

    let status = store
        .maintenance_status(duplicate.declaration().id())
        .unwrap();
    assert_eq!(
        status.coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
    );
    assert!(status
        .supersession_source()
        .expect("coalesced work should cite a leader")
        .contains("coalesced with"));
    let report = store.milestone_11_maintenance_report();
    assert_eq!(report.coalesced_work_count, 1);
    assert_eq!(
        store
            .milestone_11_counter_contract()
            .maintenance_coalesced_work_count,
        1
    );
}

#[test]
fn superseded_work_cancels_before_reservation() {
    let path = unique_test_store_path("worth-store-m11-maintenance-superseded-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let duplicate_id = "maintenance-compaction-superseded";
    let duplicate_batch = duplicate_compaction_batch(&batch, duplicate_id);
    let receipt = store.admit_maintenance_batch(duplicate_batch).unwrap();
    let (leader, duplicate) = equivalent_compaction_pair(&receipt, duplicate_id);
    drop(store);
    force_local_file_supersession_epoch(&path, duplicate.declaration().id(), 1);

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let cancelled = reopened.start_maintenance_declaration(&leader).unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let status = reopened
        .maintenance_status(leader.declaration().id())
        .unwrap();
    assert_eq!(
        status.coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CancelledAsSuperseded)
    );
    assert!(status
        .supersession_source()
        .expect("superseded work should cite a source")
        .contains("epoch 1"));
    assert_eq!(
        reopened
            .milestone_11_maintenance_report()
            .cancelled_superseded_work_count,
        1
    );
    assert_eq!(
        reopened
            .milestone_11_counter_contract()
            .maintenance_cancelled_superseded_work_count,
        1
    );
}
