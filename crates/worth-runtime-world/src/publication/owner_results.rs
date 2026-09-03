use worth_relational::facade::mvcc::PerformedRelationalCommit;
use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

use crate::history::CompositeComponentChangePosture;

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
        performed: PerformedRelationalCommit,
        settlement: Option<worth_relational::facade::history::RelationalCommitReceipt>,
    },
}

/// Exact result of the Signal leg. A changed Signal component must carry the
/// owner-issued advance/fork result.
#[derive(Debug)]
pub struct CompositeSignalOwnerResult {
    result: CompositeSignalOwnerResultKind,
}

#[derive(Debug)]
enum CompositeSignalOwnerResultKind {
    RetainedExact,
    Advanced(SignalBranchAdvanceOutcome),
    Forked(SignalBranchForkOutcome),
}

impl CompositeRelationalOwnerResult {
    pub(super) fn retained() -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::RetainedExact,
        }
    }

    pub(super) fn published(
        performed: PerformedRelationalCommit,
        settlement: Option<worth_relational::facade::history::RelationalCommitReceipt>,
    ) -> Self {
        Self {
            result: CompositeRelationalOwnerResultKind::Published {
                performed,
                settlement,
            },
        }
    }
}

impl CompositeSignalOwnerResult {
    pub(super) fn retained() -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::RetainedExact,
        }
    }

    pub(super) fn advanced(result: SignalBranchAdvanceOutcome) -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::Advanced(result),
        }
    }

    pub(super) fn forked(result: SignalBranchForkOutcome) -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::Forked(result),
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

    pub(crate) fn relational_published(
        performed: PerformedRelationalCommit,
        signal: CompositeSignalOwnerResult,
    ) -> Self {
        Self {
            relational: CompositeRelationalOwnerResult::published(performed, None),
            signal,
        }
    }

    pub(crate) fn signal_advanced(
        relational: CompositeRelationalOwnerResult,
        advanced: SignalBranchAdvanceOutcome,
    ) -> Self {
        Self {
            relational,
            signal: CompositeSignalOwnerResult::advanced(advanced),
        }
    }

    pub(crate) fn signal_forked(
        relational: CompositeRelationalOwnerResult,
        forked: SignalBranchForkOutcome,
    ) -> Self {
        Self {
            relational,
            signal: CompositeSignalOwnerResult::forked(forked),
        }
    }

    pub fn relational_posture(&self) -> CompositeComponentChangePosture {
        match self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeRelationalOwnerResultKind::Published { .. } => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub fn signal_posture(&self) -> CompositeComponentChangePosture {
        match self.signal.result {
            CompositeSignalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeSignalOwnerResultKind::Advanced(_)
            | CompositeSignalOwnerResultKind::Forked(_) => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub(crate) fn relational_publication_identity(
        &self,
    ) -> Option<worth_relational::facade::history::RelationalCommitIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published { performed, .. } => {
                Some(performed.commit_identity())
            }
        }
    }

    pub(crate) fn relational_publication_basis_identity(
        &self,
    ) -> Option<&worth_relational::facade::branch::RelationalBranchBasisAdmissionIdentity> {
        match &self.relational.result {
            CompositeRelationalOwnerResultKind::RetainedExact => None,
            CompositeRelationalOwnerResultKind::Published { performed, .. } => {
                Some(performed.next_basis().admission_identity())
            }
        }
    }

    pub(crate) fn signal_publication_identity(
        &self,
    ) -> Option<crate::history::CompositeSignalPublicationIdentity> {
        match &self.signal.result {
            CompositeSignalOwnerResultKind::RetainedExact => None,
            CompositeSignalOwnerResultKind::Advanced(result) => Some(
                crate::history::CompositeSignalPublicationIdentity::Advanced(
                    result.advanced_basis().admission_identity().clone(),
                ),
            ),
            CompositeSignalOwnerResultKind::Forked(result) => {
                Some(crate::history::CompositeSignalPublicationIdentity::Forked(
                    result.created_basis().admission_identity().clone(),
                ))
            }
        }
    }

    pub(crate) fn matches_plan(&self, plan: &super::LoweredOwnerComponentPlan) -> bool {
        use super::{RelationalComponentPlanPosture, SignalComponentPlanPosture};

        let relational_matches = match plan.relational().posture() {
            RelationalComponentPlanPosture::RetainExact => {
                self.relational_posture() == CompositeComponentChangePosture::RetainExact
            }
            RelationalComponentPlanPosture::PublishPrepared
            | RelationalComponentPlanPosture::ForkThenPublish => {
                self.relational_posture() == CompositeComponentChangePosture::Published
            }
        };
        let signal_matches = match plan.signal().posture() {
            SignalComponentPlanPosture::RetainExact => {
                self.signal_posture() == CompositeComponentChangePosture::RetainExact
            }
            SignalComponentPlanPosture::AdvanceExact
            | SignalComponentPlanPosture::ForkThenAdvance => {
                self.signal_posture() == CompositeComponentChangePosture::Published
            }
        };
        relational_matches && signal_matches
    }
}
