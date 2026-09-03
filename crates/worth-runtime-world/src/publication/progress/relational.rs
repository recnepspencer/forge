use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::branch::RelationalForkOutcome;
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::mvcc::PerformedRelationalCommit;
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_relational::facade::transactions::CommitResult;

use super::{
    RelationalAttemptProgress, RelationalAttemptProgressPosture, RelationalProgressEvidence,
};

impl RelationalAttemptProgress {
    pub(crate) fn forked(
        fork: RelationalForkOutcome,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Performed,
            evidence: None,
            fork: Some(fork),
            fork_successor_basis: Some(successor_basis),
        }
    }

    pub(crate) fn performed_after_fork(
        fork: RelationalForkOutcome,
        performed: PerformedRelationalCommit,
    ) -> Self {
        Self::forked_evidence(
            fork,
            RelationalAttemptProgressPosture::Performed,
            RelationalProgressEvidence::Performed(performed),
        )
    }

    pub(crate) fn settled_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self::forked_evidence(
            fork,
            RelationalAttemptProgressPosture::Settled,
            RelationalProgressEvidence::Settled {
                commit_identity,
                successor_basis,
                result,
            },
        )
    }

    pub(crate) fn settlement_pending_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self::forked_evidence(
            fork,
            RelationalAttemptProgressPosture::SettlementPending,
            RelationalProgressEvidence::SettlementPending {
                commit_identity,
                successor_basis,
                settlement,
            },
        )
    }

    pub(crate) fn settlement_required_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self::forked_evidence(
            fork,
            RelationalAttemptProgressPosture::SettlementRequired,
            RelationalProgressEvidence::SettlementRequired {
                commit_identity,
                successor_basis,
            },
        )
    }

    pub(crate) fn settled_receipt_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self::forked_evidence(
            fork,
            RelationalAttemptProgressPosture::Settled,
            RelationalProgressEvidence::SettledReceipt {
                commit_identity,
                successor_basis,
                receipt,
            },
        )
    }

    pub(crate) fn settlement_required(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::SettlementRequired,
            evidence: Some(RelationalProgressEvidence::SettlementRequired {
                commit_identity,
                successor_basis,
            }),
            fork: None,
            fork_successor_basis: None,
        }
    }

    fn forked_evidence(
        fork: RelationalForkOutcome,
        posture: RelationalAttemptProgressPosture,
        evidence: RelationalProgressEvidence,
    ) -> Self {
        Self {
            posture,
            fork_successor_basis: None,
            evidence: Some(evidence),
            fork: Some(fork),
        }
    }

    pub(crate) fn requires_settlement(&self) -> bool {
        matches!(
            self.evidence.as_ref(),
            Some(
                RelationalProgressEvidence::Performed(_)
                    | RelationalProgressEvidence::SettlementRequired { .. }
                    | RelationalProgressEvidence::SettlementPending { .. }
            )
        )
    }

    pub(crate) fn is_fork_only(&self) -> bool {
        self.posture == RelationalAttemptProgressPosture::Performed
            && self.evidence.is_none()
            && self.fork.is_some()
            && self.fork_successor_basis.is_some()
    }

    pub(crate) fn settled_receipt(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Settled,
            evidence: Some(RelationalProgressEvidence::SettledReceipt {
                commit_identity,
                successor_basis,
                receipt,
            }),
            fork: None,
            fork_successor_basis: None,
        }
    }
}
