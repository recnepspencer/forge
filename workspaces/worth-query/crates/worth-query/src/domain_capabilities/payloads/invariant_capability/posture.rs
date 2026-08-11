use super::super::common::WorthQueryDomainCapabilitySemanticPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantCapabilityContributionPosture {
    CapabilityGap,
    InvariantDenial,
    SupportSummary,
    InvariantRegistration,
}

impl WorthQueryInvariantCapabilityContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityGap => "capability-gap",
            Self::InvariantDenial => "invariant-denial",
            Self::SupportSummary => "support-summary",
            Self::InvariantRegistration => "invariant-registration",
        }
    }

    pub const fn semantic_posture(self) -> WorthQueryDomainCapabilitySemanticPosture {
        match self {
            Self::CapabilityGap => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantCapabilityGap
            }
            Self::InvariantDenial => WorthQueryDomainCapabilitySemanticPosture::InvariantDenial,
            Self::SupportSummary => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantSupportSummary
            }
            Self::InvariantRegistration => {
                WorthQueryDomainCapabilitySemanticPosture::InvariantRegistration
            }
        }
    }
}
