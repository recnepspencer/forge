use crate::evidence::{HarnessDigestExpectation, HarnessEvidenceFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessExpectedObservation {
    EvidenceFamily(HarnessEvidenceFamily),
    Digest(HarnessDigestExpectation),
}

impl HarnessExpectedObservation {
    pub fn runtime_receipt() -> Self {
        Self::EvidenceFamily(HarnessEvidenceFamily::RuntimeReceipt)
    }

    pub fn visual_observation() -> Self {
        Self::EvidenceFamily(HarnessEvidenceFamily::VisibleFrameObservation)
    }

    pub fn active_plan_digest_derived_from_run() -> Self {
        Self::Digest(HarnessDigestExpectation::active_plan_derived_from_run())
    }

    pub fn artifact_digest_derived_from_run() -> Self {
        Self::Digest(HarnessDigestExpectation::artifact_derived_from_run())
    }

    pub fn active_plan_digest_fixed_for_diagnostics(expected: u64) -> Self {
        Self::Digest(HarnessDigestExpectation::active_plan_fixed_for_diagnostics(
            expected,
        ))
    }

    pub fn evidence_family(self) -> HarnessEvidenceFamily {
        match self {
            Self::EvidenceFamily(family) => family,
            Self::Digest(expectation) => expectation.family(),
        }
    }
}
