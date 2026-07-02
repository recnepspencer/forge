use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};

use super::*;
use crate::workload_composition::batch_admission::BatchAdmissionPlanDenialKind;
use crate::workload_composition::planner_owned_routing::{
    admitted_public_proof_input::current_worth_touched_graph_conflict_public_proof_input_with_packet_loader,
    current_replay_undo_transaction_route_packet,
    current_worth_touched_graph_conflict_independence_route_packet,
    derived_diagnostics::current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader,
    selected_route::current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders,
};

#[test]
fn batch_admission_explanation_binds_selected_route_and_batch_identity() {
    let route_packet = current_worth_touched_graph_conflict_batch_admission_route_packet()
        .expect("planner-owned batch-admission route packet");
    let selected_route =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("selected-route packet");
    let public_proof_input =
        crate::workload_composition::current_worth_touched_graph_conflict_public_proof_input()
            .expect("public proof input");
    let proof_chain_inputs = selected_route.lower_proof_chain_inputs();

    assert_eq!(
        selected_route.batch_admission_route_packet_identity(),
        route_packet.packet_identity()
    );
    assert_eq!(
        public_proof_input.batch_admission_route_packet_identity(),
        route_packet.packet_identity()
    );
    assert_eq!(
        selected_route.selected_batch_plan_digest(),
        route_packet.selected_batch_plan_digest()
    );
    assert_eq!(
        selected_route.batch_execution_receipt_digest(),
        route_packet.batch_execution_receipt_digest()
    );
    assert_eq!(
        proof_chain_inputs.selected_batch_plan_digest,
        route_packet.selected_batch_plan_digest()
    );
    assert_eq!(
        proof_chain_inputs.batch_execution_receipt_digest,
        route_packet.batch_execution_receipt_digest()
    );
}

#[test]
fn batch_denial_witness_remains_distinct_from_conflict_and_independence_denial() {
    let batch_denied =
        current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_override(
            |receipt| {
                receipt.with_test_denial_kind(BatchAdmissionPlanDenialKind::SelectedPlanDenied)
            },
        )
        .expect("batch-denied route packet");
    let conflict_denied = crate::workload_composition::planner_owned_routing::
        current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override(
            |receipt| receipt.with_test_denial_kind(BatchAdmissionPlanDenialKind::SelectedPlanDenied),
        )
        .expect("conflict-denied route packet");
    let independence_denied = crate::workload_composition::planner_owned_routing::
        current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override(
            |receipt| {
                receipt.with_test_denial_kind(
                    BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof,
                )
            },
        )
        .expect("independence-denied route packet");

    assert_eq!(
        batch_denied.denial_witness_kind(),
        Some(BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial)
    );
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
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
        topology::certification::current_topology_milestone_fifteen_planner_seed_support,
        worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
        current_replay_undo_transaction_route_packet,
        || Ok(batch_denied.clone()),
        current_worth_touched_graph_conflict_independence_route_packet,
        crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    )
    .expect("selected-route packet should preserve batch denial witness");
    let public_proof_input =
        current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
            Ok(selected_route_packet.clone())
        })
        .expect("public proof input should preserve batch denial witness");
    let diagnostic_input =
        current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader(
            || Ok(selected_route_packet.clone()),
        )
        .expect("diagnostic input should preserve batch denial witness");
    let read_diagnostics = diagnostic_input.as_read_diagnostics();
    let expected_identity = batch_denied
        .denial_witness()
        .expect("batch denial witness")
        .identity_digest();

    assert_eq!(
        selected_route_packet.batch_admission_denial_witness_kind(),
        Some(BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial)
    );
    assert_eq!(
        selected_route_packet.batch_admission_denial_witness_identity(),
        Some(expected_identity)
    );
    assert_eq!(
        public_proof_input.batch_admission_denial_witness_kind(),
        Some(BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial)
    );
    assert_eq!(
        public_proof_input.batch_admission_denial_witness_identity(),
        Some(expected_identity)
    );
    assert_eq!(
        diagnostic_input.batch_admission_denial_witness_kind(),
        Some(BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial)
    );
    assert_eq!(
        diagnostic_input.batch_admission_denial_witness_identity(),
        Some(expected_identity)
    );
    assert_eq!(
        read_diagnostics.batch_admission_denial_witness_kind,
        Some(BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial)
    );
    assert_eq!(
        read_diagnostics.batch_admission_denial_witness_identity,
        Some(expected_identity.to_string())
    );
    assert_ne!(
        read_diagnostics.batch_admission_denial_witness_identity,
        conflict_denied
            .denial_witness()
            .map(|witness| witness.identity_digest().to_string())
    );
    assert_ne!(
        read_diagnostics.batch_admission_denial_witness_identity,
        independence_denied
            .denial_witness()
            .map(|witness| witness.identity_digest().to_string())
    );
}
