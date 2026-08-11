use crate::{
    ColdDerivedFamilyPolicy, PlacementExecutionOrigin, PlacementObservationScopeClass,
    WORTHStoreBuilder,
};
use rusqlite::{params, Connection};

use super::world::{conservative_policy, tiering_phase3_sqlite_fixture};

#[test]
fn sqlite_completed_cutover_with_inconsistent_residency_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-inconsistent-cutover-residency",
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE tier_residency_records SET canonical_residence = ?1",
            params![crate::TierResidenceClass::Hot.label()],
        )
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should reject completed transfer inconsistent with residency");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(
        error
            .message()
            .contains("inconsistent with canonical residency truth"),
        "unexpected error: {}",
        error.message()
    );
}

#[test]
fn sqlite_cutover_completed_transfer_without_required_witness_fields_fails_on_open() {
    let path = super::super::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-tiering-phase3-sqlite-bad-cutover",
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
    let intent = store.prepare_derived_tier_move(plan).unwrap();
    store.transfer_tier_replica(intent).unwrap();
    drop(store);

    let connection = Connection::open(&path).unwrap();
    connection
        .execute("UPDATE tier_transfer_records SET cutover_completed = 1", [])
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on inconsistent completed transfer");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::PlacementWitnessConstructionViolation
    );
}
