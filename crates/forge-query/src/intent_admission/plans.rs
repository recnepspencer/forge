use crate::identity::hash_parts;
use crate::runtime::ForgeQueryIntentDeclaration;

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeIntentExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectTriggeredIntentExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmittedIntentPlan {
    Authoritative(ForgeQueryAuthoritativeIntentExecutionPlan),
    EffectTriggered(ForgeQueryEffectTriggeredIntentExecutionPlan),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryAdmittedIntentPlanCore {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    request_digest: String,
    eligibility_digest: String,
    declaration: ForgeQueryIntentDeclaration,
    decision_digest: String,
}

impl ForgeQueryAuthoritativeIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(eligibility, execution_seam),
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.inner.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl ForgeQueryEffectTriggeredIntentExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(eligibility, execution_seam),
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.inner.execution_seam
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.inner.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl ForgeQueryAdmittedIntentPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        match eligibility.request().family() {
            ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent => Self::Authoritative(
                ForgeQueryAuthoritativeIntentExecutionPlan::from_eligibility(
                    eligibility,
                    execution_seam,
                ),
            ),
            ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent => Self::EffectTriggered(
                ForgeQueryEffectTriggeredIntentExecutionPlan::from_eligibility(
                    eligibility,
                    execution_seam,
                ),
            ),
            family => panic!("phase 3 covered floor cannot mint admitted plans for {family:?}"),
        }
    }

    fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        match self {
            Self::Authoritative(plan) => plan.core(),
            Self::EffectTriggered(plan) => plan.core(),
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.core().family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.core().entrypoint
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.core().execution_seam
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.core().declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.core().request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.core().eligibility_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.core().decision_digest
    }
}

impl ForgeQueryAdmittedIntentPlanCore {
    fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let decision_digest = hash_parts(&[
            "forge_query_intent_admission_decision_v1".to_string(),
            format!("family:{}", eligibility.request().family().as_str()),
            format!("entrypoint:{}", eligibility.request().entrypoint().as_str()),
            format!("request:{}", eligibility.request().request_digest()),
            format!("eligibility:{}", eligibility.eligibility_digest()),
            "decision:admitted".to_string(),
        ]);
        Self {
            family: eligibility.request().family(),
            entrypoint: eligibility.request().entrypoint(),
            execution_seam,
            request_digest: eligibility.request().request_digest().to_string(),
            eligibility_digest: eligibility.eligibility_digest().to_string(),
            declaration: eligibility.request().declaration().clone(),
            decision_digest,
        }
    }
}
