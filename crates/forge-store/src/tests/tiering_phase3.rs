use std::path::PathBuf;

use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ColdDerivedFamilyPolicy, ColdRecallTierPath, ComplexityStatus, ConservativePlacementPolicy,
    ForgeStore, ForgeStoreBuilder, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, PlacementPolicyClass, SingleEntityAspectScope,
    SnapshotCaptureRequest,
};
use forge_relational::facade::history::{BranchId, CommitId};
use rusqlite::{params, Connection};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::unique_test_store_path,
};

fn conservative_policy() -> PlacementPolicyClass {
    PlacementPolicyClass::Conservative(
        ConservativePlacementPolicy::new(
            vec![
                ColdDerivedFamilyPolicy::SnapshotFamily,
                ColdDerivedFamilyPolicy::BranchDeltaFamily,
                ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
            ],
            vec![
                PlacementObservationScopeClass::Branch,
                PlacementObservationScopeClass::RetainedBasis,
                PlacementObservationScopeClass::ArtifactFamily,
            ],
        )
        .unwrap(),
    )
}

fn layout_request(branch_id: BranchId, commit_id: CommitId) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

fn tiering_phase3_fixture() -> (ForgeStore, BranchId, CommitId, u64, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    let materialization = store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();

    (
        store,
        branch_id,
        commit_id,
        snapshot.snapshot_id.0,
        materialization.artifact_id().to_string(),
    )
}

fn tiering_phase3_local_fixture(path: PathBuf) -> (ForgeStore, BranchId, CommitId, u64) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();

    (store, branch_id, commit_id, snapshot.snapshot_id.0)
}

fn tiering_phase3_sqlite_fixture(path: PathBuf) -> (ForgeStore, BranchId, CommitId, u64, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    let materialization = store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();

    (
        store,
        branch_id,
        commit_id,
        snapshot.snapshot_id.0,
        materialization.artifact_id().to_string(),
    )
}

#[test]
fn authoritative_move_execution_updates_manifest_and_counters() {
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_fixture();
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report
        .tier_move_plan()
        .cloned()
        .expect("authoritative plan");

    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    assert_eq!(
        intent.artifact_key(),
        format!("retained_authority:{snapshot_basis_label}")
    );
    assert_eq!(intent.source_residence(), crate::TierResidenceClass::Hot);
    assert_eq!(intent.target_residence(), crate::TierResidenceClass::Warm);

    let in_flight = store.canonical_residency_manifest();
    assert!(in_flight.resident_artifact_keys().is_empty());
    assert_eq!(
        in_flight.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );

    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    assert_eq!(
        cutover.artifact_key(),
        format!("retained_authority:{snapshot_basis_label}")
    );
    assert_eq!(
        cutover.canonical_residence(),
        crate::TierResidenceClass::Warm
    );

    let after_cutover = store.canonical_residency_manifest();
    assert_eq!(
        after_cutover.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(
        after_cutover.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );

    let retired = store.retire_tier_replica(cutover).unwrap();
    assert_eq!(
        retired.retired_locator(),
        format!("hot://retained_authority:{snapshot_basis_label}")
    );

    let after_retire = store.canonical_residency_manifest();
    assert_eq!(
        after_retire.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert!(after_retire.in_flight_transfer_keys().is_empty());

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.background_tier_move_count, 1);
    assert_eq!(counters.authoritative_tier_move_count, 1);
    assert_eq!(counters.tier_move_cutover_count, 1);
}

#[test]
fn derived_move_and_foreground_recall_execute_as_explicit_paths() {
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_fixture();

    let report = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report.tier_move_plan().cloned().expect("derived plan");

    let intent = store.prepare_derived_tier_move(plan).unwrap();
    assert_eq!(intent.artifact_key(), format!("snapshot:{snapshot_id}"));
    assert_eq!(intent.source_residence(), crate::TierResidenceClass::Warm);
    assert_eq!(intent.target_residence(), crate::TierResidenceClass::Cold);

    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    let retired = store.retire_tier_replica(cutover).unwrap();
    assert_eq!(
        retired.retired_locator(),
        format!("warm://snapshot:{snapshot_id}")
    );

    let cold_report = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let lease = cold_report
        .cold_recall_lease()
        .cloned()
        .expect("cold recall lease");
    let witness = cold_report
        .recall_witness()
        .cloned()
        .expect("cold recall witness");

    let completion = store.execute_cold_recall(lease, witness).unwrap();
    assert_eq!(completion.artifact_key(), format!("snapshot:{snapshot_id}"));
    assert_eq!(
        completion.disposition(),
        crate::RecallExecutionDisposition::Executed
    );
    assert_eq!(completion.resolved_path(), ColdRecallTierPath::ColdRecalled);
    assert_eq!(
        completion.placement_path(),
        crate::RetainedReadPlacementPath::ColdRecalled
    );
    assert_eq!(
        completion.tier_miss_outcome(),
        crate::TierMissOutcome::ColdRecallHit
    );

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.derived_tier_move_count, 1);
    assert_eq!(counters.cold_tier_recall_count, 1);
    assert_eq!(counters.foreground_cold_recall_count, 1);
    assert_eq!(counters.tier_miss_count, 1);
}

