use crate::identity::hash_parts;
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{ForgeQueryReadFamily, ForgeQueryRuntimeLiveSubscriptionInstallation};

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentEligibilityTraceEvidence,
};
use crate::intent_admission::ForgeQueryLiveReadExecutionPlan;
use crate::intent_admission::ForgeQueryReadExecutionPlan;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadExecutionHandoff {
    read_family: ForgeQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadExecutionHandoff {
    installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl ForgeQueryReadExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryReadExecutionPlan) -> Self {
        let context_digest = plan
            .basis_context()
            .map(|context| context.basis_digest())
            .unwrap_or("runtime-current");
        let handoff_digest = hash_parts(&[
            "forge_query_read_execution_handoff_v1".to_string(),
            format!("family:{}", plan.family().as_str()),
            format!(
                "entrypoint:{}",
                if plan.basis_context().is_some() {
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
                        .as_str()
                } else {
                    ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily.as_str()
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        if self.basis_context.is_some() {
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
        } else {
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
        }
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
    }

    pub fn read_family(&self) -> &ForgeQueryReadFamily {
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

impl ForgeQueryLiveReadExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryLiveReadExecutionPlan) -> Self {
        let installation = plan.installation().clone();
        let handoff_digest = hash_parts(&[
            "forge_query_live_read_execution_handoff_v1".to_string(),
            format!("family:{}", plan.family().as_str()),
            format!(
                "entrypoint:{}",
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead.as_str()
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
    }

    pub fn installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.installation
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
