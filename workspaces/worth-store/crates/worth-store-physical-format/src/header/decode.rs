use super::layout::{RESERVED_CHECKSUM_OFFSET, RESERVED_RECOVERY_LSN_OFFSET};
use super::reject_generation_owner_coordinates;
use crate::{
    PhysicalByteOrder, PhysicalFrameKind, PhysicalGeneration, PhysicalHeaderDecodeCounterSnapshot,
    PhysicalHeaderDecodeDenial, PhysicalHeaderDecodeDenialKind, PhysicalHeaderDecodeWitness,
    PhysicalHeaderKind, PhysicalHeaderReservedFields, PhysicalPageKind, PhysicalPublicationState,
    PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CommonHeaderFields {
    pub(super) generation: PhysicalGeneration,
    pub(super) publication: PhysicalPublicationState,
    pub(super) payload_length: u32,
    pub(super) reserved_fields: PhysicalHeaderReservedFields,
}

pub(super) fn decode_page_kind(
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

pub(super) fn decode_frame_kind(
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

pub(super) fn decode_common_header(
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

pub(super) fn reject_short_header(
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

pub(super) fn reject_exact_payload_length(
    actual_total: usize,
    payload_length: u32,
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    let expected_total = PHYSICAL_HEADER_LENGTH as usize + payload_length as usize;
    if actual_total != expected_total {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch,
            counters.with_length_mismatch_denial(),
        )
        .with_lengths(expected_total, actual_total));
    }
    Ok(())
}

pub(super) fn reject_header_mismatch_for_witness(
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
    reject_generation_owner_coordinates(byte_order, bytes, witness.owner(), counters)?;
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
        byte_order.read_u32([
            bytes[RESERVED_CHECKSUM_OFFSET],
            bytes[RESERVED_CHECKSUM_OFFSET + 1],
            bytes[RESERVED_CHECKSUM_OFFSET + 2],
            bytes[RESERVED_CHECKSUM_OFFSET + 3],
        ]),
        byte_order.read_u64([
            bytes[RESERVED_RECOVERY_LSN_OFFSET],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 1],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 2],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 3],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 4],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 5],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 6],
            bytes[RESERVED_RECOVERY_LSN_OFFSET + 7],
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
