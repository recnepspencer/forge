use super::{
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, CapabilityConfidenceLimits,
    CapabilityEvidenceClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilityAdmissionRequest {
    profile: BackendTargetProfile,
    evidence_basis: BackendCapabilityEvidenceBasis,
    support: BackendCapabilitySupportSet,
    media_assumptions: BackendMediaAssumptionSet,
    rebind_triggers: BackendRebindTriggers,
}

impl BackendCapabilityAdmissionRequest {
    pub(crate) const fn for_filesystem_qualification(
        profile: BackendTargetProfile,
        evidence_basis: BackendCapabilityEvidenceBasis,
        support: BackendCapabilitySupportSet,
        media_assumptions: BackendMediaAssumptionSet,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self::construct(
            profile,
            evidence_basis,
            support,
            media_assumptions,
            rebind_triggers,
        )
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn new(
        profile: BackendTargetProfile,
        evidence_basis: BackendCapabilityEvidenceBasis,
        support: BackendCapabilitySupportSet,
        media_assumptions: BackendMediaAssumptionSet,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self::construct(
            profile,
            evidence_basis,
            support,
            media_assumptions,
            rebind_triggers,
        )
    }

    const fn construct(
        profile: BackendTargetProfile,
        evidence_basis: BackendCapabilityEvidenceBasis,
        support: BackendCapabilitySupportSet,
        media_assumptions: BackendMediaAssumptionSet,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            profile,
            evidence_basis,
            support,
            media_assumptions,
            rebind_triggers,
        }
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_basis.evidence_class()
    }

    pub const fn evidence_basis(self) -> BackendCapabilityEvidenceBasis {
        self.evidence_basis
    }

    pub const fn support(self) -> BackendCapabilitySupportSet {
        self.support
    }

    pub const fn media_assumptions(self) -> BackendMediaAssumptionSet {
        self.media_assumptions
    }

    pub const fn rebind_triggers(self) -> BackendRebindTriggers {
        self.rebind_triggers
    }

    pub const fn confidence_limits(self) -> CapabilityConfidenceLimits {
        self.evidence_basis.confidence_limits()
    }
}
