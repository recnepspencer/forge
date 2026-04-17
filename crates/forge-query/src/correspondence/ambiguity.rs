use super::outcome::{AdvisoryStructuralAmbiguous, LineageStructuralDisagreement};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceAmbiguityEnvelope {
    outcome: AdvisoryStructuralAmbiguous,
    reason: &'static str,
}

impl CorrespondenceAmbiguityEnvelope {
    pub fn outcome(&self) -> &AdvisoryStructuralAmbiguous {
        &self.outcome
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn new(outcome: AdvisoryStructuralAmbiguous, reason: &'static str) -> Self {
        Self { outcome, reason }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceDisagreementEnvelope {
    outcome: LineageStructuralDisagreement,
    reason: &'static str,
}

impl CorrespondenceDisagreementEnvelope {
    pub fn outcome(&self) -> &LineageStructuralDisagreement {
        &self.outcome
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn new(outcome: LineageStructuralDisagreement, reason: &'static str) -> Self {
        Self { outcome, reason }
    }
}
