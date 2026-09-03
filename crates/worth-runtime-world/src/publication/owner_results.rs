use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_relational::facade::transactions::CommitResult;

use crate::history::CompositeComponentChangePosture;

#[path = "owner_results/signal.rs"]
mod signal;
pub use signal::CompositeSignalOwnerResult;

/// Exact result of the Relational leg. The retained variant is evidence that
/// no Relational owner movement was requested, not an absent or guessed result.
#[derive(Debug)]
pub struct CompositeRelationalOwnerResult {
    result: CompositeRelationalOwnerResultKind,
}

#[derive(Debug)]
enum CompositeRelationalOwnerResultKind {
    RetainedExact,
    Published {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: Option<worth_relational::facade::history::RelationalCommitReceipt>,
        result: Option<CommitResult>,
    },
    SettlementRequired {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    },
    SettlementPending {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    },
}

impl CompositeRelationalOwnerResult {
    pub(super) fn retained() -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::RetainedExact,
        }
    }

    pub(crate) fn settled(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        let settlement = Some(result.outcome().commit.clone());
        Self {
            result: CompositeRelationalOwnerResultKind::Published {
                commit_identity,
                successor_basis,
                settlement,
                result: Some(result),
            },
        }
    }

    pub(super) fn settlement_pending(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::SettlementPending {
                commit_identity,
                successor_basis,
                settlement,
            },
        }
    }

    pub(super) fn settlement_required(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::SettlementRequired {
                commit_identity,
                successor_basis,
            },
        }
    }

    pub(crate) fn settled_receipt(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::Published {
                commit_identity,
                successor_basis,
                settlement: Some(receipt),
                result: None,
            },
        }
    }
}

/// The two owner results carried by one performed publication. They are
/// created only from the corresponding owner progress and cannot be mixed
/// independently with a commit posture.
#[derive(Debug)]
pub struct CompositeOwnerExecutionResults {
    relational: CompositeRelationalOwnerResult,
    signal: CompositeSignalOwnerResult,
}

impl CompositeOwnerExecutionResults {
    pub(super) fn from_components(
        relational: CompositeRelationalOwnerResult,
        signal: CompositeSignalOwnerResult,
    ) -> Self {
        Self { relational, signal }
    }

    pub(crate) fn retained() -> Self {
        Self {
            relational: CompositeRelationalOwnerResult::retained(),
            signal: CompositeSignalOwnerResult::retained(),
        }
    }

    pub(crate) fn relational_settlement_pending(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            relational: CompositeRelationalOwnerResult::settlement_pending(
                commit_identity,
                successor_basis,
                settlement,
            ),
            signal: CompositeSignalOwnerResult::retained(),
        }
    }

    pub(crate) fn relational_settlement_required(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            relational: CompositeRelationalOwnerResult::settlement_required(
                commit_identity,
                successor_basis,
            ),
            signal: CompositeSignalOwnerResult::retained(),
        }
    }

    pub(crate) fn relational_settled(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self {
            relational: CompositeRelationalOwnerResult::settled(
                commit_identity,
                successor_basis,
                result,
            ),
            signal: CompositeSignalOwnerResult::retained(),
        }
    }

    pub(crate) fn with_relational_settled(
        self,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        let (_, signal) = self.into_parts();
        Self {
            relational: CompositeRelationalOwnerResult::settled(
                commit_identity,
                successor_basis,
                result,
            ),
            signal,
        }
    }

    pub(crate) fn with_relational_settled_receipt(
        self,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        let (_, signal) = self.into_parts();
        Self {
            relational: CompositeRelationalOwnerResult::settled_receipt(
                commit_identity,
                successor_basis,
                receipt,
            ),
            signal,
        }
    }

    pub(crate) fn with_relational_settlement_pending(
        self,
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        let (_, signal) = self.into_parts();
        Self {
            relational: CompositeRelationalOwnerResult::settlement_pending(
                commit_identity,
                successor_basis,
                settlement,
            ),
            signal,
        }
    }

    fn into_parts(self) -> (CompositeRelationalOwnerResult, CompositeSignalOwnerResult) {
        (self.relational, self.signal)
    }

    pub fn relational_posture(&self) -> CompositeComponentChangePosture {
        match self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeRelationalOwnerResultKind::Published { .. } => {
                CompositeComponentChangePosture::Published
            }
            CompositeRelationalOwnerResultKind::SettlementPending { .. } => {
                CompositeComponentChangePosture::Published
            }
            CompositeRelationalOwnerResultKind::SettlementRequired { .. } => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub fn signal_posture(&self) -> CompositeComponentChangePosture {
        self.signal.posture()
    }

    pub(crate) fn relational_publication_identity(
        &self,
    ) -> Option<worth_relational::facade::history::RelationalCommitIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published {
                commit_identity, ..
            }
            | CompositeRelationalOwnerResultKind::SettlementPending {
                commit_identity, ..
            }
            | CompositeRelationalOwnerResultKind::SettlementRequired {
                commit_identity, ..
            } => Some(commit_identity.clone()),
        }
    }

    pub(crate) fn relational_publication_basis_identity(
        &self,
    ) -> Option<&worth_relational::facade::branch::RelationalBranchBasisAdmissionIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published {
                successor_basis, ..
            }
            | CompositeRelationalOwnerResultKind::SettlementPending {
                successor_basis, ..
            }
            | CompositeRelationalOwnerResultKind::SettlementRequired {
                successor_basis, ..
            } => Some(successor_basis.admission_identity()),
        }
    }

    pub(crate) fn relational_settlement(
        &self,
    ) -> Option<&worth_relational::facade::history::RelationalCommitReceipt> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::Published { settlement, .. } => settlement.as_ref(),
            CompositeRelationalOwnerResultKind::RetainedExact
            | CompositeRelationalOwnerResultKind::SettlementPending { .. }
            | CompositeRelationalOwnerResultKind::SettlementRequired { .. } => None,
        }
    }

    pub(crate) fn relational_commit_result(&self) -> Option<&CommitResult> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::Published { result, .. } => result.as_ref(),
            CompositeRelationalOwnerResultKind::RetainedExact
            | CompositeRelationalOwnerResultKind::SettlementPending { .. }
            | CompositeRelationalOwnerResultKind::SettlementRequired { .. } => None,
        }
    }

    pub(crate) fn signal_publication_identity(
        &self,
    ) -> Option<crate::history::CompositeSignalPublicationIdentity> {
        self.signal.publication_identity()
    }

    pub(crate) fn matches_plan(&self, plan: &super::LoweredOwnerComponentPlan) -> bool {
        use super::RelationalComponentPlanPosture;

        let relational_matches = match plan.relational().posture() {
            RelationalComponentPlanPosture::RetainExact => {
                self.relational_posture() == CompositeComponentChangePosture::RetainExact
            }
            RelationalComponentPlanPosture::PublishPrepared
            | RelationalComponentPlanPosture::ForkThenPublish => {
                self.relational_posture() == CompositeComponentChangePosture::Published
            }
        };
        let signal_matches = self.signal.matches_plan(plan.signal().posture());
        relational_matches && signal_matches
    }
}
