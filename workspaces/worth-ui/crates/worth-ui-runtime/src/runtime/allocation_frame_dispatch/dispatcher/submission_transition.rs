use super::UiAllocationFrameEpochAssignment;
use crate::runtime::UiAllocationFrameSubmissionOutcome;

/// Submission result plus the only witness that may advance active runtime epoch.
pub(crate) struct UiAllocationFrameSubmissionTransition {
    outcome: UiAllocationFrameSubmissionOutcome,
    epoch_assignment: Option<UiAllocationFrameEpochAssignment>,
    rejected_ingress: Option<super::UiAdmittedAllocationStreamIngress>,
}

impl UiAllocationFrameSubmissionTransition {
    pub(super) fn new(
        outcome: UiAllocationFrameSubmissionOutcome,
        epoch_assignment: Option<UiAllocationFrameEpochAssignment>,
    ) -> Self {
        Self {
            outcome,
            epoch_assignment,
            rejected_ingress: None,
        }
    }

    pub(super) fn backpressured(
        outcome: UiAllocationFrameSubmissionOutcome,
        epoch_assignment: Option<UiAllocationFrameEpochAssignment>,
        rejected_ingress: super::UiAdmittedAllocationStreamIngress,
    ) -> Self {
        Self {
            outcome,
            epoch_assignment,
            rejected_ingress: Some(rejected_ingress),
        }
    }

    pub(super) fn denied(outcome: UiAllocationFrameSubmissionOutcome) -> Self {
        Self::new(outcome, None)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiAllocationFrameSubmissionOutcome,
        Option<UiAllocationFrameEpochAssignment>,
        Option<super::UiAdmittedAllocationStreamIngress>,
    ) {
        (self.outcome, self.epoch_assignment, self.rejected_ingress)
    }

    pub(super) fn into_outcome(self) -> UiAllocationFrameSubmissionOutcome {
        self.outcome
    }

    #[cfg(test)]
    pub(super) fn outcome(self) -> UiAllocationFrameSubmissionOutcome {
        self.into_outcome()
    }
}
