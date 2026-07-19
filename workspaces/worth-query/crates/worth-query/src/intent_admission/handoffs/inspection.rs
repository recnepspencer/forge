use crate::identity::hash_parts;
use crate::intent_admission::{
    WorthQueryDerivedInspectionExecutionPlan, WorthQueryDerivedMaterializationExecutionPlan,
};

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedMaterializationExecutionHandoff {
    view_name: String,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedInspectionExecutionHandoff {
    view_name: String,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl WorthQueryDerivedMaterializationExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryDerivedMaterializationExecutionPlan) -> Self {
        let view_name = plan.seed().view_name().to_string();
        let handoff_digest = hash_parts(&[
            "worth_query_derived_materialization_execution_handoff_v1".to_string(),
            format!("decision:{}", plan.decision_digest()),
            format!("view:{view_name}"),
        ]);
        Self {
            view_name,
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
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

impl WorthQueryDerivedInspectionExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryDerivedInspectionExecutionPlan) -> Self {
        let view_name = plan.seed().view_name().to_string();
        let handoff_digest = hash_parts(&[
            "worth_query_derived_inspection_execution_handoff_v1".to_string(),
            format!("decision:{}", plan.decision_digest()),
            format!("view:{view_name}"),
        ]);
        Self {
            view_name,
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
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
