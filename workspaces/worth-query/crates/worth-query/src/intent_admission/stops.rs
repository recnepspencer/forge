use crate::identity::hash_parts;

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentAdvisoryDecision, WorthQueryIntentViolationDecision,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdvisoryStop {
    advisory: WorthQueryIntentAdvisoryDecision,
    stop_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentViolationStop {
    violation: WorthQueryIntentViolationDecision,
    stop_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentNonAdmittedStop {
    Advisory(WorthQueryIntentAdvisoryStop),
    Violation(WorthQueryIntentViolationStop),
}

impl WorthQueryIntentAdvisoryStop {
    pub(crate) fn from_decision(advisory: WorthQueryIntentAdvisoryDecision) -> Self {
        let stop_digest = hash_parts(&[
            "worth_query_intent_advisory_stop_v1".to_string(),
            format!("family:{}", advisory.family().as_str()),
            format!("entrypoint:{}", advisory.entrypoint().as_str()),
            format!("request:{}", advisory.request_digest()),
            format!("eligibility:{}", advisory.eligibility_digest()),
            format!("decision:{}", advisory.decision_digest()),
        ]);
        Self {
            advisory,
            stop_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.advisory.family()
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.advisory.entrypoint()
    }

    pub fn stage(&self) -> &'static str {
        self.advisory.stage()
    }

    pub fn message(&self) -> &str {
        self.advisory.message()
    }

    pub fn request_digest(&self) -> &str {
        self.advisory.request_digest()
    }

    pub fn eligibility_digest(&self) -> &str {
        self.advisory.eligibility_digest()
    }

    pub fn decision_digest(&self) -> &str {
        self.advisory.decision_digest()
    }

    pub fn stop_digest(&self) -> &str {
        &self.stop_digest
    }

    pub(crate) fn into_violation_stop(self) -> WorthQueryIntentViolationStop {
        WorthQueryIntentViolationStop::from_decision(self.advisory.into_violation())
    }
}

impl WorthQueryIntentViolationStop {
    pub(crate) fn from_decision(violation: WorthQueryIntentViolationDecision) -> Self {
        let stop_digest = hash_parts(&[
            "worth_query_intent_violation_stop_v1".to_string(),
            format!("family:{}", violation.family().as_str()),
            format!("entrypoint:{}", violation.entrypoint().as_str()),
            format!("request:{}", violation.request_digest()),
            format!("eligibility:{}", violation.eligibility_digest()),
            format!("decision:{}", violation.decision_digest()),
        ]);
        Self {
            violation,
            stop_digest,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.violation.family()
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.violation.entrypoint()
    }

    pub fn stage(&self) -> &'static str {
        self.violation.stage()
    }

    pub fn message(&self) -> &str {
        self.violation.message()
    }

    pub fn request_digest(&self) -> &str {
        self.violation.request_digest()
    }

    pub fn eligibility_digest(&self) -> &str {
        self.violation.eligibility_digest()
    }

    pub fn decision_digest(&self) -> &str {
        self.violation.decision_digest()
    }

    pub fn stop_digest(&self) -> &str {
        &self.stop_digest
    }

    pub fn violation(&self) -> &WorthQueryIntentViolationDecision {
        &self.violation
    }
}

impl WorthQueryIntentNonAdmittedStop {
    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        match self {
            Self::Advisory(stop) => stop.family(),
            Self::Violation(stop) => stop.family(),
        }
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        match self {
            Self::Advisory(stop) => stop.entrypoint(),
            Self::Violation(stop) => stop.entrypoint(),
        }
    }

    pub fn stage(&self) -> &'static str {
        match self {
            Self::Advisory(stop) => stop.stage(),
            Self::Violation(stop) => stop.stage(),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Advisory(stop) => stop.message(),
            Self::Violation(stop) => stop.message(),
        }
    }

    pub fn request_digest(&self) -> &str {
        match self {
            Self::Advisory(stop) => stop.request_digest(),
            Self::Violation(stop) => stop.request_digest(),
        }
    }

    pub fn eligibility_digest(&self) -> &str {
        match self {
            Self::Advisory(stop) => stop.eligibility_digest(),
            Self::Violation(stop) => stop.eligibility_digest(),
        }
    }

    pub fn decision_digest(&self) -> &str {
        match self {
            Self::Advisory(stop) => stop.decision_digest(),
            Self::Violation(stop) => stop.decision_digest(),
        }
    }

    pub fn stop_digest(&self) -> &str {
        match self {
            Self::Advisory(stop) => stop.stop_digest(),
            Self::Violation(stop) => stop.stop_digest(),
        }
    }

    pub(crate) fn into_violation_stop(self) -> WorthQueryIntentViolationStop {
        match self {
            Self::Advisory(stop) => stop.into_violation_stop(),
            Self::Violation(stop) => stop,
        }
    }
}
