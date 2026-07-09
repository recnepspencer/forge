use super::*;

fn admitted_compaction(
    receipt: &crate::MaintenanceAdmissionReceipt,
) -> crate::AdmittedMaintenanceDeclaration {
    receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("compaction declaration")
        .clone()
}

fn equivalent_compaction_pair(
    receipt: &crate::MaintenanceAdmissionReceipt,
    duplicate_id: &str,
) -> (
    crate::AdmittedMaintenanceDeclaration,
    crate::AdmittedMaintenanceDeclaration,
) {
    let duplicate = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| declaration.declaration().id().as_str() == duplicate_id)
        .expect("duplicate compaction declaration should exist")
        .clone();
    let leader = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            declaration.declaration().id() != duplicate.declaration().id()
                && declaration.descriptor().equivalence_key()
                    == duplicate.descriptor().equivalence_key()
        })
        .expect("leader compaction declaration should exist")
        .clone();
    (leader, duplicate)
}

#[test]
fn deferred_plan_outcome_is_persisted_and_operator_visible_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-deferred-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_deferred(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let deferred = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Deferred
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Deferred)
    );
    assert_eq!(
        local_status.pending_reason(),
        Some("maintenance work was deferred pending an operator-visible signal")
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.deferred_declaration_count, 1);
    assert_eq!(local_report.cancelled_declaration_count, 0);

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-deferred-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_deferred(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let deferred = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Deferred
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Deferred)
    );
    assert_eq!(
        sqlite_status.pending_reason(),
        Some("maintenance work was deferred pending an operator-visible signal")
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.deferred_declaration_count, 1);
    assert_eq!(sqlite_report.cancelled_declaration_count, 0);
}

#[test]
fn cancelled_plan_outcome_is_persisted_before_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-cancelled-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_cancelled(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let cancelled = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Cancelled)
    );
    assert_eq!(
        local_status.pending_reason(),
        Some("maintenance descriptor is stale and must be cancelled before execution")
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.cancelled_declaration_count, 1);
    assert_eq!(local_report.deferred_declaration_count, 0);
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_freshness_rejection_count,
        1
    );
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_rejected_plan_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-cancelled-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_cancelled(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let cancelled = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Cancelled)
    );
    assert_eq!(
        sqlite_status.pending_reason(),
        Some("maintenance descriptor is stale and must be cancelled before execution")
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.cancelled_declaration_count, 1);
    assert_eq!(sqlite_report.deferred_declaration_count, 0);
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_freshness_rejection_count,
        1
    );
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_rejected_plan_count,
        1
    );
}

#[test]
fn escalated_plan_outcome_is_visible_after_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-escalated-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_escalated(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Escalated)
    );
    assert!(local_status
        .foreground_impact()
        .borrowed_foreground_reservation());
    assert!(local_status.foreground_impact().foreground_wait_required());
    assert!(local_status
        .foreground_impact()
        .cutover_dependency_required());
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.escalated_declaration_count, 1);
    assert_eq!(local_report.foreground_borrowed_declaration_count, 1);
    assert_eq!(local_report.foreground_waited_declaration_count, 1);
    assert_eq!(local_report.cutover_dependency_declaration_count, 1);

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-escalated-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_escalated(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Escalated)
    );
    assert!(sqlite_status
        .foreground_impact()
        .borrowed_foreground_reservation());
    assert!(sqlite_status.foreground_impact().foreground_wait_required());
    assert!(sqlite_status
        .foreground_impact()
        .cutover_dependency_required());
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.escalated_declaration_count, 1);
    assert_eq!(sqlite_report.foreground_borrowed_declaration_count, 1);
    assert_eq!(sqlite_report.foreground_waited_declaration_count, 1);
    assert_eq!(sqlite_report.cutover_dependency_declaration_count, 1);
}

