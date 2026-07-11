use crate::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalByteOrder, PhysicalDecodedHeader,
    PhysicalFrameHeader, PhysicalFrameKind, PhysicalGeneration,
    PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderDecodeDenial,
    PhysicalHeaderDecodeDenialKind, PhysicalHeaderDecodeReport, PhysicalHeaderDecodeWitness,
    PhysicalHeaderKind, PhysicalHeaderReservedFields, PhysicalPageHeader, PhysicalPageKind,
    PhysicalPayloadView, PhysicalPayloadViewAdmission, PhysicalPublicationState,
    PhysicalReferenceValidationWitness, PHYSICAL_HEADER_LENGTH,
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

    pub fn decode_page_header(
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
        reject_payload_length_mismatch(bytes, common.payload_length, counters)?;
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
        reject_payload_length_mismatch(bytes, common.payload_length, counters)?;
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
        reject_payload_length_mismatch(bytes, witness.payload_length(), witness.counters())?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommonHeaderFields {
    generation: PhysicalGeneration,
    publication: PhysicalPublicationState,
    payload_length: u32,
    reserved_fields: PhysicalHeaderReservedFields,
}

fn decode_page_kind(
    tag: u8,
    expected: PhysicalPageKind,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<PhysicalPageKind, PhysicalHeaderDecodeDenial> {
    let Some(kind) = PhysicalPageKind::from_tag(tag) else {
        return Err(unknown_kind_denial(tag, counters));
    };
    if kind != expected {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::UnexpectedPageKind,
            counters,
        )
        .with_expected_page_kind(expected)
        .with_observed_kind_tag(tag));
    }
    Ok(kind)
}

fn decode_frame_kind(
    tag: u8,
    expected: PhysicalFrameKind,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<PhysicalFrameKind, PhysicalHeaderDecodeDenial> {
    let Some(kind) = PhysicalFrameKind::from_tag(tag) else {
        return Err(unknown_kind_denial(tag, counters));
    };
    if kind != expected {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::UnexpectedFrameKind,
            counters,
        )
        .with_expected_frame_kind(expected)
        .with_observed_kind_tag(tag));
    }
    Ok(kind)
}

fn decode_common_header(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<CommonHeaderFields, PhysicalHeaderDecodeDenial> {
    reject_unsupported_version(byte_order, bytes, counters)?;
    reject_header_length_mismatch(byte_order, bytes, counters)?;
    let generation = read_generation(byte_order, bytes, counters)?;
    let publication = read_publication(bytes, counters)?;
    let reserved_fields = read_reserved_fields(byte_order, bytes);
    if let Some(field) = reserved_fields.misused_field() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::ReservedFieldMisuse,
            counters.with_reserved_field_denial(),
        )
        .with_reserved_field(field));
    }
    Ok(CommonHeaderFields {
        generation,
        publication,
        payload_length: read_payload_length(byte_order, bytes),
        reserved_fields,
    })
}

fn reject_short_header(
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    if bytes.len() < PHYSICAL_HEADER_LENGTH as usize {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::HeaderTooShort,
            counters.with_length_mismatch_denial(),
        )
        .with_lengths(PHYSICAL_HEADER_LENGTH as usize, bytes.len()));
    }
    Ok(())
}

fn reject_unsupported_version(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let version = byte_order.read_u16([bytes[1], bytes[2]]);
    if version != crate::PhysicalFormatVersion::initial_format_version().value() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::UnsupportedVersion,
            counters.with_unsupported_version_denial(),
        ));
    }
    Ok(())
}

fn reject_header_length_mismatch(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let header_length = byte_order.read_u16([bytes[3], bytes[4]]);
    if header_length != PHYSICAL_HEADER_LENGTH {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::HeaderLengthMismatch,
            counters.with_length_mismatch_denial(),
        )
        .with_lengths(PHYSICAL_HEADER_LENGTH as usize, header_length as usize));
    }
    Ok(())
}

fn reject_payload_length_mismatch(
    bytes: &[u8],
    payload_length: u32,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let expected_total = PHYSICAL_HEADER_LENGTH as usize + payload_length as usize;
    if bytes.len() < expected_total {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch,
            counters.with_length_mismatch_denial(),
        )
        .with_lengths(expected_total, bytes.len()));
    }
    Ok(())
}

fn reject_header_mismatch_for_witness(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    witness: PhysicalHeaderDecodeWitness,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let counters = witness.counters();
    reject_short_header(bytes, counters)?;
    reject_kind_mismatch_for_witness(bytes[0], witness.header().kind(), counters)?;
    let common = decode_common_header(byte_order, bytes, counters)?;
    if common.generation != witness.header().generation() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::InvalidGeneration,
            counters,
        ));
    }
    if common.publication != witness.header().publication() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::InvalidPublicationState,
            counters,
        ));
    }
    if common.reserved_fields != witness.header().reserved_fields() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::ReservedFieldMisuse,
            counters.with_reserved_field_denial(),
        ));
    }
    if common.payload_length != witness.header().payload_length() {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch,
            counters.with_length_mismatch_denial(),
        )
        .with_lengths(
            witness.header().payload_length() as usize,
            common.payload_length as usize,
        ));
    }
    Ok(())
}

fn reject_kind_mismatch_for_witness(
    tag: u8,
    kind: PhysicalHeaderKind,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    match kind {
        PhysicalHeaderKind::Page(expected) => decode_page_kind(tag, expected, counters).map(|_| ()),
        PhysicalHeaderKind::Frame(expected) => {
            decode_frame_kind(tag, expected, counters).map(|_| ())
        }
    }
}

fn read_generation(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<PhysicalGeneration, PhysicalHeaderDecodeDenial> {
    PhysicalGeneration::from_raw(byte_order.read_u64([
        bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], bytes[16],
    ]))
    .map_err(|_| {
        PhysicalHeaderDecodeDenial::new(PhysicalHeaderDecodeDenialKind::InvalidGeneration, counters)
    })
}

fn read_publication(
    bytes: &[u8],
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<PhysicalPublicationState, PhysicalHeaderDecodeDenial> {
    PhysicalPublicationState::from_code(bytes[17]).ok_or_else(|| {
        PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::InvalidPublicationState,
            counters,
        )
    })
}

fn read_payload_length(byte_order: PhysicalByteOrder, bytes: &[u8]) -> u32 {
    byte_order.read_u32([bytes[5], bytes[6], bytes[7], bytes[8]])
}

fn read_reserved_fields(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
) -> PhysicalHeaderReservedFields {
    PhysicalHeaderReservedFields::new(
        byte_order.read_u32([bytes[18], bytes[19], bytes[20], bytes[21]]),
        byte_order.read_u64([
            bytes[22], bytes[23], bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29],
        ]),
    )
}

fn unknown_kind_denial(
    tag: u8,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> PhysicalHeaderDecodeDenial {
    PhysicalHeaderDecodeDenial::new(
        PhysicalHeaderDecodeDenialKind::UnknownHeaderKind,
        counters.with_unknown_kind_denial(),
    )
    .with_observed_kind_tag(tag)
}
