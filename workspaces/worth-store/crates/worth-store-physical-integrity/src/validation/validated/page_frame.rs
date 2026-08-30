use worth_store_physical_format::{
    InlinePageGeometry, PageGenerationCell, PhysicalRecordFormatDeclaration,
};

use super::super::{
    PhysicalArtifactScope, PhysicalIntegrityValidationDigest, PhysicalIntegrityValidationMechanism,
    PhysicalIntegrityValidationRecord, UntrustedPhysicalArtifact,
};

#[derive(Debug)]
pub struct IntegrityValidatedPageFrame<'media> {
    scope: PhysicalArtifactScope,
    record_format: PhysicalRecordFormatDeclaration,
    page: PageGenerationCell,
    slot_count: u16,
    free_bytes: u32,
    validation_record: PhysicalIntegrityValidationRecord,
    inspected: UntrustedPhysicalArtifact<'media>,
}

impl<'media> IntegrityValidatedPageFrame<'media> {
    pub(crate) fn new(
        scope: PhysicalArtifactScope,
        geometry: InlinePageGeometry,
        validated_range_checksum: u32,
        inspected: UntrustedPhysicalArtifact<'media>,
    ) -> Option<Self> {
        if scope.page_identity()? != geometry.page_cell()
            || inspected.byte_count() != scope.byte_range().length()
            || inspected.byte_count() != u64::from(scope.record_format().page_size().bytes())
        {
            return None;
        }
        let validation_record = PhysicalIntegrityValidationRecord::from_validated_scope(
            scope,
            PhysicalIntegrityValidationDigest::crc32c(scope.exact_page_scope_digest()),
            PhysicalIntegrityValidationDigest::crc32c(validated_range_checksum),
            PhysicalIntegrityValidationMechanism::Crc32cV1,
        )?;
        Some(Self {
            scope,
            record_format: scope.record_format(),
            page: geometry.page_cell(),
            slot_count: geometry.slot_count(),
            free_bytes: geometry.free_bytes(),
            validation_record,
            inspected,
        })
    }

    pub const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub const fn record_format(&self) -> PhysicalRecordFormatDeclaration {
        self.record_format
    }

    pub const fn page_identity(&self) -> PageGenerationCell {
        self.page
    }

    pub const fn slot_count(&self) -> u16 {
        self.slot_count
    }

    pub const fn free_bytes(&self) -> u32 {
        self.free_bytes
    }

    pub const fn into_validation_record(self) -> PhysicalIntegrityValidationRecord {
        self.validation_record
    }

    /// Matches the exact immutable slice incarnation inspected by validation.
    /// It exposes no bytes and grants no decoder authority.
    pub fn matches_input(&self, input: UntrustedPhysicalArtifact<'media>) -> bool {
        self.inspected.same_incarnation(input)
    }
}
