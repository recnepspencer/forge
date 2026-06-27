use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayScopeIdentity, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, ReplayUndoTransactionScopeClaim, UndoScopeIdentity,
};

use super::ReplayUndoTransactionBoundaryPacketCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoTransactionBoundarySupportPosture {
    Ordinary,
    QueryGap {
        owner: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoTransactionBoundaryInput {
    touched_digest: String,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    invalidation_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    evidence_lookup_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    replay_scope_identity: ReplayScopeIdentity,
    undo_scope_identity: UndoScopeIdentity,
    support_posture: ReplayUndoTransactionBoundarySupportPosture,
    mutation_claims: Vec<ReplayUndoTransactionScopeClaim>,
    counters: ReplayUndoTransactionBoundaryPacketCounters,
}

impl ReplayUndoTransactionBoundaryInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        touched_digest: impl Into<String>,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        invalidation_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        evidence_lookup_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        replay_scope_identity: ReplayScopeIdentity,
        undo_scope_identity: UndoScopeIdentity,
        support_posture: ReplayUndoTransactionBoundarySupportPosture,
        mutation_claims: Vec<ReplayUndoTransactionScopeClaim>,
        counters: ReplayUndoTransactionBoundaryPacketCounters,
    ) -> Self {
        Self {
            touched_digest: touched_digest.into(),
            stage_index_identity,
            invalidation_receipt_identity,
            evidence_lookup_receipt_identity,
            replay_scope_identity,
            undo_scope_identity,
            support_posture,
            mutation_claims,
            counters,
        }
    }

    pub fn touched_digest(&self) -> &str {
        &self.touched_digest
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }

    pub const fn invalidation_receipt_identity(
        &self,
    ) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.invalidation_receipt_identity
    }

    pub const fn evidence_lookup_receipt_identity(
        &self,
    ) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.evidence_lookup_receipt_identity
    }

    pub const fn replay_scope_identity(&self) -> &ReplayScopeIdentity {
        &self.replay_scope_identity
    }

    pub const fn undo_scope_identity(&self) -> &UndoScopeIdentity {
        &self.undo_scope_identity
    }

    pub const fn support_posture(&self) -> &ReplayUndoTransactionBoundarySupportPosture {
        &self.support_posture
    }

    pub fn mutation_claims(&self) -> &[ReplayUndoTransactionScopeClaim] {
        &self.mutation_claims
    }

    pub const fn counters(&self) -> &ReplayUndoTransactionBoundaryPacketCounters {
        &self.counters
    }
}
