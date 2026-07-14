use crate::identity::hash_parts;
use crate::runtime::WorthQueryExistingTruthProbeRequest;

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};
use crate::intent_admission::WorthQueryExistingTruthProbeExecutionPlan;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryExistingTruthProbeExecutionHandoff {
    request: WorthQueryExistingTruthProbeRequest,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl WorthQueryExistingTruthProbeExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryExistingTruthProbeExecutionPlan) -> Self {
        let request = plan.request().clone();
        let handoff_digest = hash_parts(&[
            "worth_query_existing_truth_probe_execution_handoff_v1".to_string(),
            format!("family:{}", plan.family().as_str()),
            format!(
                "entrypoint:{}",
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
                    .as_str()
            ),
            format!(
                "execution-seam:{}",
                plan.execution_seam()
                    .expect("probe routing handoff requires execution seam")
                    .as_str()
            ),
            format!("decision:{}", plan.decision_digest()),
            format!("request:{}", request.request_digest()),
        ]);
        Self {
            request,
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute
    }

    pub fn request(&self) -> &WorthQueryExistingTruthProbeRequest {
        &self.request
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}
