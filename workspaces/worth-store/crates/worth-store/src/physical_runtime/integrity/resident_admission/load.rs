use worth_store_buffer_pool::PhysicalFrameLease;
use worth_store_physical_integrity::{
    PhysicalArtifactScope, PhysicalIntegrityRejection, PhysicalIntegrityValidationRecord,
};

use super::{
    denial::ResidentIntegrityAdmissionDenial, record_binding::ResidentIntegrityRecordBinding,
    source_scope::require_exact_resident_source,
};
use crate::physical_runtime::{LifecycleGeneration, ResidentAdmissionCounterCells};

#[derive(Clone, Copy)]
pub(in crate::physical_runtime) struct ResidentAdmissionContext<'counter> {
    lifecycle: LifecycleGeneration,
    counters: &'counter ResidentAdmissionCounterCells,
}

impl<'counter> ResidentAdmissionContext<'counter> {
    pub(in crate::physical_runtime) const fn new(
        lifecycle: LifecycleGeneration,
        counters: &'counter ResidentAdmissionCounterCells,
    ) -> Self {
        Self {
            lifecycle,
            counters,
        }
    }

    pub(super) fn exact_input<'lease>(
        self,
        lease: &'lease PhysicalFrameLease,
        scope: PhysicalArtifactScope,
    ) -> Result<
        worth_store_physical_integrity::UntrustedPhysicalArtifact<'lease>,
        ResidentIntegrityAdmissionDenial,
    > {
        require_exact_resident_source(lease, scope).map_err(|denial| self.reject(denial))
    }

    pub(super) fn reuse<'lease>(
        self,
        lease: &'lease PhysicalFrameLease,
        scope: PhysicalArtifactScope,
    ) -> Result<Option<ResidentIntegrityRecordBinding<'lease>>, ResidentIntegrityAdmissionDenial>
    {
        match ResidentIntegrityRecordBinding::reuse_exact(lease, self.lifecycle, scope) {
            Ok(Some(binding)) => {
                self.counters.observe_exact_record_reuse();
                Ok(Some(binding))
            }
            Ok(None) => Ok(None),
            Err(denial) => Err(self.reject(denial)),
        }
    }

    pub(super) fn bind_validated<'lease>(
        self,
        lease: &'lease PhysicalFrameLease,
        scope: PhysicalArtifactScope,
        record: PhysicalIntegrityValidationRecord,
    ) -> Result<ResidentIntegrityRecordBinding<'lease>, ResidentIntegrityAdmissionDenial> {
        ResidentIntegrityRecordBinding::bind_fresh(lease, self.lifecycle, scope, record)
            .map_err(|denial| self.reject(denial))
    }

    pub(super) fn observe_fresh_validation(self) {
        self.counters.observe_fresh_validation();
    }

    pub(super) fn validation_rejected<T>(
        self,
        rejection: PhysicalIntegrityRejection,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        Err(self.reject(ResidentIntegrityAdmissionDenial::Validation(rejection)))
    }

    pub(super) fn deny<T>(
        self,
        denial: ResidentIntegrityAdmissionDenial,
    ) -> Result<T, ResidentIntegrityAdmissionDenial> {
        Err(self.reject(denial))
    }

    pub(super) fn enter_owner_decoder<'lease>(
        self,
        binding: ResidentIntegrityRecordBinding<'lease>,
    ) -> Result<&'lease PhysicalFrameLease, ResidentIntegrityAdmissionDenial> {
        match binding.enter_owner_decoder(self.lifecycle) {
            Ok(lease) => {
                self.counters.observe_owner_decoder_entry();
                Ok(lease)
            }
            Err(denial) => Err(self.reject(denial)),
        }
    }

    fn reject(self, denial: ResidentIntegrityAdmissionDenial) -> ResidentIntegrityAdmissionDenial {
        self.counters.observe_rejection_before_decoder();
        denial
    }
}
