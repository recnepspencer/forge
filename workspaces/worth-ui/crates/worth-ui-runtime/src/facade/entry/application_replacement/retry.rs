use super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiApplicationCutoverRetry, WorthUiApplicationReplacementOutcome,
};

impl WorthUiApplicationCutoverRetry {
    pub fn retry(
        self,
        session: &mut WorthUiActiveApplicationSession,
        boundary: crate::runtime::WorthUiFrameBoundary,
    ) -> Result<WorthUiApplicationReplacementOutcome, WorthUiApplicationCutoverDenial> {
        session.activate_prepared_replacement(
            self.pending,
            self.admitted_delta,
            boundary,
            self.lane_parity_report,
        )
    }
}

impl std::fmt::Debug for WorthUiApplicationCutoverRetry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiApplicationCutoverRetry")
            .field("candidate_generation", self.pending.basis.next_generation())
            .finish_non_exhaustive()
    }
}
