use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryDerivedInspectionExecutionPlan, ForgeQueryDerivedMaterializationExecutionPlan,
};

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationExecutionHandoff {
    view_name: String,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedInspectionExecutionHandoff {
    view_name: String,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl ForgeQueryDerivedMaterializationExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryDerivedMaterializationExecutionPlan) -> Self {
        let view_name = plan.seed().view_name().to_string();
        let handoff_digest = hash_parts(&[
            "forge_query_derived_materialization_execution_handoff_v1".to_string(),
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
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

impl ForgeQueryDerivedInspectionExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryDerivedInspectionExecutionPlan) -> Self {
        let view_name = plan.seed().view_name().to_string();
        let handoff_digest = hash_parts(&[
            "forge_query_derived_inspection_execution_handoff_v1".to_string(),
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
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
