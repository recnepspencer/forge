use super::vocabulary::{CapabilityConfidenceScope, CapabilityResidualRisk};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityConfidenceLimits {
    residual_risk: CapabilityResidualRisk,
    scope: CapabilityConfidenceScope,
    confidence_limit_count: u8,
}

impl CapabilityConfidenceLimits {
    pub const fn bounded_backend_and_media(confidence_limit_count: u8) -> Self {
        Self {
            residual_risk: CapabilityResidualRisk::Bounded,
            scope: CapabilityConfidenceScope::BackendAndMedia,
            confidence_limit_count,
        }
    }

    pub const fn certified_backend_profile() -> Self {
        Self {
            residual_risk: CapabilityResidualRisk::None,
            scope: CapabilityConfidenceScope::BackendProfile,
            confidence_limit_count: 0,
        }
    }

    pub const fn certification_only(confidence_limit_count: u8) -> Self {
        Self {
            residual_risk: CapabilityResidualRisk::Bounded,
            scope: CapabilityConfidenceScope::CertificationOnly,
            confidence_limit_count,
        }
    }

    pub const fn unverifiable_assumption() -> Self {
        Self {
            residual_risk: CapabilityResidualRisk::Unverifiable,
            scope: CapabilityConfidenceScope::UnboundedAssumption,
            confidence_limit_count: u8::MAX,
        }
    }

    pub const fn residual_risk(self) -> CapabilityResidualRisk {
        self.residual_risk
    }

    pub const fn scope(self) -> CapabilityConfidenceScope {
        self.scope
    }

    pub const fn confidence_limit_count(self) -> u8 {
        self.confidence_limit_count
    }

    pub const fn can_back_runtime_claim(self) -> bool {
        !matches!(
            (self.residual_risk, self.scope),
            (CapabilityResidualRisk::Unverifiable, _)
                | (_, CapabilityConfidenceScope::CertificationOnly)
                | (_, CapabilityConfidenceScope::UnboundedAssumption)
        )
    }
}
