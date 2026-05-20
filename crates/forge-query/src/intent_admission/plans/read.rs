use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{ForgeQueryReadFamily, ForgeQueryRuntimeLiveSubscriptionInstallation};

use super::{
    ForgeQueryAdmittedIntentPlanCore, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    read_family: ForgeQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
}

impl ForgeQueryReadExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .read_execution_seed()
            .expect("read execution plan requires read execution seed")
            .clone();
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            read_family: seed.read_family().clone(),
            basis_context: seed.basis_context().cloned(),
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

    pub fn read_family(&self) -> &ForgeQueryReadFamily {
        &self.read_family
    }

    pub fn basis_context(&self) -> Option<&AdmittedQueryBasisContext> {
        self.basis_context.as_ref()
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

impl ForgeQueryLiveReadExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let installation = eligibility
            .request()
            .live_read_execution_seed()
            .expect("live read execution plan requires live read execution seed");
        let installation = installation.installation().clone();
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            installation,
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

    pub fn installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.installation
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
