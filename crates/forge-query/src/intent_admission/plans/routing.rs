use crate::runtime::ForgeQueryExistingTruthProbeRequest;

use super::{
    ForgeQueryAdmittedIntentPlanCore, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    request: ForgeQueryExistingTruthProbeRequest,
}

impl ForgeQueryExistingTruthProbeExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let request = eligibility
            .request()
            .existing_truth_probe_seed()
            .expect("probe routing plan requires probe routing seed")
            .request()
            .clone();
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            request,
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn request(&self) -> &ForgeQueryExistingTruthProbeRequest {
        &self.request
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &super::ForgeQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}
