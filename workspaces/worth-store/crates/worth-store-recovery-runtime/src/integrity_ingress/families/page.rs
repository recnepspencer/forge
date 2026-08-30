use worth_store_physical_format::{PageGenerationCell, PhysicalRecordFormatDeclaration};
use worth_store_physical_integrity::IntegrityValidatedPageFrame;

use super::super::admission::require_observed_recovery_source;
use super::super::{
    ObservedRecoverySource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedPageFrame<'media> {
    source: ObservedRecoverySource<'media>,
    validated: IntegrityValidatedPageFrame<'media>,
}

pub(crate) struct PageFrameProjection {
    pub record_format: PhysicalRecordFormatDeclaration,
    pub page_identity: PageGenerationCell,
    pub slot_count: u16,
    pub free_bytes: u32,
}

impl<'media> IntegrityAdmittedPageFrame<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedRecoverySource<'media>,
        validated: IntegrityValidatedPageFrame<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_recovery_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project(
        &self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> PageFrameProjection {
        counters.record_owner_projection();
        PageFrameProjection {
            record_format: self.validated.record_format(),
            page_identity: self.validated.page_identity(),
            slot_count: self.validated.slot_count(),
            free_bytes: self.validated.free_bytes(),
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
        validated: IntegrityValidatedPageFrame<'media>,
    ) {
        let _ = IntegrityAdmittedPageFrame::bind(source, validated);
    }
    let _ = bind;
}
