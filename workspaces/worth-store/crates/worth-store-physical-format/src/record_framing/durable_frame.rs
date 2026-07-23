use super::crc32c;
use crate::{PhysicalRecordFormatDeclaration, PhysicalRecordFormatDenial};

pub const FRAME_HEADER_BYTES: usize = 40;
const FRAME_MAGIC: [u8; 8] = *b"WRC5FRM\0";
const CHECKSUM_OFFSET: usize = 36;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DurableFrameKind {
    BootstrapCatalog = 1,
    RootManifest = 2,
    InlinePage = 3,
    Extent = 4,
    SegmentManifest = 5,
    ExtentManifest = 6,
    FreeSpaceManifest = 7,
    RootRoutingBlock = 8,
    SegmentMembershipBlock = 9,
    FreeSpaceMembershipBlock = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableFrameDenial {
    Truncated,
    WrongMagic,
    IllegalKind(u8),
    UnsupportedSchema(u8),
    UnsupportedFormat(PhysicalRecordFormatDenial),
    ReservedFieldNonZero,
    LengthMismatch,
    IntegrityMismatch,
}

pub(crate) struct DecodedFrame<'a> {
    pub(crate) identity: u64,
    pub(crate) payload: &'a [u8],
}

pub(crate) fn encode_frame(
    kind: DurableFrameKind,
    format: PhysicalRecordFormatDeclaration,
    identity: u64,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = vec![0; FRAME_HEADER_BYTES + payload.len()];
    bytes[..8].copy_from_slice(&FRAME_MAGIC);
    bytes[8] = kind as u8;
    bytes[9] = 1;
    bytes[10..20].copy_from_slice(&format.encode());
    bytes[20..22].copy_from_slice(&(FRAME_HEADER_BYTES as u16).to_le_bytes());
    bytes[24..28].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes[28..36].copy_from_slice(&identity.to_le_bytes());
    bytes[FRAME_HEADER_BYTES..].copy_from_slice(payload);
    let checksum = crc32c::checksum(&[&bytes[..CHECKSUM_OFFSET], &bytes[FRAME_HEADER_BYTES..]]);
    bytes[CHECKSUM_OFFSET..FRAME_HEADER_BYTES].copy_from_slice(&checksum.to_le_bytes());
    bytes
}

pub(crate) fn initialize_frame(
    kind: DurableFrameKind,
    format: PhysicalRecordFormatDeclaration,
    identity: u64,
    payload_bytes: usize,
) -> Vec<u8> {
    initialize_frame_reusing(Vec::new(), kind, format, identity, payload_bytes)
}

pub(crate) fn initialize_frame_reusing(
    mut bytes: Vec<u8>,
    kind: DurableFrameKind,
    format: PhysicalRecordFormatDeclaration,
    identity: u64,
    payload_bytes: usize,
) -> Vec<u8> {
    bytes.resize(FRAME_HEADER_BYTES + payload_bytes, 0);
    bytes.fill(0);
    write_header(&mut bytes, kind, format, identity, payload_bytes);
    refresh_checksum(&mut bytes);
    bytes
}

pub(crate) fn reseal_frame(
    bytes: &mut [u8],
    kind: DurableFrameKind,
    format: PhysicalRecordFormatDeclaration,
    identity: u64,
) {
    let payload_bytes = bytes.len() - FRAME_HEADER_BYTES;
    write_header(bytes, kind, format, identity, payload_bytes);
    refresh_checksum(bytes);
}

fn write_header(
    bytes: &mut [u8],
    kind: DurableFrameKind,
    format: PhysicalRecordFormatDeclaration,
    identity: u64,
    payload_bytes: usize,
) {
    bytes[..FRAME_HEADER_BYTES].fill(0);
    bytes[..8].copy_from_slice(&FRAME_MAGIC);
    bytes[8] = kind as u8;
    bytes[9] = 1;
    bytes[10..20].copy_from_slice(&format.encode());
    bytes[20..22].copy_from_slice(&(FRAME_HEADER_BYTES as u16).to_le_bytes());
    bytes[24..28].copy_from_slice(&(payload_bytes as u32).to_le_bytes());
    bytes[28..36].copy_from_slice(&identity.to_le_bytes());
}

fn refresh_checksum(bytes: &mut [u8]) {
    let checksum = crc32c::checksum(&[&bytes[..CHECKSUM_OFFSET], &bytes[FRAME_HEADER_BYTES..]]);
    bytes[CHECKSUM_OFFSET..FRAME_HEADER_BYTES].copy_from_slice(&checksum.to_le_bytes());
}

pub(crate) fn decode_frame(
    bytes: &[u8],
    expected_kind: DurableFrameKind,
) -> Result<(PhysicalRecordFormatDeclaration, DecodedFrame<'_>), DurableFrameDenial> {
    if bytes.len() < FRAME_HEADER_BYTES {
        return Err(DurableFrameDenial::Truncated);
    }
    if bytes[..8] != FRAME_MAGIC {
        return Err(DurableFrameDenial::WrongMagic);
    }
    if bytes[8] != expected_kind as u8 {
        return Err(DurableFrameDenial::IllegalKind(bytes[8]));
    }
    if bytes[9] != 1 {
        return Err(DurableFrameDenial::UnsupportedSchema(bytes[9]));
    }
    let format = PhysicalRecordFormatDeclaration::decode(
        bytes[10..20].try_into().expect("checked frame header"),
    )
    .map_err(DurableFrameDenial::UnsupportedFormat)?;
    if u16::from_le_bytes(bytes[20..22].try_into().expect("fixed field")) as usize
        != FRAME_HEADER_BYTES
    {
        return Err(DurableFrameDenial::LengthMismatch);
    }
    if bytes[22..24] != [0; 2] {
        return Err(DurableFrameDenial::ReservedFieldNonZero);
    }
    let payload_len = u32::from_le_bytes(bytes[24..28].try_into().expect("fixed field")) as usize;
    if bytes.len() != FRAME_HEADER_BYTES + payload_len {
        return Err(DurableFrameDenial::LengthMismatch);
    }
    let stored = u32::from_le_bytes(
        bytes[CHECKSUM_OFFSET..FRAME_HEADER_BYTES]
            .try_into()
            .unwrap(),
    );
    let actual = crc32c::checksum(&[&bytes[..CHECKSUM_OFFSET], &bytes[FRAME_HEADER_BYTES..]]);
    if stored != actual {
        return Err(DurableFrameDenial::IntegrityMismatch);
    }
    Ok((
        format,
        DecodedFrame {
            identity: u64::from_le_bytes(bytes[28..36].try_into().unwrap()),
            payload: &bytes[FRAME_HEADER_BYTES..],
        },
    ))
}
