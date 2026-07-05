use crate::{
    AdmittedBackendCapabilityWitness, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendTargetProfile, CapabilityEvidenceClass,
};

use super::{
    StoreDurabilityCounterSnapshot, StoreDurabilityCounterStrength, StoreDurabilityDenial,
    StoreDurabilityDenialKind, StoreDurabilityOperation, StoreDurabilityRequirement,
    StoreDurabilityState, StoreDurabilityWriteSubmitted,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StoreDurabilityAdmissionOutcome {
    Admitted(StoreDurabilityAdmission),
    Denied(StoreDurabilityDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreDurabilityAdmission {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    requirement: StoreDurabilityRequirement,
    counters: StoreDurabilityCounterSnapshot,
}

impl StoreDurabilityAdmission {
    pub fn admit(
        requirement: StoreDurabilityRequirement,
        witness: &AdmittedBackendCapabilityWitness,
    ) -> Result<Self, StoreDurabilityDenial> {
        match Self::admit_checked(requirement, witness) {
            StoreDurabilityAdmissionOutcome::Admitted(admission) => Ok(admission),
            StoreDurabilityAdmissionOutcome::Denied(denial) => Err(denial),
        }
    }

    pub fn admit_checked(
        requirement: StoreDurabilityRequirement,
        witness: &AdmittedBackendCapabilityWitness,
    ) -> StoreDurabilityAdmissionOutcome {
        let counters = StoreDurabilityCounterSnapshot::new(StoreDurabilityCounterStrength::Exact);
        let required_evidence = CapabilityEvidenceClass::CertifiedBackendProfile;
        if witness.evidence_class() == CapabilityEvidenceClass::ExternallyGuaranteed {
            return StoreDurabilityAdmissionOutcome::Denied(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::ExternallyGuaranteedCannotSatisfyCertifiedApi,
                StoreDurabilityState::Denied,
                operation_for(requirement),
                witness.profile(),
                required_evidence,
                witness.evidence_class(),
                counters.with_denied_claim(),
            ));
        }
        if !witness.evidence_class().satisfies(required_evidence) {
            return StoreDurabilityAdmissionOutcome::Denied(StoreDurabilityDenial::new(
                StoreDurabilityDenialKind::EvidenceClassTooWeak,
                StoreDurabilityState::Denied,
                operation_for(requirement),
                witness.profile(),
                required_evidence,
                witness.evidence_class(),
                counters.with_denied_claim(),
            ));
        }
        match denied_capability(requirement, witness, counters) {
            Some(denial) => StoreDurabilityAdmissionOutcome::Denied(denial),
            None => StoreDurabilityAdmissionOutcome::Admitted(Self {
                profile: witness.profile(),
                evidence_class: witness.evidence_class(),
                requirement,
                counters,
            }),
        }
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn requirement(self) -> StoreDurabilityRequirement {
        self.requirement
    }

    pub const fn counters(self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub fn submit_write<S>(self, scope: S) -> StoreDurabilityWriteSubmitted<S> {
        StoreDurabilityWriteSubmitted::new(
            scope,
            self.profile,
            self.evidence_class,
            self.requirement,
            crate::WalDurabilityBarrierSet::EMPTY,
            self.counters.with_write_submitted(),
        )
    }
}

fn denied_capability(
    requirement: StoreDurabilityRequirement,
    witness: &AdmittedBackendCapabilityWitness,
    counters: StoreDurabilityCounterSnapshot,
) -> Option<StoreDurabilityDenial> {
    let required = [
        Some(BackendCapabilityKind::Fsync)
            .filter(|_| requirement.requires_fsync() || requirement.requires_fdatasync()),
        Some(BackendCapabilityKind::DirectorySync)
            .filter(|_| requirement.requires_directory_sync()),
        Some(BackendCapabilityKind::DurableRename)
            .filter(|_| requirement.requires_rename_durable()),
    ];
    for capability in required.into_iter().flatten() {
        match witness.support().posture(capability) {
            BackendCapabilitySupportPosture::Supported => {
                if !witness.media_assumptions().supports(capability) {
                    return Some(base_capability_denial(
                        StoreDurabilityDenialKind::MissingMediaAssumption,
                        StoreDurabilityState::Denied,
                        requirement,
                        witness,
                        counters.with_denied_claim(),
                        capability,
                        BackendCapabilitySupportPosture::Supported,
                    ));
                }
            }
            BackendCapabilitySupportPosture::Unsupported
            | BackendCapabilitySupportPosture::Unavailable => {
                return Some(base_capability_denial(
                    StoreDurabilityDenialKind::UnsupportedDurabilityCapability,
                    StoreDurabilityState::DurabilityUnsupported,
                    requirement,
                    witness,
                    counters.with_unsupported_claim(),
                    capability,
                    witness.support().posture(capability),
                ));
            }
            BackendCapabilitySupportPosture::Unknown => {
                return Some(base_capability_denial(
                    StoreDurabilityDenialKind::UnknownDurabilityPosture,
                    StoreDurabilityState::DurabilityUnknown,
                    requirement,
                    witness,
                    counters.with_unknown_claim(),
                    capability,
                    BackendCapabilitySupportPosture::Unknown,
                ));
            }
            BackendCapabilitySupportPosture::Stale => {
                return Some(base_capability_denial(
                    StoreDurabilityDenialKind::StaleDurabilityPosture,
                    StoreDurabilityState::Stale,
                    requirement,
                    witness,
                    counters.with_stale_claim(),
                    capability,
                    BackendCapabilitySupportPosture::Stale,
                ));
            }
            BackendCapabilitySupportPosture::RebindRequired => {
                return Some(
                    base_capability_denial(
                        StoreDurabilityDenialKind::RebindRequired,
                        StoreDurabilityState::RebindRequired,
                        requirement,
                        witness,
                        counters.with_rebind_required_claim(),
                        capability,
                        BackendCapabilitySupportPosture::RebindRequired,
                    )
                    .with_rebind_triggers(witness.rebind_triggers()),
                );
            }
        }
    }
    None
}

fn base_capability_denial(
    kind: StoreDurabilityDenialKind,
    state: StoreDurabilityState,
    requirement: StoreDurabilityRequirement,
    witness: &AdmittedBackendCapabilityWitness,
    counters: StoreDurabilityCounterSnapshot,
    capability: BackendCapabilityKind,
    posture: BackendCapabilitySupportPosture,
) -> StoreDurabilityDenial {
    StoreDurabilityDenial::new(
        kind,
        state,
        operation_for(requirement),
        witness.profile(),
        CapabilityEvidenceClass::CertifiedBackendProfile,
        witness.evidence_class(),
        counters,
    )
    .with_capability(capability, posture)
}

const fn operation_for(requirement: StoreDurabilityRequirement) -> StoreDurabilityOperation {
    match requirement.publication() {
        super::StoreDurabilityPublicationKind::WalFrame => StoreDurabilityOperation::WalPublication,
        super::StoreDurabilityPublicationKind::Checkpoint => {
            StoreDurabilityOperation::CheckpointPublication
        }
        super::StoreDurabilityPublicationKind::Manifest => {
            StoreDurabilityOperation::ManifestPublication
        }
    }
}