#[test]
fn recovered_stale_work_is_cancelled_during_readmission_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-readmission-stale-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());
    force_local_file_cancelled(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let cancelled = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
    );
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.rejected_recovered_declaration_count, 1);

    let sqlite_path =
        unique_test_sqlite_path("worth-store-m11-maintenance-readmission-stale-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());
    force_sqlite_cancelled(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let cancelled = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
    );
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.rejected_recovered_declaration_count, 1);
}

#[test]
fn recovered_fresh_work_is_readmitted_before_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-readmission-fresh-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.readmitted_recovered_declaration_count, 1);
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_restart_readmission_count,
        1
    );

    let sqlite_path =
        unique_test_sqlite_path("worth-store-m11-maintenance-readmission-fresh-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.readmitted_recovered_declaration_count, 1);
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_restart_readmission_count,
        1
    );
}

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

#[test]
fn budget_denial_defers_without_reserving_any_dimension() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-budget-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_high_demand(
        local_path.as_path(),
        local_compaction.declaration().id(),
        99,
        99,
        99,
        9,
    );

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let deferred = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert!(status.resource_budget_grant().is_none());
    assert_eq!(status.reservation_transition(), None);
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_io_budget_units_reserved,
        0
    );
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_quantum_exhaustion_count,
        0
    );
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_deferred_plan_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-budget-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_high_demand(
        sqlite_path.as_path(),
        sqlite_compaction.declaration().id(),
        99,
        99,
        99,
        9,
    );

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let deferred = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert!(status.resource_budget_grant().is_none());
    assert_eq!(status.reservation_transition(), None);
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_cpu_budget_units_reserved,
        0
    );
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_quantum_exhaustion_count,
        0
    );
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_deferred_plan_count,
        1
    );
}

#[test]
fn latency_guard_budget_denial_defers_work() {
    let path = unique_test_store_path("worth-store-m11-maintenance-latency-guard-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let compaction = admitted_compaction(&receipt);
    drop(store);
    force_local_file_high_latency_guard(path.as_path(), compaction.declaration().id(), 3);

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let deferred = reopened
        .start_maintenance_declaration(&compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let status = reopened
        .maintenance_status(compaction.declaration().id())
        .unwrap();
    assert!(status.resource_budget_grant().is_none());
    assert_eq!(status.reservation_transition(), None);
}

#[test]
fn starved_deferred_lane_is_visible() {
    let path = unique_test_store_path("worth-store-m11-maintenance-starved-global-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let duplicate_id = "maintenance-compaction-starved";
    let duplicate_batch = same_lane_distinct_compaction_batch(&batch, duplicate_id);
    let receipt = store.admit_maintenance_batch(duplicate_batch).unwrap();
    let duplicate = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| declaration.declaration().id().as_str() == duplicate_id)
        .expect("duplicate declaration should exist")
        .clone();
    let leader = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            declaration.declaration().id() != duplicate.declaration().id()
                && declaration.descriptor().lane_key() == duplicate.descriptor().lane_key()
        })
        .expect("same-lane leader should exist")
        .clone();
    drop(store);
    force_local_file_high_demand(path.as_path(), leader.declaration().id(), 99, 99, 99, 9);
    force_local_file_high_demand(path.as_path(), duplicate.declaration().id(), 99, 99, 99, 9);

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let _ = reopened.start_maintenance_declaration(&leader).unwrap_err();
    let _ = reopened
        .start_maintenance_declaration(&duplicate)
        .unwrap_err();

    let second_status = reopened
        .maintenance_status(duplicate.declaration().id())
        .unwrap();
    assert_eq!(
        second_status.starvation_status(),
        Some(crate::MaintenanceStarvationStatus::DeferredLanePressure)
    );
    let report = reopened.milestone_11_maintenance_report();
    assert_eq!(report.starved_lane_count, 1);
    let counters = reopened.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_starvation_trigger_count, 1);
}

