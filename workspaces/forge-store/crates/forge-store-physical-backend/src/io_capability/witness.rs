use super::{
    BackendCapabilityAdmissionDenial, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, CapabilityConfidenceLimits, CapabilityEvidenceClass,
};

pub type BackendCapabilityClaimOutcome = forge_proof::prelude::ProofOutcome<
    BackendCapabilityClaimWitness,
    BackendCapabilityAdmissionDenial,
    core::convert::Infallible,
    BackendCapabilityStale,
    BackendCapabilityRebindRequired,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmittedBackendCapabilityWitness {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    support: BackendCapabilitySupportSet,
    media_assumptions: BackendMediaAssumptionSet,
    rebind_triggers: BackendRebindTriggers,
    confidence_limits: CapabilityConfidenceLimits,
}

impl AdmittedBackendCapabilityWitness {
    pub(crate) const fn new(
        profile: BackendTargetProfile,
        evidence_class: CapabilityEvidenceClass,
        support: BackendCapabilitySupportSet,
        media_assumptions: BackendMediaAssumptionSet,
        rebind_triggers: BackendRebindTriggers,
        confidence_limits: CapabilityConfidenceLimits,
    ) -> Self {
        Self {
            profile,
            evidence_class,
            support,
            media_assumptions,
            rebind_triggers,
            confidence_limits,
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

    pub fn require(
        &self,
        kind: BackendCapabilityKind,
        required_evidence: CapabilityEvidenceClass,
    ) -> Result<BackendCapabilityClaimWitness, BackendCapabilityAdmissionDenial> {
        match self.require_checked(kind, required_evidence).into_raw() {
            forge_proof::TransitionOutcome::Success(claim) => Ok(claim),
            forge_proof::TransitionOutcome::Denied(denial) => Err(denial),
            forge_proof::TransitionOutcome::Stale(stale) => {
                Err(BackendCapabilityAdmissionDenial::StaleCapability {
                    kind: stale.kind,
                    posture: BackendCapabilitySupportPosture::Stale,
                })
            }
            forge_proof::TransitionOutcome::RebindRequired(rebind) => {
                Err(BackendCapabilityAdmissionDenial::RebindRequired {
                    kind: rebind.kind,
                    triggers: rebind.triggers,
                })
            }
            forge_proof::TransitionOutcome::Deferred(impossible) => match impossible {},
            forge_proof::TransitionOutcome::Failed(impossible) => match impossible {},
        }
    }

    pub fn require_checked(
        &self,
        kind: BackendCapabilityKind,
        required_evidence: CapabilityEvidenceClass,
    ) -> BackendCapabilityClaimOutcome {
        if !self.evidence_class.satisfies(required_evidence) {
            return forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::EvidenceClassTooWeak {
                    required: required_evidence,
                    actual: self.evidence_class,
                },
            )
            .into();
        }
        if !self.confidence_limits.can_back_runtime_claim() {
            return forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::ConfidenceLimitTooWeak,
            )
            .into();
        }
        if !self.media_assumptions.supports(kind) {
            return forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::MissingMediaAssumption(kind),
            )
            .into();
        }
        match self.support.posture(kind) {
            BackendCapabilitySupportPosture::Supported => {
                forge_proof::TransitionOutcome::success(BackendCapabilityClaimWitness {
                    profile: self.profile,
                    evidence_class: self.evidence_class,
                    kind,
                })
                .into()
            }
            BackendCapabilitySupportPosture::Unsupported => forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::UnsupportedCapability {
                    kind,
                    posture: BackendCapabilitySupportPosture::Unsupported,
                },
            )
            .into(),
            BackendCapabilitySupportPosture::Unavailable => forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::UnavailableCapability {
                    kind,
                    posture: BackendCapabilitySupportPosture::Unavailable,
                },
            )
            .into(),
            BackendCapabilitySupportPosture::Unknown => forge_proof::TransitionOutcome::denied(
                BackendCapabilityAdmissionDenial::UnknownCapability {
                    kind,
                    posture: BackendCapabilitySupportPosture::Unknown,
                },
            )
            .into(),
            BackendCapabilitySupportPosture::Stale => {
                forge_proof::TransitionOutcome::stale(BackendCapabilityStale { kind }).into()
            }
            BackendCapabilitySupportPosture::RebindRequired => {
                forge_proof::TransitionOutcome::rebind_required(BackendCapabilityRebindRequired {
                    kind,
                    triggers: self.rebind_triggers,
                })
                .into()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilityStale {
    kind: BackendCapabilityKind,
}

impl BackendCapabilityStale {
    pub const fn kind(self) -> BackendCapabilityKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilityRebindRequired {
    kind: BackendCapabilityKind,
    triggers: BackendRebindTriggers,
}

impl BackendCapabilityRebindRequired {
    pub const fn kind(self) -> BackendCapabilityKind {
        self.kind
    }

    pub const fn triggers(self) -> BackendRebindTriggers {
        self.triggers
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilityClaimWitness {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    kind: BackendCapabilityKind,
}

impl BackendCapabilityClaimWitness {
    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn kind(self) -> BackendCapabilityKind {
        self.kind
    }
}
