use super::super::*;
use super::compaction_receipt::admitted_compaction;

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
