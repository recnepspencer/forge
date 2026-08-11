use crate::{
    ColdDerivedFamilyPolicy, PlacementExecutionOrigin, PlacementObservationScopeClass,
    WORTHStoreBuilder,
};
use rusqlite::{params, Connection};

use super::world::{conservative_policy, tiering_phase3_sqlite_fixture};

#[test]
fn sqlite_invalid_tiering_artifact_family_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-bad-family",
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE tier_residency_records SET artifact_family = ?1",
            params!["not_a_family"],
        )
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on invalid tier artifact family");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::PlacementWitnessConstructionViolation
    );
}

#[test]
fn sqlite_invalid_tiering_residence_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-bad-residence",
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE tier_residency_records SET canonical_residence = ?1",
            params!["ultra_hot"],
        )
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on invalid tier residence");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::TierResidencyManifestViolation
    );
}

#[test]
fn sqlite_invalid_tiering_execution_origin_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-bad-origin",
    );
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_sqlite_fixture(path.clone());

    let report = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().unwrap();
    store.prepare_derived_tier_move(plan).unwrap();
    drop(store);

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE tier_transfer_records SET execution_origin = ?1",
            params!["teleport"],
        )
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on invalid execution origin");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::PlacementExecutionOriginIllegal
    );
}

#[test]
fn sqlite_residency_verification_label_drift_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-bad-verification-label",
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE tier_residency_records SET verification_label = ?1",
            params!["synthetic-verification-label"],
        )
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should reconstruct and reject drifted verification label");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(
        error.message().contains("verification label drifted"),
        "unexpected error: {}",
        error.message()
    );
}
