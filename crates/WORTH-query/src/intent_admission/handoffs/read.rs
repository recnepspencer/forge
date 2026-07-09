use crate::identity::hash_parts;
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{WorthQueryReadFamily, WorthQueryRuntimeLiveSubscriptionInstallation};

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};
use crate::intent_admission::WorthQueryLiveReadExecutionPlan;
use crate::intent_admission::WorthQueryReadExecutionPlan;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryReadExecutionHandoff {
    read_family: WorthQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryLiveReadExecutionHandoff {
    installation: WorthQueryRuntimeLiveSubscriptionInstallation,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl WorthQueryReadExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryReadExecutionPlan) -> Self {
        let context_digest = plan
            .basis_context()
            .map(|context| context.basis_digest())
            .unwrap_or("runtime-current");
        let handoff_digest = hash_parts(&[
            "worth_query_read_execution_handoff_v1".to_string(),
            format!("family:{}", plan.family().as_str()),
            format!(
                "entrypoint:{}",
                if plan.basis_context().is_some() {
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
                        .as_str()
                } else {
                    WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily.as_str()
                }
            ),
            format!(
                "execution-seam:{}",
                plan.execution_seam()
                    .expect("read execution handoff requires execution seam")
                    .as_str()
            ),
            format!("decision:{}", plan.decision_digest()),
            format!("read-family:{}", plan.read_family().family_digest()),
            format!("basis:{context_digest}"),
        ]);
        Self {
            read_family: plan.read_family().clone(),
            basis_context: plan.basis_context().cloned(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        if self.basis_context.is_some() {
            WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
        } else {
            WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
        }
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
    }

    pub fn read_family(&self) -> &WorthQueryReadFamily {
        &self.read_family
    }

    pub fn basis_context(&self) -> Option<&AdmittedQueryBasisContext> {
        self.basis_context.as_ref()
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

impl WorthQueryLiveReadExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryLiveReadExecutionPlan) -> Self {
        let installation = plan.installation().clone();
        let handoff_digest = hash_parts(&[
            "worth_query_live_read_execution_handoff_v1".to_string(),
            format!("family:{}", plan.family().as_str()),
            format!(
                "entrypoint:{}",
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead.as_str()
            ),
            format!(
                "execution-seam:{}",
                plan.execution_seam()
                    .expect("live read execution handoff requires execution seam")
                    .as_str()
            ),
            format!("decision:{}", plan.decision_digest()),
            format!(
                "installation:{}",
                installation.installation_projection().label()
            ),
        ]);
        Self {
            installation,
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
    }

    pub fn installation(&self) -> &WorthQueryRuntimeLiveSubscriptionInstallation {
        &self.installation
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
