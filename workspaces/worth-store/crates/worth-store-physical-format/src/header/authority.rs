use super::decode::{
    decode_common_header, decode_frame_kind, decode_page_kind, reject_exact_payload_length,
    reject_header_mismatch_for_witness, reject_short_header,
};
use super::{reject_frame_owner_coordinates, reject_page_owner_coordinates};
use crate::{
    ExtentGenerationCell, PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalByteOrder,
    PhysicalDecodedHeader, PhysicalFrameHeader, PhysicalFrameKind,
    PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderDecodeDenial,
    PhysicalHeaderDecodeDenialKind, PhysicalHeaderDecodeReport, PhysicalHeaderDecodeWitness,
    PhysicalPageHeader, PhysicalPageKind, PhysicalPayloadView, PhysicalPayloadViewAdmission,
    PhysicalReferenceValidationWitness, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalHeaderAuthority {
    scope: PhysicalHeaderAuthorityScope,
    binary: PhysicalBinaryEncodingWitness,
}

impl PhysicalHeaderAuthority {
    pub const fn for_canonical_physical_format(binary: PhysicalBinaryEncodingWitness) -> Self {
        Self {
            scope: PhysicalHeaderAuthorityScope::StorageFoundationS1,
            binary,
        }
    }

    pub const fn scope(&self) -> PhysicalHeaderAuthorityScope {
        self.scope
    }

    pub fn encode_page_header(
        &self,
        cell: PageGenerationCell,
        kind: PhysicalPageKind,
        payload_length: u32,
    ) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
        super::encode_page_header(self.byte_order(), kind, cell, payload_length)
    }

    pub fn encode_record_frame_header(
        &self,
        cell: SlotGenerationCell,
        payload_length: u32,
    ) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
        super::encode_record_frame_header(self.byte_order(), cell, payload_length)
    }

    pub fn encode_extent_frame_header(
        &self,
        cell: ExtentGenerationCell,
        payload_length: u32,
    ) -> [u8; PHYSICAL_HEADER_LENGTH as usize] {
        super::encode_extent_frame_header(self.byte_order(), cell, payload_length)
    }

    pub fn decode_page_header(
        &self,
        cell: PageGenerationCell,
        bytes: &[u8],
        expected_kind: PhysicalPageKind,
    ) -> Result<PhysicalHeaderDecodeReport, PhysicalHeaderDecodeDenial> {
        let report = self.decode_page_header_prefix(cell, bytes, expected_kind)?;
        reject_exact_payload_length(
            bytes.len(),
            report.witness().payload_length(),
            report.counters(),
        )?;
        Ok(report)
    }

    /// Decodes only the fixed-width header carried at the start of a page.
    ///
    /// This is the bounded streaming seam for callers that separately prove
    /// the complete artifact length. It never admits a payload view.
    pub fn decode_page_header_prefix(
        &self,
        cell: PageGenerationCell,
        bytes: &[u8],
        expected_kind: PhysicalPageKind,
    ) -> Result<PhysicalHeaderDecodeReport, PhysicalHeaderDecodeDenial> {
        let counters = PhysicalHeaderDecodeCounterSnapshot::for_page_header_attempt();
        reject_short_header(bytes, counters)?;
        let tag = bytes[0];
        let kind = decode_page_kind(tag, expected_kind, counters)?;
        let common = decode_common_header(self.byte_order(), bytes, counters)?;
        if common.generation != cell.generation() {
            return Err(PhysicalHeaderDecodeDenial::new(
                PhysicalHeaderDecodeDenialKind::InvalidGeneration,
                counters,
            ));
        }
        reject_page_owner_coordinates(self.byte_order(), bytes, cell, counters)?;
        let header = PhysicalPageHeader::new(
            kind,
            common.generation,
            common.publication,
            common.payload_length,
            common.reserved_fields,
        );
        Ok(PhysicalHeaderDecodeReport::new(
            PhysicalHeaderDecodeWitness::new(
                PhysicalDecodedHeader::Page(header),
                cell.owner(),
                counters,
            ),
        ))
    }

    pub fn decode_frame_header(
        &self,
        reference: PhysicalReferenceValidationWitness,
        bytes: &[u8],
        expected_kind: PhysicalFrameKind,
    ) -> Result<PhysicalHeaderDecodeReport, PhysicalHeaderDecodeDenial> {
        let report = self.decode_frame_header_prefix(reference, bytes, expected_kind)?;
        reject_exact_payload_length(
            bytes.len(),
            report.witness().payload_length(),
            report.counters(),
        )?;
        Ok(report)
    }

    /// Decodes only the fixed-width header carried at the start of a frame.
    /// Complete framing remains the caller's explicit responsibility.
    pub fn decode_frame_header_prefix(
        &self,
        reference: PhysicalReferenceValidationWitness,
        bytes: &[u8],
        expected_kind: PhysicalFrameKind,
    ) -> Result<PhysicalHeaderDecodeReport, PhysicalHeaderDecodeDenial> {
        let counters = PhysicalHeaderDecodeCounterSnapshot::for_frame_header_attempt();
        reject_short_header(bytes, counters)?;
        let tag = bytes[0];
        let kind = decode_frame_kind(tag, expected_kind, counters)?;
        let common = decode_common_header(self.byte_order(), bytes, counters)?;
        if common.generation != reference.owner().generation() {
            return Err(PhysicalHeaderDecodeDenial::new(
                PhysicalHeaderDecodeDenialKind::InvalidGeneration,
                counters,
            ));
        }
        reject_frame_owner_coordinates(self.byte_order(), bytes, reference.owner(), counters)?;
        let header = PhysicalFrameHeader::new(
            kind,
            common.generation,
            common.publication,
            common.payload_length,
            common.reserved_fields,
        );
        Ok(PhysicalHeaderDecodeReport::new(
            PhysicalHeaderDecodeWitness::new(
                PhysicalDecodedHeader::Frame(header),
                reference.owner(),
                counters,
            ),
        ))
    }

    pub fn payload_view<'a>(
        &self,
        bytes: &'a [u8],
        witness: PhysicalHeaderDecodeWitness,
    ) -> Result<PhysicalPayloadViewAdmission<'a>, PhysicalHeaderDecodeDenial> {
        reject_header_mismatch_for_witness(self.byte_order(), bytes, witness)?;
        reject_exact_payload_length(bytes.len(), witness.payload_length(), witness.counters())?;
        let start = witness.payload_offset();
        let end = start + witness.payload_length() as usize;
        Ok(PhysicalPayloadViewAdmission::new(PhysicalPayloadView::new(
            &bytes[start..end],
            witness,
        )))
    }

    pub(crate) const fn byte_order(&self) -> PhysicalByteOrder {
        self.binary.declaration().byte_order()
    }

    pub(crate) fn physical_format_version(&self) -> crate::PhysicalFormatVersion {
        self.binary.declaration().identity().version()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderAuthorityScope {
    StorageFoundationS1,
}