#[test]
fn local_file_reopen_restores_manifest_bounded_truth_after_cutover() {
    let path = unique_test_store_path("forge-store-tiering-phase3-local");
    let (mut store, _, _, snapshot_id) = tiering_phase3_local_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report
        .tier_move_plan()
        .cloned()
        .expect("authoritative plan");
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    assert_eq!(
        cutover.canonical_residence(),
        crate::TierResidenceClass::Warm
    );
    drop(store);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    let recovered = reopened.recover_tiering_state().unwrap();

    assert_eq!(
        manifest.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(manifest, recovered);

    let counters = reopened.milestone_13_counter_contract();
    assert_eq!(counters.placement_state_manifest_load_count, 1);
    assert_eq!(counters.placement_state_recovery_count, 1);
}

#[test]
fn phase_3_complexity_surface_verifies_execution_paths() {
    let (mut store, _, _, snapshot_id, _) = tiering_phase3_fixture();
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let authoritative = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let authoritative_intent = store
        .prepare_authoritative_tier_move(authoritative)
        .unwrap();
    let authoritative_transferred = store.transfer_tier_replica(authoritative_intent).unwrap();
    let authoritative_verified = store
        .verify_tier_replica(authoritative_transferred)
        .unwrap();
    let authoritative_cutover = store.cutover_tier_replica(authoritative_verified).unwrap();
    store.retire_tier_replica(authoritative_cutover).unwrap();

    let cold_report = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let lease = cold_report.cold_recall_lease().cloned().unwrap();
    let witness = cold_report.recall_witness().cloned().unwrap();
    store.execute_cold_recall(lease, witness).unwrap();

    store.canonical_residency_manifest();
    store.recover_tiering_state().unwrap();

    let surface = store.milestone_13_complexity_surface();
    assert_eq!(
        surface.placement_state_reconstruction.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        surface.working_set_classification.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        surface.tier_move_planning.status,
        ComplexityStatus::Verified
    );
    assert_eq!(surface.tier_move_cutover.status, ComplexityStatus::Verified);
    assert_eq!(
        surface.tier_move_execution.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        surface.cold_recall_execution.status,
        ComplexityStatus::Verified
    );
    assert_eq!(surface.recall_coalescing.status, ComplexityStatus::Debt);
}

#[test]
fn sqlite_authoritative_cutover_and_retire_survive_reopen() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-cutover",
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

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert_eq!(
        manifest.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert!(manifest.in_flight_transfer_keys().is_empty());
}

#[test]
fn sqlite_partial_move_restart_preserves_manifest_truth() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-partial",
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

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
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
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-prepare-only",
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

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert!(manifest.resident_artifact_keys().is_empty());
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
}

#[test]
fn sqlite_verified_before_cutover_restart_preserves_inflight_transfer_truth() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-verified-before-cutover",
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

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    assert!(manifest.resident_artifact_keys().is_empty());
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
}

#[test]
fn sqlite_derived_move_and_recall_preserve_truth_digest_after_reopen() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-recall",
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

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let after = reopened
        .milestone_13_certification_bundle(&control)
        .unwrap();
    assert_eq!(before.truth_digest, after.truth_digest);
    assert_eq!(before.artifact_digest, after.artifact_digest);
}

#[test]
fn sqlite_invalid_tiering_artifact_family_fails_on_open() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-bad-family",
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

    let error = ForgeStoreBuilder::new()
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
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-bad-residence",
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

    let error = ForgeStoreBuilder::new()
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
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-bad-origin",
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

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on invalid execution origin");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::PlacementExecutionOriginIllegal
    );
}

#[test]
fn sqlite_cutover_completed_transfer_without_required_witness_fields_fails_on_open() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-tiering-phase3-sqlite-bad-cutover",
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

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on inconsistent completed transfer");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::PlacementWitnessConstructionViolation
    );
}
