use super::test_support::{
    ordinary_touched_closure, packet_backed_boundary, spatial_closeout_with_replay_prior_proof,
    spatial_closeout_without_replay_route_match, topology_closeout_with_replay_prior_proof,
    topology_closeout_without_replay_route_match,
};
use super::{
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    ConflictPlanDownstreamProofCategory, ConflictPlanExecutionAdmission,
    SelectedSpatialConflictPlan, SelectedTopologyConflictPlan, SpatialConflictPlanDenialKind,
    TopologyConflictPlanDenialKind,
};
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input, SpatialConflictInputRequest,
    TopologyConflictInputRequest,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use topology::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictDiagnosticWitness,
    TopologyConflictFamilyIdentity, TopologyConflictPriorProofPosture,
    TopologyConflictSelectionProductPosture,
};
use worth_spatial::facade::replay_undo_semantic_graph::boolean_event_ledger_spatial_boundary_fixture;
use worth_spatial::touched_graph_conflict::{
    current_spatial_conflict_family_catalog_closeout, SpatialConflictDiagnosticWitness,
    SpatialConflictFamilyIdentity, SpatialConflictPriorProofPosture,
    SpatialConflictSelectionProductPosture,
};

#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

#[test]
fn topology_aspect_selected_plan_lowers_to_stable_family_identity() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("aspect route admits");
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");

    let first = lower_selected_topology_conflict_plan(&closeout, &admitted);
    let second = lower_selected_topology_conflict_plan(&closeout, &admitted);

    assert_topology_plan_shape(
        &first,
        &[TopologyConflictFamilyIdentity::AspectSelection],
        &[
            TopologyConflictFamilyIdentity::ReplayBoundarySelection,
            TopologyConflictFamilyIdentity::ValidatorSelection,
        ],
    );
    assert_eq!(first.overlap_category(), ConflictOverlapCategory::Aspect);
    assert_eq!(
        first.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::ProjectionConsumption
    );
    assert_eq!(
        first.prior_proof_posture(),
        TopologyConflictPriorProofPosture::NoPriorProofRequired
    );
    let selected_row = &first.selected_families()[0];
    assert_eq!(
        selected_row.identity(),
        TopologyConflictFamilyIdentity::AspectSelection
    );
    assert_eq!(
        selected_row.declaration_digest(),
        closeout
            .catalog()
            .family(TopologyConflictFamilyIdentity::AspectSelection)
            .expect("aspect declaration")
            .declaration_digest()
    );
    assert_eq!(
        selected_row.prior_proof_posture(),
        TopologyConflictPriorProofPosture::NoPriorProofRequired
    );
    assert_eq!(
        selected_row.diagnostic_witness(),
        TopologyConflictDiagnosticWitness::TouchedClosureDigest
    );
    assert_eq!(
        selected_row.selection_product_posture(),
        TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired
    );
    assert_eq!(
        selected_row.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::ProjectionConsumption
    );
    assert_eq!(first.selected_plan_digest(), second.selected_plan_digest());
}

#[test]
fn topology_replay_selected_plan_preserves_replay_family_before_execution() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase5.topology.replay");
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");

    let plan = lower_selected_topology_conflict_plan(&closeout, &admitted);

    assert_topology_plan_shape(
        &plan,
        &[TopologyConflictFamilyIdentity::ReplayBoundarySelection],
        &[
            TopologyConflictFamilyIdentity::AspectSelection,
            TopologyConflictFamilyIdentity::ValidatorSelection,
        ],
    );
    assert_eq!(plan.overlap_category(), ConflictOverlapCategory::ReplayUndo);
    assert_eq!(
        plan.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope
    );
    assert_eq!(
        plan.prior_proof_posture(),
        TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired
    );
    assert_eq!(plan.admitted_input_digest(), admitted.admission_digest());
    assert_eq!(
        plan.touched_closure().closure_digest(),
        touched_closure.closure_digest()
    );
}

#[test]
fn topology_selected_plan_denies_missing_replay_prior_proof_before_execution() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase5.topology.denial");
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = topology_closeout_with_replay_prior_proof(
        TopologyConflictPriorProofPosture::NoPriorProofRequired,
    );

    let plan = lower_selected_topology_conflict_plan(&closeout, &admitted);

    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Denied
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        TopologyConflictPlanDenialKind::MissingRequiredPriorProof
    );
    assert_eq!(
        plan.denial()
            .expect("denial row")
            .downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope
    );
}

#[test]
fn topology_selected_plan_reports_no_matching_family_when_route_shape_never_matches() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase5.topology.no_match");
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = topology_closeout_without_replay_route_match();

    let plan = lower_selected_topology_conflict_plan(&closeout, &admitted);

    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Denied
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        TopologyConflictPlanDenialKind::NoMatchingFamily
    );
}

