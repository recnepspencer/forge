use worth_store_recovery_physics::{
    RecoverySourceDecisionKind, RecoverySourceDecisionOutcome, RecoverySourceDecisionTrace,
};

use super::SourcePrecedenceAction;

pub fn map_recovery_source_decision_trace(
    trace: &RecoverySourceDecisionTrace,
) -> Vec<SourcePrecedenceAction> {
    let mut actions = Vec::with_capacity(trace.decision_rows().len() * 2 + 2);
    for row in trace.decision_rows() {
        let discovery_order = row.trace().discovery_order();
        actions.push(SourcePrecedenceAction::CandidateDiscovered {
            discovery_order,
            role: row.role(),
        });
        actions.push(match row.outcome() {
            RecoverySourceDecisionOutcome::AdmittedCandidate => {
                SourcePrecedenceAction::CandidateAdmitted { discovery_order }
            }
            RecoverySourceDecisionOutcome::ApplicationRoleOnly => {
                SourcePrecedenceAction::CandidateAdvisoryOnly { discovery_order }
            }
            RecoverySourceDecisionOutcome::DiscoveryOnly
            | RecoverySourceDecisionOutcome::RejectedResidue => {
                SourcePrecedenceAction::CandidateRejected { discovery_order }
            }
            RecoverySourceDecisionOutcome::RecoveryBlocked => {
                SourcePrecedenceAction::SourceQuarantined
            }
        });
    }
    if trace.decision_rows().len() > 1 {
        actions.push(SourcePrecedenceAction::ContradictionPreserved);
    }
    actions.push(match trace.kind() {
        RecoverySourceDecisionKind::CheckpointPlusWalTail | RecoverySourceDecisionKind::WalOnly => {
            SourcePrecedenceAction::SourceSelected
        }
        RecoverySourceDecisionKind::NoValidCheckpoint => SourcePrecedenceAction::SourceDenied,
        RecoverySourceDecisionKind::RecoveryBlocked => SourcePrecedenceAction::SourceQuarantined,
    });
    actions
}
