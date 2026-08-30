use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    decode_data_frame_page_lsn, DurableFrameKind, PageGenerationCell, PhysicalPageLsn,
    PhysicalRecordFormatDeclaration,
};
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
    pub page_lsn: PhysicalPageLsn,
    pub encoded_digest: [u8; 32],
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
        let input = self
            .source
            .input()
            .expect("an admitted page retains its exact C.4 observation");
        PageFrameProjection {
            record_format: self.validated.record_format(),
            page_identity: self.validated.page_identity(),
            slot_count: self.validated.slot_count(),
            free_bytes: self.validated.free_bytes(),
            page_lsn: decode_data_frame_page_lsn(input.bytes(), DurableFrameKind::InlinePage)
                .expect("an intact Phase 4 page frame retains a decodable page LSN"),
            encoded_digest: Sha256::digest(input.bytes()).into(),
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
