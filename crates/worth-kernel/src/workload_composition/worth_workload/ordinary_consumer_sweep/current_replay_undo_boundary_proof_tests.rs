use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphTouchedSubject, ReplayUndoTransactionScopeClaim,
    ReplayUndoTransactionScopeKind,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
};

use super::current_replay_undo_boundary_proof::test_current_replay_undo_boundary_proof_with_input_override;

#[test]
fn replay_undo_boundary_proof_rejects_foreign_packet_stage_identity() {
    let error = test_current_replay_undo_boundary_proof_with_input_override(|input| {
        crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput::new(
            input.touched_digest(),
            admit_replay_undo_stage_index_identity("foreign-stage-index"),
            input.invalidation_receipt_identity().clone(),
            input.evidence_lookup_receipt_identity().clone(),
            input.replay_scope_identity().clone(),
            input.undo_scope_identity().clone(),
            input.support_posture().clone(),
            input.mutation_claims().to_vec(),
            input.counters().clone(),
        )
    })
    .expect_err("foreign stage identity should deny");

    let message = format!("{error:?}");
    assert!(message.contains("planner-owned replay/undo route"));
    assert!(message.contains("stage index"));
}

#[test]
fn replay_undo_boundary_proof_rejects_foreign_packet_lookup_identity() {
    let error = test_current_replay_undo_boundary_proof_with_input_override(|input| {
        crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput::new(
            input.touched_digest(),
            input.stage_index_identity().clone(),
            input.invalidation_receipt_identity().clone(),
            admit_spatial_evidence_lookup_prior_proof_identity("foreign-lookup-receipt"),
            input.replay_scope_identity().clone(),
            input.undo_scope_identity().clone(),
            input.support_posture().clone(),
            input.mutation_claims().to_vec(),
            input.counters().clone(),
        )
    })
    .expect_err("foreign lookup identity should deny");

    let message = format!("{error:?}");
    assert!(message.contains("planner-owned replay/undo route"));
    assert!(message.contains("lookup receipt"));
}

#[test]
fn replay_undo_boundary_proof_rejects_foreign_replay_scope_identity() {
    let error = test_current_replay_undo_boundary_proof_with_input_override(|input| {
        let current_basis = input.replay_scope_identity().equivalence_basis().clone();
        let mut hostile_subjects = current_basis.touched_subjects().to_vec();
        hostile_subjects.push(
            ReplayUndoSemanticGraphTouchedSubject::TopologyRelationKind {
                relation_kind: "foreign-replay-execution-proof".to_string(),
            },
        );
        let foreign_replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
            ReplayUndoSemanticGraphEquivalenceBasis::new(
                current_basis.locality_scope(),
                hostile_subjects,
                current_basis.prior_proof_identity().clone(),
                current_basis.stage_index_identity().cloned(),
            ),
        ));

        crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput::new(
            input.touched_digest(),
            input.stage_index_identity().clone(),
            input.invalidation_receipt_identity().clone(),
            input.evidence_lookup_receipt_identity().clone(),
            foreign_replay_scope.clone(),
            input.undo_scope_identity().clone(),
            input.support_posture().clone(),
            input
                .mutation_claims()
                .iter()
                .map(|claim| match claim.kind() {
                    ReplayUndoTransactionScopeKind::Replay => ReplayUndoTransactionScopeClaim::new(
                        ReplayUndoTransactionScopeKind::Replay,
                        foreign_replay_scope.digest(),
                    ),
                    ReplayUndoTransactionScopeKind::Undo => claim.clone(),
                })
                .collect(),
            input.counters().clone(),
        )
    })
    .expect_err("foreign replay scope identity should deny");

    let message = format!("{error:?}");
    assert!(message.contains("planner-owned replay/undo route"));
    assert!(message.contains("replay scope"));
}
