use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictIndependencePlannerRouteFamily, ConflictIndependencePlannerRouteWitnessKind,
};

use super::*;
use crate::workload_composition::batch_admission::BatchAdmissionPlanDenialKind;
use crate::workload_composition::planner_owned_routing::{
    admitted_public_proof_input::current_worth_touched_graph_conflict_public_proof_input_with_packet_loader,
    current_replay_undo_transaction_route_packet,
    derived_diagnostics::current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader,
    selected_route::current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders,
    selected_route::WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingErrorKind,
};

#[test]
fn conflict_and_independence_explanation_share_selected_route_chain() {
    let route_packet = current_worth_touched_graph_conflict_independence_route_packet()
        .expect("planner-owned conflict/independence route packet");
    let selected_route =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("selected-route packet");
    let public_proof_input =
        crate::workload_composition::current_worth_touched_graph_conflict_public_proof_input()
            .expect("public proof input");

    assert_eq!(
        route_packet.conflict_route_family(),
        ConflictIndependencePlannerRouteFamily::ConflictRoute
    );
    assert_eq!(
        route_packet.independence_route_family(),
        ConflictIndependencePlannerRouteFamily::IndependenceRoute
    );
    assert_eq!(
        selected_route.conflict_independence_route_packet_identity(),
        route_packet.packet_identity()
    );
    assert_eq!(
        public_proof_input.conflict_independence_route_packet_identity(),
        route_packet.packet_identity()
    );
    assert_eq!(
        selected_route.selected_conflict_plan_digests(),
        route_packet.selected_conflict_plan_digests()
    );
    assert_eq!(
        selected_route.independence_proof_digests(),
        route_packet.independence_proof_identities()
    );
}

#[test]
fn denial_witness_localizes_conflict_vs_independence_failure() {
    let conflict_denied =
        current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override(
            |receipt| {
                receipt.with_test_denial_kind(BatchAdmissionPlanDenialKind::SelectedPlanDenied)
            },
        )
        .expect("conflict-denied route packet");
    let independence_denied =
        current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override(
            |receipt| {
                receipt.with_test_denial_kind(
                    BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof,
                )
            },
        )
        .expect("independence-denied route packet");

    assert_eq!(
        conflict_denied
            .denial_witness()
            .expect("conflict denial witness")
            .kind(),
        ConflictIndependencePlannerRouteWitnessKind::ConflictRouteDenial
    );
    assert_eq!(
        independence_denied
            .denial_witness()
            .expect("independence denial witness")
            .kind(),
        ConflictIndependencePlannerRouteWitnessKind::IndependenceDenial
    );
    assert_ne!(
        conflict_denied
            .denial_witness()
            .expect("conflict denial witness")
            .identity_digest(),
        independence_denied
            .denial_witness()
            .expect("independence denial witness")
            .identity_digest()
    );

    assert_downstream_denial_propagation(
        conflict_denied.clone(),
        ConflictIndependencePlannerRouteWitnessKind::ConflictRouteDenial,
        conflict_denied
            .denial_witness()
            .expect("conflict denial witness")
            .identity_digest(),
    );
    assert_downstream_denial_propagation(
        independence_denied.clone(),
        ConflictIndependencePlannerRouteWitnessKind::IndependenceDenial,
        independence_denied
            .denial_witness()
            .expect("independence denial witness")
            .identity_digest(),
    );
}

fn assert_downstream_denial_propagation(
    denied_route_packet: ConflictIndependencePlannerRoutePacket,
    expected_kind: ConflictIndependencePlannerRouteWitnessKind,
    expected_identity: &str,
) {
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
        topology::certification::current_topology_milestone_fifteen_planner_seed_support,
        worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
        current_replay_undo_transaction_route_packet,
        crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
        || Ok(denied_route_packet),
        crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    )
    .expect("selected-route packet should preserve typed denial witness");
    assert_selected_route_denial(&selected_route_packet, expected_kind, expected_identity);
    let public_proof_input =
        current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
            Ok(selected_route_packet.clone())
        })
        .expect("public proof input should preserve typed denial witness");
    let diagnostic_input =
        current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader(
            || Ok(selected_route_packet.clone()),
        )
        .expect("diagnostic input should preserve typed denial witness");
    let read_diagnostics = diagnostic_input.as_read_diagnostics();

    assert_eq!(
        public_proof_input.conflict_independence_denial_witness_kind(),
        Some(expected_kind)
    );
    assert_eq!(
        public_proof_input.conflict_independence_denial_witness_identity(),
        Some(expected_identity)
    );
    assert_eq!(
        diagnostic_input.conflict_independence_denial_witness_kind(),
        Some(expected_kind)
    );
    assert_eq!(
        diagnostic_input.conflict_independence_denial_witness_identity(),
        Some(expected_identity)
    );
    assert_eq!(
        read_diagnostics.conflict_independence_denial_witness_kind,
        Some(expected_kind)
    );
    assert_eq!(
        read_diagnostics.conflict_independence_denial_witness_identity,
        Some(expected_identity.to_string())
    );
}

fn assert_selected_route_denial(
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    expected_kind: ConflictIndependencePlannerRouteWitnessKind,
    expected_identity: &str,
) {
    assert_eq!(
        selected_route_packet.conflict_independence_denial_witness_kind(),
        Some(expected_kind)
    );
    assert_eq!(
        selected_route_packet.conflict_independence_denial_witness_identity(),
        Some(expected_identity)
    );
}

#[test]
fn selected_route_denies_stale_conflict_independence_overlap_and_locality_basis() {
    let stale_route_packet =
        current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override(
            |receipt| {
                receipt
                    .with_test_overlap_identity_digests(vec!["foreign-overlap-basis".to_string()])
                    .with_test_locality_footprint_digests(
                        vec!["foreign-locality-basis".to_string()],
                    )
            },
        )
        .expect("stale conflict/independence route packet");

    let error = current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
        topology::certification::current_topology_milestone_fifteen_planner_seed_support,
        worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
        current_replay_undo_transaction_route_packet,
        crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
        || Ok(stale_route_packet),
        crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    )
    .expect_err("stale conflict/independence route basis should be rejected");

    assert_eq!(
        error.kind(),
        PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport
    );
}