#[test]
fn explicit_global_scope_debt_lane_is_visible() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-global-debt-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_global_scope_escalated(
        local_path.as_path(),
        local_compaction.declaration().id(),
    );

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert!(local_status.explicit_global_scope_debt());
    assert!(matches!(
        local_status.lane_key().locality_scope(),
        crate::MaintenanceLocalityScope::StoreGlobalLocalityScope
    ));
    assert_eq!(
        reopened_local
            .milestone_11_maintenance_report()
            .store_global_scope_declaration_count,
        1
    );
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_store_global_scope_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-global-debt-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_global_scope_escalated(
        sqlite_path.as_path(),
        sqlite_compaction.declaration().id(),
    );

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert!(sqlite_status.explicit_global_scope_debt());
    assert!(matches!(
        sqlite_status.lane_key().locality_scope(),
        crate::MaintenanceLocalityScope::StoreGlobalLocalityScope
    ));
    assert_eq!(
        reopened_sqlite
            .milestone_11_maintenance_report()
            .store_global_scope_declaration_count,
        1
    );
}

#[test]
fn tier_work_enters_scheduler_as_milestone_11_container_lanes() {
    let placement_id = "maintenance-tier-placement-proposal";
    let move_id = "maintenance-tier-move-execution";
    let (mut placement_store, _) = build_maintenance_ready_store();
    let placement_batch = tier_placement_batch("tier-placement-batch", placement_id);
    let placement_receipt = placement_store
        .admit_maintenance_batch(placement_batch)
        .unwrap();
    let placement = placement_receipt.admitted_declarations()[0].clone();

    let completed = placement_store
        .start_maintenance_declaration(&placement)
        .unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("tier_placement_container:"));
    let placement_status = placement_store
        .maintenance_status(placement.declaration().id())
        .unwrap();
    assert_eq!(
        placement_status.work_class(),
        crate::MaintenanceWorkClass::TierPlacementProposal
    );
    assert_eq!(
        placement_status.tier_work_container_class(),
        Some(crate::TierWorkContainerClass::TierPlacementProposal)
    );

    let (mut move_store, _) = build_maintenance_ready_store();
    let move_batch = tier_move_batch("tier-move-batch", move_id, false);
    let move_receipt = move_store.admit_maintenance_batch(move_batch).unwrap();
    let tier_move = move_receipt.admitted_declarations()[0].clone();

    let completed = move_store
        .start_maintenance_declaration(&tier_move)
        .unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("tier_move_container:"));
    let move_status = move_store
        .maintenance_status(tier_move.declaration().id())
        .unwrap();
    assert_eq!(
        move_status.work_class(),
        crate::MaintenanceWorkClass::TierMoveExecution
    );
    assert_eq!(
        move_status.tier_work_container_class(),
        Some(crate::TierWorkContainerClass::TierMoveExecution)
    );
    let counters = move_store.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_tier_work_execute_count, 1);
    assert_eq!(counters.maintenance_cross_locality_escalation_count, 0);
    assert_eq!(counters.maintenance_global_scope_fallback_count, 0);
}

