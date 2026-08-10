use super::super::*;
use super::compaction_receipt::admitted_compaction;

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
