use topology::replay_undo_semantic_graph::current_replay_undo_topology_boundary;
use worth_spatial::facade::replay_undo_semantic_graph::current_boolean_split_spatial_boundary;

use crate::replay_undo_transaction_boundary::{
    admit_replay_undo_transaction_boundary_packet, assemble_replay_undo_transaction_boundary_input,
    ReplayUndoTransactionBoundaryAssemblyRequest, ReplayUndoTransactionBoundaryInput,
    ReplayUndoTransactionBoundarySupportSource,
};

use super::current_replay_undo_boundary_proof::{
    test_current_replay_undo_boundary_packet_input,
    test_lower_replay_undo_boundary_proof_from_packet,
};
use super::tests_support::{ordinary_completed_split_handoff, with_replay_undo_scope_products};

#[test]
fn replay_undo_boundary_proof_rejects_foreign_packet_stage_identity() {
    let split_boundary = current_boolean_split_spatial_boundary().expect("current split boundary");
    let current_input = test_current_replay_undo_boundary_packet_input(&split_boundary)
        .expect("current replay/undo packet input");
    let foreign_input = foreign_packet_input("phase13 foreign replay-undo packet stage");
    let packet = admit_replay_undo_transaction_boundary_packet(replay_undo_input_with_overrides(
        &current_input,
        foreign_input.stage_index_identity().clone(),
        current_input.evidence_lookup_receipt_identity().clone(),
    ))
    .expect("foreign-stage packet should remain structurally admissible");

    let topology_boundary =
        current_replay_undo_topology_boundary().expect("current topology boundary");
    let error = test_lower_replay_undo_boundary_proof_from_packet(
        &split_boundary,
        topology_boundary.boundary_digest(),
        &packet,
    )
    .expect_err("proof must reject a foreign packet stage identity");

    let error_debug = format!("{error:?}");
    assert!(error_debug.contains("MissingCurrentProofChain"));
    assert!(error_debug.contains("packet stage index"));
}

#[test]
fn replay_undo_boundary_proof_rejects_foreign_packet_lookup_identity() {
    let split_boundary = current_boolean_split_spatial_boundary().expect("current split boundary");
    let current_input = test_current_replay_undo_boundary_packet_input(&split_boundary)
        .expect("current replay/undo packet input");
    let foreign_input = foreign_packet_input("phase13 foreign replay-undo packet lookup");
    let packet = admit_replay_undo_transaction_boundary_packet(replay_undo_input_with_overrides(
        &current_input,
        current_input.stage_index_identity().clone(),
        foreign_input.evidence_lookup_receipt_identity().clone(),
    ))
    .expect("foreign-lookup packet should remain structurally admissible");

    let topology_boundary =
        current_replay_undo_topology_boundary().expect("current topology boundary");
    let error = test_lower_replay_undo_boundary_proof_from_packet(
        &split_boundary,
        topology_boundary.boundary_digest(),
        &packet,
    )
    .expect_err("proof must reject a foreign packet lookup identity");

    let error_debug = format!("{error:?}");
    assert!(error_debug.contains("MissingCurrentProofChain"));
    assert!(error_debug.contains("packet lookup receipt"));
}

fn foreign_packet_input(label: &'static str) -> ReplayUndoTransactionBoundaryInput {
    let completed_split_handoff = ordinary_completed_split_handoff(label);
    let topology_boundary =
        current_replay_undo_topology_boundary().expect("current topology boundary");
    let topology_undo_scope = topology_boundary
        .lower_undo_scope_product()
        .expect("current topology undo scope");
    with_replay_undo_scope_products(
        label,
        &completed_split_handoff,
        |replay_scope, undo_scope| {
            assemble_replay_undo_transaction_boundary_input(
                ReplayUndoTransactionBoundaryAssemblyRequest::new(
                    &topology_undo_scope,
                    replay_scope,
                    undo_scope,
                    ReplayUndoTransactionBoundarySupportSource::Ordinary,
                ),
            )
            .expect("foreign replay/undo packet input")
        },
    )
}

fn replay_undo_input_with_overrides(
    input: &ReplayUndoTransactionBoundaryInput,
    stage_index_identity:
        schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphStageIndexIdentity,
    evidence_lookup_receipt_identity:
        schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphPriorProofIdentity,
) -> ReplayUndoTransactionBoundaryInput {
    ReplayUndoTransactionBoundaryInput::new(
        input.touched_digest(),
        stage_index_identity,
        input.invalidation_receipt_identity().clone(),
        evidence_lookup_receipt_identity,
        input.replay_scope_identity().clone(),
        input.undo_scope_identity().clone(),
        input.support_posture().clone(),
        input.mutation_claims().to_vec(),
        input.counters().clone(),
    )
}
