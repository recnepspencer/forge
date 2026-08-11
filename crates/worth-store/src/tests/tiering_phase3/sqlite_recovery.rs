use crate::{
    ColdDerivedFamilyPolicy, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, WORTHStoreBuilder,
};

use super::world::{conservative_policy, tiering_phase3_sqlite_fixture};

#[test]
fn sqlite_authoritative_cutover_and_retire_survive_reopen() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-cutover",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    store.retire_tier_replica(cutover).unwrap();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert_eq!(
        manifest.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert!(manifest.in_flight_transfer_keys().is_empty());
}

#[test]
fn sqlite_partial_move_restart_preserves_manifest_truth() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-partial",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    store.cutover_tier_replica(verified).unwrap();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert_eq!(
        manifest.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
}

#[test]
fn sqlite_prepare_only_restart_preserves_inflight_transfer_truth() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-prepare-only",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    store.prepare_authoritative_tier_move(plan).unwrap();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert!(manifest.resident_artifact_keys().is_empty());
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
}

#[test]
fn sqlite_verified_before_cutover_restart_preserves_inflight_transfer_truth() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-verified-before-cutover",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    store.verify_tier_replica(transferred).unwrap();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert!(manifest.resident_artifact_keys().is_empty());
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
}

#[test]
fn sqlite_derived_move_and_recall_preserve_truth_digest_after_reopen() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-recall",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());
    let control = store.export_authoritative_records();

    let report = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    let intent = store.prepare_derived_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    store.retire_tier_replica(cutover).unwrap();

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    let before = store.milestone_13_certification_bundle(&control).unwrap();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let after = reopened
        .milestone_13_certification_bundle(&control)
        .unwrap();
    assert_eq!(before.truth_digest, after.truth_digest);
    assert_eq!(before.artifact_digest, after.artifact_digest);
}