#[test]
fn late_maintenance_families_enter_shared_scheduler_container_lanes() {
    let (mut derived_rebuild_store, _) = build_maintenance_ready_store();
    let derived_rebuild_batch = derived_family_rebuild_batch(
        "derived-family-rebuild-batch",
        "maintenance-derived-family-rebuild",
    );
    let derived_rebuild_receipt = derived_rebuild_store
        .admit_maintenance_batch(derived_rebuild_batch)
        .unwrap();
    let derived_rebuild = derived_rebuild_receipt.admitted_declarations()[0].clone();

    let completed = derived_rebuild_store
        .start_maintenance_declaration(&derived_rebuild)
        .unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("derived_family_rebuild_container:"));
    let derived_rebuild_status = derived_rebuild_store
        .maintenance_status(derived_rebuild.declaration().id())
        .unwrap();
    assert_eq!(
        derived_rebuild_status.work_class(),
        crate::MaintenanceWorkClass::DerivedFamilyRebuild
    );
    assert_eq!(
        derived_rebuild_status.debt_family(),
        Some(crate::MaintenanceDebtFamily::RebuildDebt)
    );

    let (mut snapshot_store, _) = build_maintenance_ready_store();
    let snapshot_batch =
        snapshot_refresh_batch("snapshot-refresh-batch", "maintenance-snapshot-refresh");
    let snapshot_receipt = snapshot_store
        .admit_maintenance_batch(snapshot_batch)
        .unwrap();
    let snapshot = snapshot_receipt.admitted_declarations()[0].clone();

    let completed = snapshot_store
        .start_maintenance_declaration(&snapshot)
        .unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("snapshot_refresh_container:"));
    let snapshot_status = snapshot_store
        .maintenance_status(snapshot.declaration().id())
        .unwrap();
    assert_eq!(
        snapshot_status.work_class(),
        crate::MaintenanceWorkClass::SnapshotRefresh
    );
    assert_eq!(
        snapshot_status.debt_family(),
        Some(crate::MaintenanceDebtFamily::SnapshotDebt)
    );
    assert_eq!(
        snapshot_store
            .milestone_11_counter_contract()
            .maintenance_snapshot_debt_units,
        1
    );

    let (mut replication_store, _) = build_maintenance_ready_store();
    let replication_batch = replication_preparation_batch(
        "replication-preparation-batch",
        "maintenance-replication-preparation",
    );
    let replication_receipt = replication_store
        .admit_maintenance_batch(replication_batch)
        .unwrap();
    let replication = replication_receipt.admitted_declarations()[0].clone();

    let completed = replication_store
        .start_maintenance_declaration(&replication)
        .unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("replication_preparation_container:"));
    let replication_status = replication_store
        .maintenance_status(replication.declaration().id())
        .unwrap();
    assert_eq!(
        replication_status.work_class(),
        crate::MaintenanceWorkClass::ReplicationPreparation
    );
    assert_eq!(
        replication_status.debt_family(),
        Some(crate::MaintenanceDebtFamily::ReplicationPreparationDebt)
    );
    let counters = replication_store.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_replication_prep_debt_units, 1);
    assert_eq!(counters.maintenance_tier_work_execute_count, 0);
    assert_eq!(counters.maintenance_global_scope_fallback_count, 0);
    assert_eq!(counters.maintenance_store_global_scope_count, 0);

    let (mut audit_store, _) = build_maintenance_ready_store();
    let audit_batch = maintenance_audit_batch("maintenance-audit-batch", "maintenance-audit");
    let audit_receipt = audit_store.admit_maintenance_batch(audit_batch).unwrap();
    let audit = audit_receipt.admitted_declarations()[0].clone();

    let completed = audit_store.start_maintenance_declaration(&audit).unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("maintenance_audit_container:"));
    let audit_status = audit_store
        .maintenance_status(audit.declaration().id())
        .unwrap();
    assert_eq!(
        audit_status.work_class(),
        crate::MaintenanceWorkClass::MaintenanceAudit
    );
    assert_eq!(audit_status.debt_family(), None);
    assert_eq!(
        audit_store
            .milestone_11_counter_contract()
            .maintenance_global_scope_fallback_count,
        0
    );
}

#[test]
fn explicit_cross_locality_tier_debt_is_observable_without_global_fallback() {
    let (mut store, _) = build_maintenance_ready_store();
    let batch = tier_move_batch(
        "tier-cross-locality-batch",
        "maintenance-tier-cross-locality",
        true,
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let tier_move = receipt.admitted_declarations()[0].clone();

    let completed = store.start_maintenance_declaration(&tier_move).unwrap();
    assert!(completed
        .last_completed_phase()
        .starts_with("tier_move_container:"));
    let counters = store.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_tier_work_execute_count, 1);
    assert_eq!(counters.maintenance_cross_locality_escalation_count, 1);
    assert_eq!(counters.maintenance_global_scope_fallback_count, 0);
    assert_eq!(counters.maintenance_store_global_scope_count, 0);
}
