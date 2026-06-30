use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
    ReplayUndoSemanticGraphTouchedSubject,
};
use schema::facade::platform::authority::WorthTopologyTouchedAspect;

use crate::replay_undo_transaction_boundary::{
    ReplayUndoTransactionBoundaryPacketCounters, ReplayUndoTransactionBoundarySupportPosture,
};

pub fn equivalence_basis() -> ReplayUndoSemanticGraphEquivalenceBasis {
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        vec![ReplayUndoSemanticGraphTouchedSubject::TopologyAspect {
            aspect: WorthTopologyTouchedAspect::TopologyBoundary,
        }],
        schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity(
            "invalidation:receipt",
        ),
        Some(
            schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity(
                "stage:index",
            ),
        ),
    )
}

pub fn packet_counters(mutation_claim_count: usize) -> ReplayUndoTransactionBoundaryPacketCounters {
    ReplayUndoTransactionBoundaryPacketCounters::new(
        1,
        1,
        1,
        mutation_claim_count,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    )
}

pub fn query_gap_support_posture() -> ReplayUndoTransactionBoundarySupportPosture {
    ReplayUndoTransactionBoundarySupportPosture::QueryGap {
        owner: "forge-query",
        blocker: "minimal reversible graph patch proof missing",
        removal_trigger: "phase 12+ patch-application lane lands",
    }
}
