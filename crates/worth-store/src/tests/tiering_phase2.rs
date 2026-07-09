use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ColdDerivedFamilyPolicy, ComplexityStatus, ConservativePlacementPolicy, WORTHStore,
    WORTHStoreBuilder, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, PlacementPolicyClass, SingleEntityAspectScope,
    SnapshotCaptureRequest,
};
use worth_relational::facade::history::{BranchId, CommitId};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

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

fn tiering_phase2_fixture() -> (WORTHStore, BranchId, CommitId, u64, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
fn branch_observation_and_summary_are_scope_typed() {
    let (store, branch_id, _, _, _) = tiering_phase2_fixture();

    let window = store
        .observe_working_set(PlacementObservationScopeClass::Branch, &branch_id.0)
        .unwrap();
    assert_eq!(window.scope_class(), PlacementObservationScopeClass::Branch);
    assert_eq!(window.scope_key(), branch_id.0.as_str());
    assert_eq!(window.observed_artifact_keys().len(), 1);

    let summary = store
        .summarize_placement_demand(PlacementObservationScopeClass::Branch, &branch_id.0)
        .unwrap();
    assert_eq!(
        summary.scope_class(),
        PlacementObservationScopeClass::Branch
    );
    assert_eq!(summary.scope_key(), branch_id.0.as_str());
    assert_eq!(
        summary.classification_verdict(),
        crate::HotnessClassificationVerdict::Hot
    );

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.working_set_observation_window_count, 2);
    assert_eq!(counters.working_set_reclassification_count, 1);
}

#[test]
fn retained_basis_scope_lowers_to_warm_authoritative_plan() {
    let (store, _, _, snapshot_id, _) = tiering_phase2_fixture();
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();

    assert_eq!(
        report.demand_summary().classification_verdict(),
        crate::HotnessClassificationVerdict::Warm
    );
    assert_eq!(
        report
            .retained_range_plan()
            .expect("retained range plan")
            .target_residence(),
        crate::TierResidenceClass::Warm
    );
    assert_eq!(
        report
            .tier_move_plan()
            .expect("authoritative plan")
            .target_residence(),
        crate::TierResidenceClass::Warm
    );
    assert!(report.rejection().is_none());

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.tier_move_plan_count, 1);
    assert_eq!(counters.tier_move_rejection_count, 0);
}

#[test]
fn derived_snapshot_family_lowers_to_cold_and_foreground_move_rejects() {
    let (store, _, _, snapshot_id, _) = tiering_phase2_fixture();
    let snapshot_id = snapshot_id.to_string();

    let background = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    assert_eq!(
        background
            .family_local_plan()
            .expect("family-local plan")
            .target_residence(),
        crate::TierResidenceClass::Cold
    );
    assert_eq!(
        background
            .tier_move_plan()
            .expect("derived tier move plan")
            .target_residence(),
        crate::TierResidenceClass::Cold
    );

    let foreground = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id,
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    assert!(matches!(
        foreground.rejection(),
        Some(crate::TierMoveRejection::IllegalExecutionOrigin { .. })
    ));

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.tier_move_plan_count, 1);
    assert_eq!(counters.tier_move_rejection_count, 1);
}

#[test]
fn resident_and_cold_lease_planning_stay_distinct() {
    let (store, branch_id, _, snapshot_id, _) = tiering_phase2_fixture();

    let resident = store
        .plan_resident_read_lease(
            PlacementBoundArtifactRef::authoritative_branch_head(branch_id.0.clone()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    assert!(resident.resident_lease().is_some());
    assert!(resident.cold_recall_lease().is_none());
    assert_eq!(
        resident.retained_read_path(),
        Some(crate::RetainedReadPlacementPath::HotResident)
    );
    assert_eq!(
        resident.tier_miss_outcome(),
        Some(crate::TierMissOutcome::ResidentHit)
    );

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    assert!(cold.resident_lease().is_none());
    assert!(cold.cold_recall_lease().is_some());
    assert!(cold.cold_recall_plan().is_some());
    assert!(cold.recall_witness().is_some());
    assert_eq!(
        cold.retained_read_path(),
        Some(crate::RetainedReadPlacementPath::ColdRecalled)
    );
    assert_eq!(
        cold.tier_miss_outcome(),
        Some(crate::TierMissOutcome::ColdRecallHit)
    );

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.hot_tier_resident_read_count, 1);
    assert_eq!(counters.warm_tier_resident_read_count, 0);
    assert_eq!(counters.cold_tier_recall_count, 0);
    assert_eq!(counters.foreground_cold_recall_count, 0);
    assert_eq!(counters.tier_miss_count, 0);
}

#[test]
fn broadened_recall_plan_is_explicit_and_counted() {
    let (store, _, _, _, layout_artifact_id) = tiering_phase2_fixture();

    let plan = store
        .plan_broadened_recall(
            ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
            PlacementObservationScopeClass::ArtifactFamily,
            ColdDerivedFamilyPolicy::Milestone6LayoutFamily.label(),
            vec![layout_artifact_id.clone(), layout_artifact_id.clone()],
            PlacementExecutionOrigin::Background,
        )
        .unwrap();

    assert_eq!(
        plan.scope_class(),
        PlacementObservationScopeClass::ArtifactFamily
    );
    assert_eq!(plan.widened_artifact_keys(), &[layout_artifact_id]);
    assert_eq!(
        plan.execution_origin(),
        PlacementExecutionOrigin::Background
    );

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.broadened_recall_plan_count, 1);
}

#[test]
fn adaptive_policy_debt_is_explicit_and_observable() {
    let (store, branch_id, _, _, _) = tiering_phase2_fixture();

    let report = store
        .plan_authoritative_tier_move(
            PlacementPolicyClass::AdaptiveDebt(
                crate::AdaptivePlacementDebtMarker::CrossBranchGlobalHeatBalancing,
            ),
            PlacementObservationScopeClass::Branch,
            &branch_id.0,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();

    assert!(report.tier_move_plan().is_none());
    assert!(matches!(
        report.rejection(),
        Some(crate::TierMoveRejection::UnsupportedPolicy { .. })
    ));
    assert!(report.debt().is_some());

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.placement_debt_count, 1);
    assert_eq!(counters.working_set_debt_count, 1);

    let surface = store.milestone_13_complexity_surface();
    assert_eq!(
        surface.working_set_classification.status,
        ComplexityStatus::Debt
    );
    assert_eq!(surface.tier_move_planning.status, ComplexityStatus::Debt);
}

#[test]
fn phase_2_complexity_surface_verifies_conservative_paths() {
    let (store, branch_id, _, snapshot_id, _) = tiering_phase2_fixture();

    store
        .observe_working_set(PlacementObservationScopeClass::Branch, &branch_id.0)
        .unwrap();
    store
        .summarize_placement_demand(PlacementObservationScopeClass::Branch, &branch_id.0)
        .unwrap();
    store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::Branch,
            &branch_id.0,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();

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
    assert_eq!(surface.tier_move_cutover.status, ComplexityStatus::Debt);
    assert_eq!(surface.tier_move_execution.status, ComplexityStatus::Debt);
    assert_eq!(surface.cold_recall_execution.status, ComplexityStatus::Debt);
    assert_eq!(surface.recall_coalescing.status, ComplexityStatus::Debt);
}