#[test]
fn spatial_evidence_selected_plan_lowers_to_stable_family_identity() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("evidence route admits");
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");

    let first = lower_selected_spatial_conflict_plan(&closeout, &admitted);
    let second = lower_selected_spatial_conflict_plan(&closeout, &admitted);

    assert_spatial_plan_shape(
        &first,
        &[SpatialConflictFamilyIdentity::EvidenceSelection],
        &[SpatialConflictFamilyIdentity::ReplayBoundarySelection],
    );
    assert_eq!(first.overlap_category(), ConflictOverlapCategory::Evidence);
    assert_eq!(
        first.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::ProjectionConsumption
    );
    assert_eq!(
        first.prior_proof_posture(),
        SpatialConflictPriorProofPosture::NoPriorProofRequired
    );
    let selected_row = &first.selected_families()[0];
    assert_eq!(
        selected_row.identity(),
        SpatialConflictFamilyIdentity::EvidenceSelection
    );
    assert_eq!(
        selected_row.declaration_digest(),
        closeout
            .catalog()
            .family(SpatialConflictFamilyIdentity::EvidenceSelection)
            .expect("evidence declaration")
            .declaration_digest()
    );
    assert_eq!(
        selected_row.prior_proof_posture(),
        SpatialConflictPriorProofPosture::NoPriorProofRequired
    );
    assert_eq!(
        selected_row.diagnostic_witness(),
        SpatialConflictDiagnosticWitness::EvidenceFamilyDigest
    );
    assert_eq!(
        selected_row.selection_product_posture(),
        SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired
    );
    assert_eq!(
        selected_row.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::ProjectionConsumption
    );
    assert_eq!(first.selected_plan_digest(), second.selected_plan_digest());
}

#[test]
fn spatial_replay_selected_plan_preserves_replay_family_before_execution() {
    let subject = replay_support::MetabossEventExtractionSubject::certify("phase5.spatial.replay");
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let boundary = packet_backed_boundary("phase5.spatial.replay");
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");

    let plan = lower_selected_spatial_conflict_plan(&closeout, &admitted);

    assert_spatial_plan_shape(
        &plan,
        &[SpatialConflictFamilyIdentity::ReplayBoundarySelection],
        &[SpatialConflictFamilyIdentity::EvidenceSelection],
    );
    assert_eq!(plan.overlap_category(), ConflictOverlapCategory::ReplayUndo);
    assert_eq!(
        plan.downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope
    );
    assert_eq!(
        plan.prior_proof_posture(),
        SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired
    );
    assert_eq!(plan.admitted_input_digest(), admitted.admission_digest());
    assert_eq!(
        plan.authority().digest().as_str(),
        authority.digest().as_str()
    );
}

#[test]
fn spatial_selected_plan_denies_missing_replay_prior_proof_before_execution() {
    let subject = replay_support::MetabossEventExtractionSubject::certify("phase5.spatial.denial");
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let boundary = packet_backed_boundary("phase5.spatial.denial");
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = spatial_closeout_with_replay_prior_proof(
        SpatialConflictPriorProofPosture::NoPriorProofRequired,
    );

    let plan = lower_selected_spatial_conflict_plan(&closeout, &admitted);

    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Denied
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        SpatialConflictPlanDenialKind::MissingRequiredPriorProof
    );
    assert_eq!(
        plan.denial()
            .expect("denial row")
            .downstream_proof_category(),
        ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope
    );
}

#[test]
fn spatial_selected_plan_reports_no_matching_family_when_route_shape_never_matches() {
    let subject =
        replay_support::MetabossEventExtractionSubject::certify("phase5.spatial.no_match");
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let boundary = packet_backed_boundary("phase5.spatial.no_match");
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");
    let closeout = spatial_closeout_without_replay_route_match();

    let plan = lower_selected_spatial_conflict_plan(&closeout, &admitted);

    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Denied
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        SpatialConflictPlanDenialKind::NoMatchingFamily
    );
}

fn assert_topology_plan_shape(
    plan: &SelectedTopologyConflictPlan<'_>,
    selected: &[TopologyConflictFamilyIdentity],
    unselected: &[TopologyConflictFamilyIdentity],
) {
    assert_eq!(
        plan.selected_families()
            .iter()
            .map(|row| row.identity())
            .collect::<Vec<_>>(),
        selected
    );
    assert_eq!(plan.unselected_family_identities(), unselected);
    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Admitted
    );
    assert!(plan.denial().is_none());
}

fn assert_spatial_plan_shape(
    plan: &SelectedSpatialConflictPlan<'_>,
    selected: &[SpatialConflictFamilyIdentity],
    unselected: &[SpatialConflictFamilyIdentity],
) {
    assert_eq!(
        plan.selected_families()
            .iter()
            .map(|row| row.identity())
            .collect::<Vec<_>>(),
        selected
    );
    assert_eq!(plan.unselected_family_identities(), unselected);
    assert_eq!(
        plan.execution_admission(),
        ConflictPlanExecutionAdmission::Admitted
    );
    assert!(plan.denial().is_none());
}
