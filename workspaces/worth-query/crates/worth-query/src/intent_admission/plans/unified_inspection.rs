use crate::intent_admission::eligibility::WorthQueryGenericInspectionIntentSeed;

use super::{
    WorthQueryAdmittedIntentPlanCore, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryUnifiedInspectionExecutionPlan {
    pub(crate) inner: WorthQueryAdmittedIntentPlanCore,
    seed: WorthQueryGenericInspectionIntentSeed,
}

impl WorthQueryUnifiedInspectionExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .generic_inspection_seed()
            .expect("unified inspection plan requires generic inspection seed")
            .clone();
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            seed,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn seed(&self) -> &WorthQueryGenericInspectionIntentSeed {
        &self.seed
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
