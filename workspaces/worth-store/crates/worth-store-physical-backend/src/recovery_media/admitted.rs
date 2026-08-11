use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{BoundedRecoveryFilesystemDiscovery, RecoveryFilesystemQualificationError};

pub struct AdmittedRecoveryFilesystemMedia {
    pub(super) parts: crate::filesystem_media::recovery_qualification::AdmittedRecoveryParts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMediaHandleObservation {
    live_file_handles: u64,
    live_directory_handles: u64,
}

impl AdmittedRecoveryFilesystemMedia {
    pub(crate) const fn from_parts(
        parts: crate::filesystem_media::recovery_qualification::AdmittedRecoveryParts,
    ) -> Self {
        Self { parts }
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.parts.store_identity
    }

    pub const fn media_generation(&self) -> super::PhysicalRecoveryMediaGeneration {
        self.parts.media_generation
    }

    pub const fn backend_profile(&self) -> &super::QualifiedPhysicalBackendProfile {
        &self.parts.backend_profile
    }

    pub fn recovery_effect_count(&self) -> u64 {
        self.parts.recovery_effect_count()
    }

    pub fn handle_observation(&self) -> RecoveryMediaHandleObservation {
        RecoveryMediaHandleObservation::from_owner(&self.parts.owner)
    }

    #[cfg(feature = "recovery-runtime-owner")]
    pub fn scheduler_capability_claim(
        &self,
        kind: crate::BackendCapabilityKind,
        evidence: crate::CapabilityEvidenceClass,
    ) -> Result<crate::BackendCapabilityClaimWitness, crate::BackendCapabilityAdmissionDenial> {
        self.parts.execution_capability.require(kind, evidence)
    }

    #[cfg(feature = "recovery-runtime-owner")]
    pub fn mutation_owner_observation(&self) -> crate::MutationOwnerObservation {
        self.parts.owner.mutation_owner()
    }

    pub fn bounded_discovery(
        self,
        maximum_entries: u64,
        maximum_bytes: u64,
    ) -> Result<BoundedRecoveryFilesystemDiscovery, RecoveryFilesystemQualificationError> {
        BoundedRecoveryFilesystemDiscovery::new(self.parts, maximum_entries, maximum_bytes)
    }
}

impl RecoveryMediaHandleObservation {
    pub(crate) fn from_owner(owner: &crate::filesystem_media::FilesystemMediaOwner) -> Self {
        let counters = owner.counters();
        Self {
            live_file_handles: counters.live_file_handles(),
            live_directory_handles: counters.live_directory_handles(),
        }
    }

    pub const fn live_file_handles(self) -> u64 {
        self.live_file_handles
    }

    pub const fn live_directory_handles(self) -> u64 {
        self.live_directory_handles
    }

    pub const fn excess_over(self, baseline: Self) -> u64 {
        self.live_file_handles
            .saturating_sub(baseline.live_file_handles)
            .saturating_add(
                self.live_directory_handles
                    .saturating_sub(baseline.live_directory_handles),
            )
    }
}
