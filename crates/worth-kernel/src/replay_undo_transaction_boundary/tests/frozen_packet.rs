use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, admit_undo_scope_identity, ReplayScopeIdentityInput,
    ReplayUndoTransactionScopeClaim, ReplayUndoTransactionScopeKind, UndoScopeIdentityInput,
};

use crate::replay_undo_transaction_boundary::{
    admit_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryError,
    ReplayUndoTransactionBoundaryInput, ReplayUndoTransactionBoundaryPacketCounters,
    ReplayUndoTransactionBoundarySupportPosture,
};

use super::test_support::{equivalence_basis, packet_counters};

#[test]
fn hidden_replay_mutation_gap_is_localized() {
    let basis = equivalence_basis();
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(basis.clone()));
    let undo_scope = admit_undo_scope_identity(UndoScopeIdentityInput::new(basis));
    let error =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope.clone(),
            undo_scope,
            ReplayUndoTransactionBoundarySupportPosture::QueryGap {
                owner: "forge-query",
                blocker: "minimal reversible graph patch proof missing",
                removal_trigger: "phase 12+ patch-application lane lands",
            },
            vec![ReplayUndoTransactionScopeClaim::new(
                ReplayUndoTransactionScopeKind::Replay,
                "replay:foreign-scope",
            )],
            packet_counters(1),
        ))
        .expect_err("foreign replay claim should be rejected");

    assert_eq!(
        error,
        ReplayUndoTransactionBoundaryError::HiddenReplayMutationGap {
            claim_scope_digest: "replay:foreign-scope".to_string(),
            expected_scope_digest: replay_scope.digest().to_string(),
        }
    );
}

#[test]
fn hidden_undo_mutation_gap_is_localized() {
    let basis = equivalence_basis();
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(basis.clone()));
    let undo_scope = admit_undo_scope_identity(UndoScopeIdentityInput::new(basis));
    let error =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope,
            undo_scope.clone(),
            ReplayUndoTransactionBoundarySupportPosture::QueryGap {
                owner: "forge-query",
                blocker: "minimal reversible graph patch proof missing",
                removal_trigger: "phase 12+ patch-application lane lands",
            },
            vec![ReplayUndoTransactionScopeClaim::new(
                ReplayUndoTransactionScopeKind::Undo,
                "undo:foreign-scope",
            )],
            packet_counters(1),
        ))
        .expect_err("foreign undo claim should be rejected");

    assert_eq!(
        error,
        ReplayUndoTransactionBoundaryError::HiddenUndoMutationGap {
            claim_scope_digest: "undo:foreign-scope".to_string(),
            expected_scope_digest: undo_scope.digest().to_string(),
        }
    );
}

#[test]
fn packet_identity_drifts_when_support_posture_drifts() {
    let basis = equivalence_basis();
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(basis.clone()));
    let undo_scope = admit_undo_scope_identity(UndoScopeIdentityInput::new(basis));

    let ordinary_packet =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope.clone(),
            undo_scope.clone(),
            ReplayUndoTransactionBoundarySupportPosture::Ordinary,
            Vec::new(),
            packet_counters(0),
        ))
        .expect("ordinary packet admits");
    let query_gap_packet =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope,
            undo_scope,
            ReplayUndoTransactionBoundarySupportPosture::QueryGap {
                owner: "forge-query",
                blocker: "minimal reversible graph patch proof missing",
                removal_trigger: "phase 12+ patch-application lane lands",
            },
            Vec::new(),
            packet_counters(0),
        ))
        .expect("query gap packet admits");

    assert_ne!(
        ordinary_packet.packet_identity(),
        query_gap_packet.packet_identity()
    );
}

#[test]
fn packet_identity_drifts_when_fallback_counters_drift() {
    let basis = equivalence_basis();
    let replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(basis.clone()));
    let undo_scope = admit_undo_scope_identity(UndoScopeIdentityInput::new(basis));

    let ordinary_packet =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope.clone(),
            undo_scope.clone(),
            ReplayUndoTransactionBoundarySupportPosture::QueryGap {
                owner: "forge-query",
                blocker: "minimal reversible graph patch proof missing",
                removal_trigger: "phase 12+ patch-application lane lands",
            },
            Vec::new(),
            packet_counters(0),
        ))
        .expect("ordinary packet admits");
    let drifted_packet =
        admit_replay_undo_transaction_boundary_packet(ReplayUndoTransactionBoundaryInput::new(
            "touched:digest",
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
                "invalidation:receipt",
            ),
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_spatial_evidence_lookup_prior_proof_identity(
                "lookup:receipt",
            ),
            replay_scope,
            undo_scope,
            ReplayUndoTransactionBoundarySupportPosture::QueryGap {
                owner: "forge-query",
                blocker: "minimal reversible graph patch proof missing",
                removal_trigger: "phase 12+ patch-application lane lands",
            },
            Vec::new(),
            ReplayUndoTransactionBoundaryPacketCounters::new(1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0),
        ))
        .expect("drifted packet admits");

    assert_ne!(
        ordinary_packet.packet_identity(),
        drifted_packet.packet_identity()
    );
}
