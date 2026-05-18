use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryGenericInspectionIntentSeed, ForgeQueryUnifiedInspectionExecutionPlan,
};

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryUnifiedInspectionExecutionHandoff {
    seed: ForgeQueryGenericInspectionIntentSeed,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl ForgeQueryUnifiedInspectionExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryUnifiedInspectionExecutionPlan) -> Self {
        let seed = plan.seed().clone();
        let handoff_digest = hash_parts(&[
            "forge_query_unified_inspection_execution_handoff_v1".to_string(),
            format!("decision:{}", plan.decision_digest()),
            format!("target:{}", seed.request_input_digest()),
        ]);
        Self {
            seed,
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
    }

    pub fn seed(&self) -> &ForgeQueryGenericInspectionIntentSeed {
        &self.seed
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}
