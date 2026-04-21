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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-deferred-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_deferred(&local_path, local_compaction.declaration().id());

    let mut reopened_local = ForgeStoreBuilder::new()
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

    let sqlite_path = unique_test_sqlite_path("forge-store-m11-maintenance-deferred-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_deferred(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-cancelled-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_cancelled(&local_path, local_compaction.declaration().id());

    let mut reopened_local = ForgeStoreBuilder::new()
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

    let sqlite_path = unique_test_sqlite_path("forge-store-m11-maintenance-cancelled-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_cancelled(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-escalated-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_escalated(&local_path, local_compaction.declaration().id());

    let mut reopened_local = ForgeStoreBuilder::new()
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

    let sqlite_path = unique_test_sqlite_path("forge-store-m11-maintenance-escalated-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_escalated(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-readmission-stale-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());
    force_local_file_cancelled(&local_path, local_compaction.declaration().id());

    let mut reopened_local = ForgeStoreBuilder::new()
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
        unique_test_sqlite_path("forge-store-m11-maintenance-readmission-stale-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());
    force_sqlite_cancelled(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-readmission-fresh-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());

    let mut reopened_local = ForgeStoreBuilder::new()
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
        unique_test_sqlite_path("forge-store-m11-maintenance-readmission-fresh-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m11-maintenance-superseded-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let duplicate_id = "maintenance-compaction-superseded";
    let duplicate_batch = duplicate_compaction_batch(&batch, duplicate_id);
    let receipt = store.admit_maintenance_batch(duplicate_batch).unwrap();
    let (leader, duplicate) = equivalent_compaction_pair(&receipt, duplicate_id);
    drop(store);
    force_local_file_supersession_epoch(&path, duplicate.declaration().id(), 1);

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-budget-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
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

    let mut reopened_local = ForgeStoreBuilder::new()
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

    let sqlite_path = unique_test_sqlite_path("forge-store-m11-maintenance-budget-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
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

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m11-maintenance-latency-guard-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let compaction = admitted_compaction(&receipt);
    drop(store);
    force_local_file_high_latency_guard(path.as_path(), compaction.declaration().id(), 3);

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
    let path = unique_test_store_path("forge-store-m11-maintenance-starved-global-local");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(path.clone()),
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

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
    let local_path = unique_test_store_path("forge-store-m11-maintenance-global-debt-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_global_scope_escalated(
        local_path.as_path(),
        local_compaction.declaration().id(),
    );

    let mut reopened_local = ForgeStoreBuilder::new()
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

    let sqlite_path = unique_test_sqlite_path("forge-store-m11-maintenance-global-debt-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_global_scope_escalated(
        sqlite_path.as_path(),
        sqlite_compaction.declaration().id(),
    );

    let mut reopened_sqlite = ForgeStoreBuilder::new()
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
