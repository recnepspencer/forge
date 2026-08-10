use crate::{
    ColdDerivedFamilyPolicy, ColdRecallTierPath, ComplexityStatus, PlacementBoundArtifactRef,
    PlacementExecutionOrigin, PlacementObservationScopeClass,
};

use super::world::{conservative_policy, tiering_phase3_fixture};

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
