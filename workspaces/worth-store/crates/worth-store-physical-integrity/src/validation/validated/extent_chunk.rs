use worth_store_physical_format::{
    ExtentChunkCoordinate, PersistedRecordIdentity, PhysicalRecordFormatDeclaration,
    RecordExtentGenerationCell,
};

use super::super::{
    PhysicalArtifactScope, PhysicalIntegrityValidationDigest, PhysicalIntegrityValidationMechanism,
    PhysicalIntegrityValidationRecord, UntrustedPhysicalArtifact,
};

#[derive(Debug)]
pub struct IntegrityValidatedExtentChunkFrame<'media> {
    scope: PhysicalArtifactScope,
    record_format: PhysicalRecordFormatDeclaration,
    coordinate: ExtentChunkCoordinate,
    validation_record: PhysicalIntegrityValidationRecord,
    inspected: UntrustedPhysicalArtifact<'media>,
}

impl<'media> IntegrityValidatedExtentChunkFrame<'media> {
    pub(crate) fn new(
        scope: PhysicalArtifactScope,
        record_format: PhysicalRecordFormatDeclaration,
        chunk_bytes: &'media [u8],
        validated_range_checksum: u32,
        inspected: UntrustedPhysicalArtifact<'media>,
    ) -> Option<Self> {
        let coordinate = scope.extent_chunk_coordinate()?;
        if !scope.is_extent_chunk()
            || record_format != scope.record_format()
            || inspected.byte_count() != scope.byte_range().length()
        {
            return None;
        }
        let inspected_tail = inspected
            .bytes()
            .get(inspected.bytes().len().checked_sub(chunk_bytes.len())?..)?;
        if inspected_tail.len() != chunk_bytes.len()
            || !core::ptr::eq(inspected_tail.as_ptr(), chunk_bytes.as_ptr())
        {
            return None;
        }
        let validation_record = PhysicalIntegrityValidationRecord::from_validated_scope(
            scope,
            PhysicalIntegrityValidationDigest::crc32c(scope.exact_extent_scope_digest()),
            PhysicalIntegrityValidationDigest::crc32c(validated_range_checksum),
            PhysicalIntegrityValidationMechanism::Crc32cV1,
        )?;
        Some(Self {
            scope,
            record_format,
            coordinate,
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

    pub const fn coordinate(&self) -> ExtentChunkCoordinate {
        self.coordinate
    }

    pub const fn record(&self) -> PersistedRecordIdentity {
        self.coordinate.record()
    }

    pub const fn extent_cell(&self) -> RecordExtentGenerationCell {
        self.coordinate.extent_cell()
    }

    pub const fn logical_bytes(&self) -> u64 {
        self.coordinate.logical_bytes()
    }

    pub const fn logical_offset(&self) -> u64 {
        self.coordinate.logical_offset()
    }

    pub const fn ordinal(&self) -> u32 {
        self.coordinate.ordinal()
    }

    pub const fn into_validation_record(self) -> PhysicalIntegrityValidationRecord {
        self.validation_record
    }

    pub fn matches_input(&self, input: UntrustedPhysicalArtifact<'media>) -> bool {
        self.inspected.same_incarnation(input)
    }
}
