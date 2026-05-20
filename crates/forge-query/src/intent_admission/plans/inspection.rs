use crate::intent_admission::eligibility::ForgeQueryDerivedViewIntentSeed;

use super::{
    ForgeQueryAdmittedIntentPlanCore, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationExecutionPlan {
    pub(crate) inner: ForgeQueryAdmittedIntentPlanCore,
    seed: ForgeQueryDerivedViewIntentSeed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedInspectionExecutionPlan {
    pub(crate) inner: ForgeQueryAdmittedIntentPlanCore,
    seed: ForgeQueryDerivedViewIntentSeed,
}

impl ForgeQueryDerivedMaterializationExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .derived_view_seed()
            .expect("derived materialization plan requires derived view seed")
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

    pub fn seed(&self) -> &ForgeQueryDerivedViewIntentSeed {
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

impl ForgeQueryDerivedInspectionExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .derived_view_seed()
            .expect("derived inspection plan requires derived view seed")
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

    pub fn seed(&self) -> &ForgeQueryDerivedViewIntentSeed {
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
