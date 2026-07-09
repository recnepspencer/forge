use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, CapabilityConfidenceLimits,
    CapabilityEvidenceClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6BackendCapabilityReadinessPublication {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    support: BackendCapabilitySupportSet,
    media_assumptions: BackendMediaAssumptionSet,
    rebind_triggers: BackendRebindTriggers,
    confidence_limits: CapabilityConfidenceLimits,
}

impl S6BackendCapabilityReadinessPublication {
    pub fn from_admitted_backend_capability(witness: &AdmittedBackendCapabilityWitness) -> Self {
        Self {
            profile: witness.profile(),
            evidence_class: witness.evidence_class(),
            support: witness.support(),
            media_assumptions: witness.media_assumptions(),
            rebind_triggers: witness.rebind_triggers(),
            confidence_limits: witness.confidence_limits(),
        }
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn support(&self) -> BackendCapabilitySupportSet {
        self.support
    }

    pub const fn media_assumptions(&self) -> BackendMediaAssumptionSet {
        self.media_assumptions
    }

    pub const fn rebind_triggers(&self) -> BackendRebindTriggers {
        self.rebind_triggers
    }

    pub const fn confidence_limits(&self) -> CapabilityConfidenceLimits {
        self.confidence_limits
    }
}

pub fn publish_s6_backend_capability_readiness(
    witness: &AdmittedBackendCapabilityWitness,
) -> S6BackendCapabilityReadinessPublication {
    S6BackendCapabilityReadinessPublication::from_admitted_backend_capability(witness)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6BackendCapabilityAdmissionCertificationEvidence {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    support: BackendCapabilitySupportSet,
    media_assumptions: BackendMediaAssumptionSet,
    rebind_triggers: BackendRebindTriggers,
    confidence_limits: CapabilityConfidenceLimits,
}

impl S6BackendCapabilityAdmissionCertificationEvidence {
    pub fn from_admitted_backend_capability(
        witness: &AdmittedBackendCapabilityWitness,
        readiness: &S6BackendCapabilityReadinessPublication,
    ) -> Option<Self> {
        let evidence = Self {
            profile: witness.profile(),
            evidence_class: witness.evidence_class(),
            support: witness.support(),
            media_assumptions: witness.media_assumptions(),
            rebind_triggers: witness.rebind_triggers(),
            confidence_limits: witness.confidence_limits(),
        };
        evidence.matches_readiness(readiness).then_some(evidence)
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn support(&self) -> BackendCapabilitySupportSet {
        self.support
    }

    pub const fn media_assumptions(&self) -> BackendMediaAssumptionSet {
        self.media_assumptions
    }

    pub const fn rebind_triggers(&self) -> BackendRebindTriggers {
        self.rebind_triggers
    }

    pub const fn confidence_limits(&self) -> CapabilityConfidenceLimits {
        self.confidence_limits
    }

    fn matches_readiness(&self, readiness: &S6BackendCapabilityReadinessPublication) -> bool {
        self.profile == readiness.profile()
            && self.evidence_class == readiness.evidence_class()
            && self.support == readiness.support()
            && self.media_assumptions == readiness.media_assumptions()
            && self.rebind_triggers == readiness.rebind_triggers()
            && self.confidence_limits == readiness.confidence_limits()
    }
}

pub fn certify_s6_backend_capability_admission(
    witness: &AdmittedBackendCapabilityWitness,
    readiness: &S6BackendCapabilityReadinessPublication,
) -> Option<S6BackendCapabilityAdmissionCertificationEvidence> {
    S6BackendCapabilityAdmissionCertificationEvidence::from_admitted_backend_capability(
        witness, readiness,
    )
}
