mod compilation;
mod outcome;

pub use outcome::{
    UiSourceCompilationDenialReceipt, UiSourceRebindAttemptBasis, UiSourceRebindAttemptDenial,
    UiSourceRebindAttemptDenialReceipt, UiSourceRebindAttemptFailure, UiSourceRebindAttemptOutcome,
};

use super::WorthUiSettledSourceSnapshot;

impl WorthUiSettledSourceSnapshot {
    pub fn attempt_source_rebind(
        self,
        snapshot: &crate::capability::CapabilitySnapshot,
    ) -> UiSourceRebindAttemptOutcome {
        compilation::attempt(self, snapshot)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub fn attempt_candidate_for_certification(
        self,
        snapshot: &crate::capability::CapabilitySnapshot,
    ) -> Result<
        super::WorthUiWatchedCandidateSubmission,
        super::WorthUiWatchedCandidateSubmissionDenial,
    > {
        self.attempt_source_rebind(snapshot)
            .into_candidate_submission()
            .map_err(UiSourceRebindAttemptFailure::into_migration_denial)
    }
}
