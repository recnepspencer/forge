use crate::identity::hash_parts;

use super::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdvisoryStop, ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentViolationStop,
    ForgeQueryRawIntentAdmissionRequest,
};

pub fn admit_runtime_intent_request(
    request: ForgeQueryRawIntentAdmissionRequest,
) -> ForgeQueryIntentAdmissionDecision {
    let eligibility = ForgeQueryIntentAdmissionEligibility::from_request(request);
    match eligibility.pre_decision_posture() {
        ForgeQueryIntentAdmissionPreDecisionPosture::Admitted => {
            ForgeQueryIntentAdmissionDecision::Admitted(
                ForgeQueryAdmittedIntentPlan::from_eligibility(eligibility),
            )
        }
        ForgeQueryIntentAdmissionPreDecisionPosture::Deferred { stage, message } => {
            ForgeQueryIntentAdmissionDecision::Advisory(ForgeQueryIntentAdvisoryDecision::new(
                eligibility.request().family(),
                eligibility.request().entrypoint(),
                stage,
                message,
                eligibility.request().request_digest(),
                eligibility.eligibility_digest(),
            ))
        }
        ForgeQueryIntentAdmissionPreDecisionPosture::Violation { stage, message } => {
            ForgeQueryIntentAdmissionDecision::Violation(
                ForgeQueryIntentViolationDecision::from_eligibility_violation(
                    &eligibility,
                    stage,
                    message,
                ),
            )
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionDecision {
    Admitted(ForgeQueryAdmittedIntentPlan),
    Advisory(ForgeQueryIntentAdvisoryDecision),
    Violation(ForgeQueryIntentViolationDecision),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdvisoryDecision {
    family: super::ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    stage: &'static str,
    message: String,
    request_digest: String,
    eligibility_digest: String,
    decision_digest: String,
}

impl ForgeQueryIntentAdvisoryDecision {
    pub(crate) fn new(
        family: super::ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        stage: &'static str,
        message: impl Into<String>,
        request_digest: &str,
        eligibility_digest: &str,
    ) -> Self {
        let message = message.into();
        let decision_digest = hash_parts(&[
            "forge_query_intent_advisory_decision_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!("stage:{stage}"),
            format!("message:{message}"),
            format!("request:{request_digest}"),
            format!("eligibility:{eligibility_digest}"),
        ]);
        Self {
            family,
            entrypoint,
            stage,
            message,
            request_digest: request_digest.to_string(),
            eligibility_digest: eligibility_digest.to_string(),
            decision_digest,
        }
    }

    pub fn family(&self) -> super::ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub(crate) fn into_violation(self) -> ForgeQueryIntentViolationDecision {
        ForgeQueryIntentViolationDecision::new(
            self.family(),
            self.entrypoint(),
            self.stage(),
            self.message(),
            self.request_digest(),
            self.eligibility_digest(),
        )
    }

    pub fn into_stop(self) -> ForgeQueryIntentAdvisoryStop {
        ForgeQueryIntentAdvisoryStop::from_decision(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentViolationDecision {
    family: super::ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    stage: &'static str,
    message: String,
    request_digest: String,
    eligibility_digest: String,
    decision_digest: String,
}

impl ForgeQueryIntentViolationDecision {
    pub(crate) fn new(
        family: super::ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        stage: &'static str,
        message: impl Into<String>,
        request_digest: &str,
        eligibility_digest: &str,
    ) -> Self {
        let message = message.into();
        let decision_digest = hash_parts(&[
            "forge_query_intent_violation_decision_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!("stage:{stage}"),
            format!("message:{message}"),
            format!("request:{request_digest}"),
            format!("eligibility:{eligibility_digest}"),
        ]);
        Self {
            family,
            entrypoint,
            stage,
            message,
            request_digest: request_digest.to_string(),
            eligibility_digest: eligibility_digest.to_string(),
            decision_digest,
        }
    }

    pub(crate) fn from_eligibility_denial(
        eligibility: &ForgeQueryIntentAdmissionEligibility,
        stage: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            eligibility.request().family(),
            eligibility.request().entrypoint(),
            stage,
            message,
            eligibility.request().request_digest(),
            eligibility.eligibility_digest(),
        )
    }

    pub(crate) fn from_eligibility_violation(
        eligibility: &ForgeQueryIntentAdmissionEligibility,
        stage: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::from_eligibility_denial(eligibility, stage, message)
    }

    pub fn family(&self) -> super::ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn into_stop(self) -> ForgeQueryIntentViolationStop {
        ForgeQueryIntentViolationStop::from_decision(self)
    }
}

impl ForgeQueryIntentAdmissionDecision {
    pub fn into_non_admitted_stop(self) -> Option<ForgeQueryIntentNonAdmittedStop> {
        match self {
            Self::Admitted(_) => None,
            Self::Advisory(advisory) => Some(ForgeQueryIntentNonAdmittedStop::Advisory(
                advisory.into_stop(),
            )),
            Self::Violation(violation) => Some(ForgeQueryIntentNonAdmittedStop::Violation(
                violation.into_stop(),
            )),
        }
    }
}
