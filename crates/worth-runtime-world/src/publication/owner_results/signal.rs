use crate::history::CompositeComponentChangePosture;

use super::super::SignalComponentPlanPosture;

use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

/// Exact result of the Signal leg, including the distinct fork-and-advance
/// operation so history cannot confuse a created-only branch with a moved one.
#[derive(Debug)]
pub struct CompositeSignalOwnerResult {
    result: CompositeSignalOwnerResultKind,
}

#[derive(Debug)]
enum CompositeSignalOwnerResultKind {
    RetainedExact,
    Advanced(SignalBranchAdvanceOutcome),
    Forked(SignalBranchForkOutcome),
    ForkedAndAdvanced {
        forked: SignalBranchForkOutcome,
        advanced: SignalBranchAdvanceOutcome,
    },
}

impl CompositeSignalOwnerResult {
    pub(crate) fn retained() -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::RetainedExact,
        }
    }

    pub(crate) fn advanced(result: SignalBranchAdvanceOutcome) -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::Advanced(result),
        }
    }

    pub(crate) fn forked(result: SignalBranchForkOutcome) -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::Forked(result),
        }
    }

    pub(crate) fn forked_and_advanced(
        forked: SignalBranchForkOutcome,
        advanced: SignalBranchAdvanceOutcome,
    ) -> Self {
        Self {
            result: CompositeSignalOwnerResultKind::ForkedAndAdvanced { forked, advanced },
        }
    }

    pub(crate) fn posture(&self) -> CompositeComponentChangePosture {
        match self.result {
            CompositeSignalOwnerResultKind::RetainedExact => {
                CompositeComponentChangePosture::RetainExact
            }
            CompositeSignalOwnerResultKind::Advanced(_)
            | CompositeSignalOwnerResultKind::Forked(_)
            | CompositeSignalOwnerResultKind::ForkedAndAdvanced { .. } => {
                CompositeComponentChangePosture::Published
            }
        }
    }

    pub(crate) fn publication_identity(
        &self,
    ) -> Option<crate::history::CompositeSignalPublicationIdentity> {
        match &self.result {
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
            CompositeSignalOwnerResultKind::ForkedAndAdvanced { advanced, .. } => Some(
                crate::history::CompositeSignalPublicationIdentity::ForkedAndAdvanced(
                    advanced.advanced_basis().admission_identity().clone(),
                ),
            ),
        }
    }

    pub(crate) fn matches_plan(&self, posture: SignalComponentPlanPosture) -> bool {
        match posture {
            SignalComponentPlanPosture::RetainExact => {
                self.posture() == CompositeComponentChangePosture::RetainExact
            }
            SignalComponentPlanPosture::AdvanceExact => {
                self.posture() == CompositeComponentChangePosture::Published
            }
            SignalComponentPlanPosture::ForkExact => {
                matches!(self.result, CompositeSignalOwnerResultKind::Forked(_))
            }
            SignalComponentPlanPosture::ForkAndAdvance => matches!(
                self.result,
                CompositeSignalOwnerResultKind::ForkedAndAdvanced { .. }
            ),
        }
    }
}
