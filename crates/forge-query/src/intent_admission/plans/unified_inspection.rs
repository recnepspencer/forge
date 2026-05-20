use crate::intent_admission::eligibility::ForgeQueryGenericInspectionIntentSeed;

use super::{
    ForgeQueryAdmittedIntentPlanCore, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryUnifiedInspectionExecutionPlan {
    pub(crate) inner: ForgeQueryAdmittedIntentPlanCore,
    seed: ForgeQueryGenericInspectionIntentSeed,
}

impl ForgeQueryUnifiedInspectionExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .generic_inspection_seed()
            .expect("unified inspection plan requires generic inspection seed")
            .clone();
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            seed,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn seed(&self) -> &ForgeQueryGenericInspectionIntentSeed {
        &self.seed
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
