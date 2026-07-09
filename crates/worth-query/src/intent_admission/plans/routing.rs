use crate::runtime::WorthQueryExistingTruthProbeRequest;

use super::{
    WorthQueryAdmittedIntentPlanCore, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryExistingTruthProbeExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    request: WorthQueryExistingTruthProbeRequest,
}

impl WorthQueryExistingTruthProbeExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let request = eligibility
            .request()
            .existing_truth_probe_seed()
            .expect("probe routing plan requires probe routing seed")
            .request()
            .clone();
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            request,
        }
    }

    pub(crate) fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn request(&self) -> &WorthQueryExistingTruthProbeRequest {
        &self.request
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &super::WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}
