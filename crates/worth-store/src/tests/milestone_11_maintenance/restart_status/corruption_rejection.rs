use super::super::*;
use super::compaction_receipt::equivalent_compaction_pair;

#[test]
fn corrupted_queue_summary_is_rejected_on_reopen() {
    let duplicate_id = "maintenance-compaction-corrupted-summary";
    let path = unique_test_store_path("worth-store-m11-maintenance-corrupted-summary");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let duplicate_batch = duplicate_compaction_batch(&batch, duplicate_id);
    let receipt = store.admit_maintenance_batch(duplicate_batch).unwrap();
    let (_leader, duplicate) = equivalent_compaction_pair(&receipt, duplicate_id);
    let _ = store.start_maintenance_declaration(&duplicate).unwrap_err();
    drop(store);

    let raw = std::fs::read(&path).expect("store file should exist");
    let mut state: crate::backend::records::StoreState =
        serde_json::from_slice(&raw).expect("store state should decode");
    let summary = state
        .maintenance_queue_summary_records
        .values_mut()
        .find(|record| record.summary.coalesced_count() > 0)
        .expect("coalesced queue summary should exist");
    summary.summary = crate::MaintenanceQueueSummary::new(
        summary.summary.lane_key().clone(),
        summary.summary.admitted_count() + 1,
        summary.summary.reserved_count(),
        summary.summary.deferred_count(),
        summary.summary.active_quantum_count(),
        0,
        summary.summary.cancelled_superseded_count(),
        std::collections::BTreeMap::new(),
        std::collections::BTreeMap::new(),
        std::collections::BTreeMap::new(),
    );
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("corrupted store state should write");

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should reject corrupted maintenance queue summaries");
    assert!(error.message().contains("maintenance queue summary"));
}
