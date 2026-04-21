use super::*;

fn latest_branch_head(
    store: &ForgeStore,
) -> (
    forge_relational::facade::history::BranchId,
    forge_relational::facade::history::CommitId,
) {
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export
        .commit_envelopes
        .last()
        .expect("foreground maintenance fixture requires a canonical commit");
    (
        envelope.envelope.branch_context.clone(),
        envelope.envelope.commit.commit_id,
    )
}

fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(super::super::harness::fixtures::runtime::create_entity_commit(runtime, "alpha"))
}

#[test]
fn stable_basis_reads_report_waiting_on_escalated_same_locality_maintenance() {
    let path = unique_test_store_path("forge-store-m11-foreground-stable-basis");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
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

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
    let path = unique_test_store_path("forge-store-m11-foreground-continuation");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
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

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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

#[test]
fn recovered_intake_and_cold_start_boot_are_visible_before_execution() {
    let path = unique_test_store_path("forge-store-m11-recovered-intake");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_recovered(&path, &declaration_id);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened.milestone_11_maintenance_report();
    assert!(report.cold_start_boot.used_legacy_summary_backfill());
    assert_eq!(report.cold_start_boot.recovered_backlog_count(), 1);
    assert_eq!(report.recovered_intake.pending_recovered_count(), 1);
    assert_eq!(report.recovered_intake.lane_intake().len(), 1);
    assert_eq!(
        report.recovered_intake.lane_intake()[0].pending_recovered_count(),
        1
    );
    let counters = reopened.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_cold_start_boot_count, 1);
    assert_eq!(counters.maintenance_cold_start_summary_load_count, 0);
    assert_eq!(counters.maintenance_cold_start_legacy_backfill_count, 1);
    assert_eq!(counters.maintenance_cold_start_recovery_backlog_count, 1);
    assert_eq!(counters.maintenance_restart_recovered_count, 1);
    assert_eq!(counters.maintenance_cold_start_global_scan_count, 0);
}

#[test]
fn authoritative_commit_append_reports_reservation_violation_on_escalated_same_locality_maintenance(
) {
    let path = unique_test_store_path("forge-store-m11-foreground-write-append");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(initial).unwrap();
    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let update = update_entity_on_branch_with_commit(&mut runtime, entity_id, "foreground-write");
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let persisted = reopened.append_canonical_commit(update).unwrap();
    assert_eq!(
        persisted.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        persisted.foreground_isolation().violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}

#[test]
fn cursor_acknowledgment_reports_reservation_violation_on_escalated_same_locality_maintenance() {
    let path = unique_test_store_path("forge-store-m11-foreground-write-cursor");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let (branch_id, commit_id) = latest_branch_head(&store);
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let checkpoint = reopened
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();
    assert_eq!(
        checkpoint.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        checkpoint.foreground_isolation().violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}

#[test]
fn durable_publication_report_summarizes_foreground_write_interference() {
    let path = unique_test_store_path("forge-store-m11-publication-write-report");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(crate::DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    let durable_mutation_id = acknowledged.durable_mutation_id();
    let published = acknowledged.persisted().envelope().clone();
    let branch_id = published.branch_context.clone();
    let commit_id = published.commit.commit_id;
    let (mut store, _runtime) = durable.shutdown();

    store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id, commit_id))
        .unwrap();
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
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

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened
        .durable_publication_report(durable_mutation_id, Some(commit_id))
        .unwrap();
    let isolation = report
        .foreground_write_isolation()
        .expect("durable publication report should carry same-branch write isolation");
    assert_eq!(
        isolation.posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        isolation.violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}

#[test]
fn maintenance_recovery_report_summarizes_scheduler_backlog_counts() {
    let path = unique_test_store_path("forge-store-m11-maintenance-recovery-summary");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let active_declaration_count = receipt.admitted_declarations().len() as u64;
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_recovered(&path, &declaration_id);
    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened.maintenance_recovery_report().unwrap();
    assert_eq!(report.active_declaration_count(), active_declaration_count);
    assert_eq!(report.escalated_declaration_count(), 1);
    assert_eq!(report.recovered_backlog_count(), 1);
}

#[test]
fn milestone_11_certification_bundle_publishes_acceptance_artifacts() {
    let path = unique_test_store_path("forge-store-m11-certification-bundle");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_recovered(&path, &declaration_id);
    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let (branch_id, commit_id) = latest_branch_head(&reopened);
    let _ = reopened
        .read_stable_basis(stable_basis_request_for_store(
            &reopened, branch_id, commit_id,
        ))
        .unwrap();
    let control_export = reopened.export_authoritative_records();
    let bundle = reopened.milestone_11_certification_bundle(&control_export, &[]);

    assert!(!bundle.truth_digest.is_empty());
    assert!(!bundle.diagnostics_digest.is_empty());
    assert!(!bundle.failure_digest.is_empty());
    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.scheduler_topology_declared);
    assert_eq!(
        bundle.scheduler_topology_report.queue_family_count,
        bundle.maintenance_report.work_class_counts.len() as u64
    );
    assert!(bundle.scheduler_topology_report.queue_family_count > 0);
    assert!(
        bundle
            .scheduler_topology_report
            .has_foreground_reservation_pool
    );
    assert!(
        bundle
            .scheduler_topology_report
            .has_background_reservation_pool
    );
    assert_eq!(
        bundle.resource_budget_report.io_budget_units_reserved,
        bundle.counter_contract.maintenance_io_budget_units_reserved
    );
    assert_eq!(bundle.maintenance_interference_matrix.len(), 1);
    assert!(bundle.maintenance_interference_matrix[0].truth_visible_equal);
    assert!(bundle.maintenance_interference_matrix[0].foreground_interference_count >= 1);
    assert_eq!(bundle.debt_escalation_report.escalated_declaration_count, 1);
    assert_eq!(bundle.maintenance_report.recovered_declaration_count, 1);
}
