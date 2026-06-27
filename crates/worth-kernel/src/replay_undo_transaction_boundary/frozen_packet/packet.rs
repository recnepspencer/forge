use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayScopeIdentity, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, ReplayUndoTransactionScopeClaim,
    ReplayUndoTransactionScopeKind, UndoScopeIdentity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    ReplayUndoTransactionBoundaryInput, ReplayUndoTransactionBoundaryPacketCounters,
    ReplayUndoTransactionBoundarySupportPosture,
};
use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoTransactionBoundaryPacket {
    touched_digest: String,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    invalidation_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    evidence_lookup_receipt_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    replay_scope_identity: ReplayScopeIdentity,
    undo_scope_identity: UndoScopeIdentity,
    support_posture: ReplayUndoTransactionBoundarySupportPosture,
    mutation_claims: Vec<ReplayUndoTransactionScopeClaim>,
    counters: ReplayUndoTransactionBoundaryPacketCounters,
    packet_identity: String,
}

pub fn admit_replay_undo_transaction_boundary_packet(
    input: ReplayUndoTransactionBoundaryInput,
) -> Result<ReplayUndoTransactionBoundaryPacket, ReplayUndoTransactionBoundaryError> {
    for claim in input.mutation_claims() {
        match claim.kind() {
            ReplayUndoTransactionScopeKind::Replay
                if claim.scope_identity_digest() != input.replay_scope_identity().digest() =>
            {
                return Err(
                    ReplayUndoTransactionBoundaryError::HiddenReplayMutationGap {
                        claim_scope_digest: claim.scope_identity_digest().to_string(),
                        expected_scope_digest: input.replay_scope_identity().digest().to_string(),
                    },
                );
            }
            ReplayUndoTransactionScopeKind::Undo
                if claim.scope_identity_digest() != input.undo_scope_identity().digest() =>
            {
                return Err(ReplayUndoTransactionBoundaryError::HiddenUndoMutationGap {
                    claim_scope_digest: claim.scope_identity_digest().to_string(),
                    expected_scope_digest: input.undo_scope_identity().digest().to_string(),
                });
            }
            _ => {}
        }
    }

    let packet_identity = lower_packet_identity(&input);
    Ok(ReplayUndoTransactionBoundaryPacket {
        touched_digest: input.touched_digest().to_string(),
        stage_index_identity: input.stage_index_identity().clone(),
        invalidation_receipt_identity: input.invalidation_receipt_identity().clone(),
        evidence_lookup_receipt_identity: input.evidence_lookup_receipt_identity().clone(),
        replay_scope_identity: input.replay_scope_identity().clone(),
        undo_scope_identity: input.undo_scope_identity().clone(),
        support_posture: input.support_posture().clone(),
        mutation_claims: input.mutation_claims().to_vec(),
        counters: input.counters().clone(),
        packet_identity,
    })
}

impl ReplayUndoTransactionBoundaryPacket {
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

    pub fn packet_identity(&self) -> &str {
        &self.packet_identity
    }
}

fn lower_packet_identity(input: &ReplayUndoTransactionBoundaryInput) -> String {
    let mut claims = input
        .mutation_claims()
        .iter()
        .map(|claim| format!("{:?}:{}", claim.kind(), claim.scope_identity_digest()))
        .collect::<Vec<_>>();
    claims.sort();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:replay-undo-transaction-boundary-packet:v2".to_string(),
            format!("touched-digest:{}", input.touched_digest()),
            format!("stage-index:{}", input.stage_index_identity().digest()),
            format!(
                "invalidation-receipt:{}",
                input.invalidation_receipt_identity().digest()
            ),
            format!(
                "evidence-lookup-receipt:{}",
                input.evidence_lookup_receipt_identity().digest()
            ),
            format!("replay-scope:{}", input.replay_scope_identity().digest()),
            format!("undo-scope:{}", input.undo_scope_identity().digest()),
            format!(
                "support-posture:{}",
                support_posture_digest(input.support_posture())
            ),
            format!("mutation-claims:{}", claims.join("|")),
            format!(
                "topology-touched-subjects:{}",
                input.counters().topology_touched_subject_count()
            ),
            format!(
                "replay-touched-subjects:{}",
                input.counters().replay_touched_subject_count()
            ),
            format!(
                "undo-touched-subjects:{}",
                input.counters().undo_touched_subject_count()
            ),
            format!(
                "mutation-claim-count:{}",
                input.counters().mutation_claim_count()
            ),
            format!(
                "replay-raw-row-scans:{}",
                input.counters().replay_raw_row_scan_count()
            ),
            format!(
                "replay-broad-receipt-scans:{}",
                input.counters().replay_broad_receipt_scan_count()
            ),
            format!(
                "replay-caller-owned-scans:{}",
                input.counters().replay_caller_owned_scan_count()
            ),
            format!(
                "replay-retained-replay-bindings:{}",
                input.counters().replay_retained_replay_binding_count()
            ),
            format!(
                "undo-lookup-consumed-workload-handoffs:{}",
                input
                    .counters()
                    .undo_lookup_consumed_workload_handoff_count()
            ),
            format!(
                "undo-raw-row-scans:{}",
                input.counters().undo_raw_row_scan_count()
            ),
            format!(
                "undo-broad-receipt-scans:{}",
                input.counters().undo_broad_receipt_scan_count()
            ),
            format!(
                "undo-caller-owned-scans:{}",
                input.counters().undo_caller_owned_scan_count()
            ),
        ],
    )
}

fn support_posture_digest(posture: &ReplayUndoTransactionBoundarySupportPosture) -> String {
    match posture {
        ReplayUndoTransactionBoundarySupportPosture::Ordinary => "ordinary".to_string(),
        ReplayUndoTransactionBoundarySupportPosture::QueryGap {
            owner,
            blocker,
            removal_trigger,
        } => format!("query-gap:{owner}:{blocker}:{removal_trigger}"),
    }
}
