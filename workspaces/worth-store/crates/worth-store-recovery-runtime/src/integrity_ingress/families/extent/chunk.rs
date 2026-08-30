use worth_store_physical_format::{
    ExtentChunkCoordinate, PersistedRecordIdentity, PhysicalRecordFormatDeclaration,
    RecordExtentGenerationCell,
};
use worth_store_physical_integrity::IntegrityValidatedExtentChunkFrame;

use super::super::super::admission::require_observed_recovery_source;
use super::super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedExtentChunkFrame<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedExtentChunkFrame<'media>,
}

pub(crate) struct ExtentChunkProjection {
    pub coordinate: ExtentChunkCoordinate,
    pub record: PersistedRecordIdentity,
    pub extent_cell: RecordExtentGenerationCell,
    pub record_format: PhysicalRecordFormatDeclaration,
    pub logical_bytes: u64,
    pub logical_offset: u64,
    pub ordinal: u32,
}

impl<'media> IntegrityAdmittedExtentChunkFrame<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedExtentChunkFrame<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> ExtentChunkProjection {
        counters.record_owner_projection();
        ExtentChunkProjection {
            coordinate: self.validated.coordinate(),
            record: self.validated.record(),
            extent_cell: self.validated.extent_cell(),
            record_format: self.validated.record_format(),
            logical_bytes: self.validated.logical_bytes(),
            logical_offset: self.validated.logical_offset(),
            ordinal: self.validated.ordinal(),
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
        validated: IntegrityValidatedExtentChunkFrame<'media>,
    ) {
        let _ = IntegrityAdmittedExtentChunkFrame::bind(source, validated);
    }
    let _ = bind;
}
