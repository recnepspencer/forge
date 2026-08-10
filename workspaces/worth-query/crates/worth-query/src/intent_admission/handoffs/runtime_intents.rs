use crate::identity::hash_parts;
use crate::runtime::WorthQueryIntentDeclaration;

use super::{
    WorthQueryAuthoritativeIntentExecutionPlan, WorthQueryEffectTriggeredIntentExecutionPlan,
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeIntentExecutionHandoff {
    declaration: WorthQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryEffectTriggeredIntentExecutionHandoff {
    declaration: WorthQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl WorthQueryAuthoritativeIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryAuthoritativeIntentExecutionPlan) -> Self {
        Self {
            declaration: plan.declaration().clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("authoritative runtime handoff requires execution seam"),
                plan.decision_digest(),
                plan.declaration(),
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        &self.declaration
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

impl WorthQueryEffectTriggeredIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryEffectTriggeredIntentExecutionPlan) -> Self {
        Self {
            declaration: plan.declaration().clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("effect runtime handoff requires execution seam"),
                plan.decision_digest(),
                plan.declaration(),
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::EffectTriggeredWriteIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        &self.declaration
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

fn handoff_digest(
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    decision_digest: &str,
    declaration: &WorthQueryIntentDeclaration,
) -> String {
    hash_parts(&[
        "worth_query_admitted_intent_execution_handoff_v1".to_string(),
        format!("family:{}", family.as_str()),
        format!("entrypoint:{}", entrypoint.as_str()),
        format!("execution-seam:{}", execution_seam.as_str()),
        format!("decision:{decision_digest}"),
        format!("intent:{}", declaration.input_digest()),
    ])
}
