use super::super::WorthUiPresentationAsyncObservation;
use super::WorthUiPresentationCompletionAdvance;

impl WorthUiPresentationCompletionAdvance {
    pub fn report(&self) -> &worth_runtime_bridge::facade::BridgeAsyncCompletionAdmissionReport {
        &self.report
    }

    pub fn batch(&self) -> &worth_query::facade::runtime::WorthQueryAsyncResultTransitionBatch {
        &self.batch
    }

    pub const fn observation(&self) -> WorthUiPresentationAsyncObservation {
        self.observation
    }
}
