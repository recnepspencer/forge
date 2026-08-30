use worth_store_physical_format::{
    PersistedRecordIdentity, PhysicalRecordFormatDeclaration, RecordExtentGenerationCell,
};
use worth_store_physical_integrity::IntegrityValidatedExtentManifest;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedExtentManifest<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedExtentManifest<'media>,
}

pub(crate) struct ExtentManifestProjection {
    pub record: PersistedRecordIdentity,
    pub extent_cell: RecordExtentGenerationCell,
    pub record_format: PhysicalRecordFormatDeclaration,
    pub logical_bytes: u64,
    pub maximum_frame_bytes: u32,
    pub chunk_payload_capacity: u32,
    pub chunk_count: u32,
}

impl<'media> IntegrityAdmittedExtentManifest<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedExtentManifest<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> ExtentManifestProjection {
        counters.record_owner_projection();
        ExtentManifestProjection {
            record: self.validated.record(),
            extent_cell: self.validated.extent_cell(),
            record_format: self.validated.record_format(),
            logical_bytes: self.validated.logical_bytes(),
            maximum_frame_bytes: self.validated.maximum_frame_bytes(),
            chunk_payload_capacity: self.validated.chunk_payload_capacity(),
            chunk_count: self.validated.chunk_count(),
        }
    }

    pub(crate) fn scope(&self) -> worth_store_physical_integrity::PhysicalArtifactScope {
        self.source.scope()
    }
}

#[cfg(test)]
pub(super) fn owner_valid_compile_contract() {
    fn bind<'media>(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedExtentManifest<'media>,
    ) {
        let _ = IntegrityAdmittedExtentManifest::bind(source, validated);
    }
    let _ = bind;
}
