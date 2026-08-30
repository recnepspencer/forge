use worth_store_buffer_pool::{PhysicalFrameLease, PhysicalResidentFrameGeneration};
use worth_store_physical_integrity::{PhysicalArtifactScope, PhysicalIntegrityValidationRecord};

use super::denial::ResidentIntegrityAdmissionDenial;
use super::source_scope::require_exact_resident_source;
use crate::physical_runtime::LifecycleGeneration;

/// Private binding of one descriptive C6 record to its live Store incarnation.
pub(super) struct ResidentIntegrityRecordBinding<'lease> {
    lease: &'lease PhysicalFrameLease,
    lifecycle_generation: LifecycleGeneration,
    frame_generation: PhysicalResidentFrameGeneration,
    scope: PhysicalArtifactScope,
}

impl<'lease> ResidentIntegrityRecordBinding<'lease> {
    pub(super) fn bind_fresh(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        scope: PhysicalArtifactScope,
        record: PhysicalIntegrityValidationRecord,
    ) -> Result<Self, ResidentIntegrityAdmissionDenial> {
        require_exact_resident_source(lease, scope)?;
        if !record.matches_scope(scope) {
            return Err(ResidentIntegrityAdmissionDenial::SourceScopeMismatch);
        }
        lease
            .commit_integrity_validation(record)
            .map_err(ResidentIntegrityAdmissionDenial::Frame)?;
        Ok(Self::new(lease, lifecycle_generation, scope))
    }

    pub(super) fn reuse_exact(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        scope: PhysicalArtifactScope,
    ) -> Result<Option<Self>, ResidentIntegrityAdmissionDenial> {
        require_exact_resident_source(lease, scope)?;
        let Some(record) = lease.integrity_validation() else {
            return Ok(None);
        };
        Ok(record
            .matches_scope(scope)
            .then(|| Self::new(lease, lifecycle_generation, scope)))
    }

    pub(super) fn enter_owner_decoder(
        self,
        current_lifecycle: LifecycleGeneration,
    ) -> Result<&'lease PhysicalFrameLease, ResidentIntegrityAdmissionDenial> {
        if self.lifecycle_generation != current_lifecycle {
            return Err(ResidentIntegrityAdmissionDenial::LifecycleGenerationChanged);
        }
        if self.lease.resident_generation() != self.frame_generation {
            return Err(ResidentIntegrityAdmissionDenial::FrameGenerationChanged);
        }
        require_exact_resident_source(self.lease, self.scope)?;
        let record = self
            .lease
            .integrity_validation()
            .ok_or(ResidentIntegrityAdmissionDenial::RetainedRecordInvalidated)?;
        if !record.matches_scope(self.scope) {
            return Err(ResidentIntegrityAdmissionDenial::RetainedRecordChanged);
        }
        Ok(self.lease)
    }

    pub(super) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    fn new(
        lease: &'lease PhysicalFrameLease,
        lifecycle_generation: LifecycleGeneration,
        scope: PhysicalArtifactScope,
    ) -> Self {
        Self {
            lease,
            lifecycle_generation,
            frame_generation: lease.resident_generation(),
            scope,
        }
    }
}
