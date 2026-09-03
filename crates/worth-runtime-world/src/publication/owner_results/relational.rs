use worth_relational::facade::branch::{AdmittedRelationalBranchBasis, RelationalForkOutcome};
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_relational::facade::transactions::CommitResult;

use super::{CompositeRelationalOwnerResult, CompositeRelationalOwnerResultKind};

impl CompositeRelationalOwnerResult {
    pub(crate) fn retained() -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::RetainedExact,
        }
    }

    pub(crate) fn settled(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self::settled_with_fork(None, commit_identity, successor_basis, result)
    }

    pub(crate) fn settlement_pending(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self::settlement_pending_with_fork(None, commit_identity, successor_basis, settlement)
    }

    pub(crate) fn settlement_pending_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self::settlement_pending_with_fork(Some(fork), commit_identity, successor_basis, settlement)
    }

    pub(super) fn settlement_pending_with_fork(
        fork: Option<RelationalForkOutcome>,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::SettlementPending {
                commit_identity,
                successor_basis,
                fork,
                settlement,
            },
        }
    }

    pub(crate) fn settlement_required(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self::settlement_required_with_fork(None, commit_identity, successor_basis)
    }

    pub(crate) fn settlement_required_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self::settlement_required_with_fork(Some(fork), commit_identity, successor_basis)
    }

    pub(super) fn settlement_required_with_fork(
        fork: Option<RelationalForkOutcome>,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::SettlementRequired {
                commit_identity,
                successor_basis,
                fork,
            },
        }
    }

    pub(crate) fn settled_receipt(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self::settled_receipt_with_fork(None, commit_identity, successor_basis, receipt)
    }

    pub(super) fn settled_receipt_with_fork(
        fork: Option<RelationalForkOutcome>,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::Published {
                commit_identity,
                successor_basis,
                fork,
                settlement: Some(receipt),
                result: None,
            },
        }
    }

    pub(crate) fn forked(
        fork: RelationalForkOutcome,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::Forked {
                fork,
                successor_basis,
            },
        }
    }

    pub(crate) fn settled_after_fork(
        fork: RelationalForkOutcome,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self::settled_with_fork(Some(fork), commit_identity, successor_basis, result)
    }

    pub(super) fn settled_with_fork(
        fork: Option<RelationalForkOutcome>,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::Published {
                commit_identity,
                successor_basis,
                fork,
                settlement: Some(result.outcome().commit.clone()),
                result: Some(result),
            },
        }
    }

    pub(super) fn into_fork(self) -> Option<RelationalForkOutcome> {
        match self.result {
            CompositeRelationalOwnerResultKind::Forked { fork, .. } => Some(fork),
            CompositeRelationalOwnerResultKind::Published { fork, .. } => fork,
            CompositeRelationalOwnerResultKind::SettlementRequired { fork, .. }
            | CompositeRelationalOwnerResultKind::SettlementPending { fork, .. } => fork,
            CompositeRelationalOwnerResultKind::RetainedExact => None,
        }
    }
}
