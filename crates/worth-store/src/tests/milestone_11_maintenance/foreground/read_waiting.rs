use super::super::*;
use super::branch_head::latest_branch_head;

#[test]
fn stable_basis_reads_report_waiting_on_escalated_same_locality_maintenance() {
    let path = unique_test_store_path("worth-store-m11-foreground-stable-basis");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let (branch_id, commit_id) = latest_branch_head(&reopened);
    let handle = reopened
        .read_stable_basis(stable_basis_request_for_store(
            &reopened, branch_id, commit_id,
        ))
        .unwrap();

    assert_eq!(
        handle.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::WaitedOnMaintenance
    );
    assert_eq!(
        handle.foreground_isolation().wait_dependency(),
        Some(crate::ForegroundWaitDependency::MaintenanceCutover)
    );
    let counters = reopened.milestone_11_counter_contract();
    assert!(counters.maintenance_foreground_interference_count >= 1);
    assert!(counters.maintenance_foreground_wait_on_cutover_count >= 1);
}

#[test]
fn continuation_plans_report_waiting_on_escalated_same_locality_maintenance() {
    let path = unique_test_store_path("worth-store-m11-foreground-continuation");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let (branch_id, commit_id) = latest_branch_head(&store);
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let (branch_id, commit_id) = latest_branch_head(&reopened);
    let basis = reopened
        .read_stable_basis(stable_basis_request_for_store(
            &reopened,
            branch_id.clone(),
            commit_id,
        ))
        .unwrap();
    let plan = reopened
        .plan_cursor_continuation(crate::CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
            basis,
            crate::ContinuationBatchBudget::new(
                crate::FetchWidth::new(8),
                crate::MaxBatchItems::new(8),
                crate::MaxCoveredCommits::new(8),
                crate::MaxMaterializedBytes::new(4_096),
                crate::MaxSupportRowsPerBatch::new(16),
            ),
        ))
        .unwrap();

    assert_eq!(
        plan.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::WaitedOnMaintenance
    );
    assert_eq!(
        plan.foreground_isolation().wait_dependency(),
        Some(crate::ForegroundWaitDependency::MaintenanceCutover)
    );
}
