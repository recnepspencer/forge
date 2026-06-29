use super::test_support::{
    ordinary_touched_closure, packet_backed_boundary, spatial_closeout_with_replay_prior_proof,
    spatial_closeout_without_replay_route_match, spatial_equivalent_closeout_reordered,
    topology_closeout_with_replay_prior_proof, topology_closeout_without_replay_route_match,
    topology_equivalent_closeout_reordered,
};
use super::{
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    SpatialConflictPlanDenialKind, TopologyConflictPlanDenialKind,
};
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input, SpatialConflictInputRequest,
    TopologyConflictInputRequest,
};
use topology::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictPriorProofPosture,
};
use worth_spatial::facade::replay_undo_semantic_graph::boolean_event_ledger_spatial_boundary_fixture;
use worth_spatial::touched_graph_conflict::{
    current_spatial_conflict_family_catalog_closeout, SpatialConflictPriorProofPosture,
};

#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

#[test]
fn topology_selected_plan_identity_ignores_equivalent_catalog_declaration_order() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("aspect route admits");

    let canonical = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let reordered = topology_equivalent_closeout_reordered();
    let canonical_plan = lower_selected_topology_conflict_plan(&canonical, &admitted);
    let reordered_plan = lower_selected_topology_conflict_plan(&reordered, &admitted);

    assert_eq!(
        canonical_plan.selected_plan_digest(),
        reordered_plan.selected_plan_digest()
    );
    assert_eq!(
        canonical_plan.selected_families(),
        reordered_plan.selected_families()
    );
    assert_eq!(
        canonical_plan.unselected_family_identities(),
        reordered_plan.unselected_family_identities()
    );
    assert_eq!(
        canonical_plan.execution_admission(),
        reordered_plan.execution_admission()
    );
    assert_eq!(canonical_plan.denial(), reordered_plan.denial());
}

#[test]
fn spatial_selected_plan_identity_ignores_equivalent_catalog_declaration_order() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("evidence route admits");

    let canonical = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let reordered = spatial_equivalent_closeout_reordered();
    let canonical_plan = lower_selected_spatial_conflict_plan(&canonical, &admitted);
    let reordered_plan = lower_selected_spatial_conflict_plan(&reordered, &admitted);

    assert_eq!(
        canonical_plan.selected_plan_digest(),
        reordered_plan.selected_plan_digest()
    );
    assert_eq!(
        canonical_plan.selected_families(),
        reordered_plan.selected_families()
    );
    assert_eq!(
        canonical_plan.unselected_family_identities(),
        reordered_plan.unselected_family_identities()
    );
    assert_eq!(
        canonical_plan.execution_admission(),
        reordered_plan.execution_admission()
    );
    assert_eq!(canonical_plan.denial(), reordered_plan.denial());
}

#[test]
fn topology_denial_kinds_do_not_share_selected_plan_identity() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase5.topology.digest_denial");
    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");

    let missing_prior = lower_selected_topology_conflict_plan(
        &topology_closeout_with_replay_prior_proof(
            TopologyConflictPriorProofPosture::NoPriorProofRequired,
        ),
        &admitted,
    );
    let no_match = lower_selected_topology_conflict_plan(
        &topology_closeout_without_replay_route_match(),
        &admitted,
    );

    assert_eq!(
        missing_prior.denial().expect("denial").kind(),
        TopologyConflictPlanDenialKind::MissingRequiredPriorProof
    );
    assert_eq!(
        no_match.denial().expect("denial").kind(),
        TopologyConflictPlanDenialKind::NoMatchingFamily
    );
    assert_ne!(
        missing_prior.selected_plan_digest(),
        no_match.selected_plan_digest()
    );
}

#[test]
fn spatial_denial_kinds_do_not_share_selected_plan_identity() {
    let subject =
        replay_support::MetabossEventExtractionSubject::certify("phase5.spatial.digest_denial");
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let boundary = packet_backed_boundary("phase5.spatial.digest_denial");
    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("replay route admits");

    let missing_prior = lower_selected_spatial_conflict_plan(
        &spatial_closeout_with_replay_prior_proof(
            SpatialConflictPriorProofPosture::NoPriorProofRequired,
        ),
        &admitted,
    );
    let no_match = lower_selected_spatial_conflict_plan(
        &spatial_closeout_without_replay_route_match(),
        &admitted,
    );

    assert_eq!(
        missing_prior.denial().expect("denial").kind(),
        SpatialConflictPlanDenialKind::MissingRequiredPriorProof
    );
    assert_eq!(
        no_match.denial().expect("denial").kind(),
        SpatialConflictPlanDenialKind::NoMatchingFamily
    );
    assert_ne!(
        missing_prior.selected_plan_digest(),
        no_match.selected_plan_digest()
    );
}
