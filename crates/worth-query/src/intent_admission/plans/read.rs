use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{WorthQueryReadFamily, WorthQueryRuntimeLiveSubscriptionInstallation};

use super::{
    WorthQueryAdmittedIntentPlanCore, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryReadExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    read_family: WorthQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryLiveReadExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    installation: WorthQueryRuntimeLiveSubscriptionInstallation,
}

impl WorthQueryReadExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .read_execution_seed()
            .expect("read execution plan requires read execution seed")
            .clone();
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            read_family: seed.read_family().clone(),
            basis_context: seed.basis_context().cloned(),
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

    pub fn read_family(&self) -> &WorthQueryReadFamily {
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

    pub fn eligibility_trace(&self) -> &super::WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl WorthQueryLiveReadExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let installation = eligibility
            .request()
            .live_read_execution_seed()
            .expect("live read execution plan requires live read execution seed");
        let installation = installation.installation().clone();
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            installation,
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

    pub fn installation(&self) -> &WorthQueryRuntimeLiveSubscriptionInstallation {
        &self.installation
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
