use super::{CapabilityConfidenceLimits, CapabilityEvidenceClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilityEvidenceBasis {
    evidence_class: CapabilityEvidenceClass,
    confidence_limits: CapabilityConfidenceLimits,
}

impl BackendCapabilityEvidenceBasis {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn declared_by_config(confidence_limit_count: u8) -> Self {
        Self {
            evidence_class: CapabilityEvidenceClass::DeclaredByConfig,
            confidence_limits: CapabilityConfidenceLimits::bounded_backend_and_media(
                confidence_limit_count,
            ),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn observed_by_probe(confidence_limit_count: u8) -> Self {
        Self {
            evidence_class: CapabilityEvidenceClass::ObservedByProbe,
            confidence_limits: CapabilityConfidenceLimits::bounded_backend_and_media(
                confidence_limit_count,
            ),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn externally_guaranteed(confidence_limit_count: u8) -> Self {
        Self {
            evidence_class: CapabilityEvidenceClass::ExternallyGuaranteed,
            confidence_limits: CapabilityConfidenceLimits::bounded_backend_and_media(
                confidence_limit_count,
            ),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn unverifiable_assumption() -> Self {
        Self {
            evidence_class: CapabilityEvidenceClass::UnverifiableAssumption,
            confidence_limits: CapabilityConfidenceLimits::unverifiable_assumption(),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn certified_backend_profile() -> Self {
        Self {
            evidence_class: CapabilityEvidenceClass::CertifiedBackendProfile,
            confidence_limits: CapabilityConfidenceLimits::certified_backend_profile(),
        }
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn confidence_limits(self) -> CapabilityConfidenceLimits {
        self.confidence_limits
    }
}
