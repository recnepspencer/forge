use crate::intent_admission::eligibility::WorthQueryDerivedViewIntentSeed;

use super::{
    WorthQueryAdmittedIntentPlanCore, WorthQueryIntentAdmissionEligibility,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedMaterializationExecutionPlan {
    pub(crate) inner: WorthQueryAdmittedIntentPlanCore,
    seed: WorthQueryDerivedViewIntentSeed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedInspectionExecutionPlan {
    pub(crate) inner: WorthQueryAdmittedIntentPlanCore,
    seed: WorthQueryDerivedViewIntentSeed,
}

impl WorthQueryDerivedMaterializationExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .derived_view_seed()
            .expect("derived materialization plan requires derived view seed")
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

    pub fn seed(&self) -> &WorthQueryDerivedViewIntentSeed {
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

impl WorthQueryDerivedInspectionExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .derived_view_seed()
            .expect("derived inspection plan requires derived view seed")
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

    pub fn seed(&self) -> &WorthQueryDerivedViewIntentSeed {
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
