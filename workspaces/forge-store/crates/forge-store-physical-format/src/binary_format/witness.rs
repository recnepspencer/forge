use super::declaration::{find_alignment, find_width};
use crate::{
    PhysicalAlignmentClass, PhysicalAlignmentSite, PhysicalBinaryFormatError, PhysicalByteOrder,
    PhysicalFieldWidth, PhysicalFieldWidthKind, PhysicalFormatDeclaration, PhysicalFormatIdentity,
    PhysicalFormatMagic, PhysicalFormatVersion, PhysicalForwardCompatibilityPolicy,
    PhysicalPageSizeClass, PhysicalReservedFieldPolicy,
};

const GOLDEN_HEADER_LEN: usize = crate::binary_format::golden_bytes::GOLDEN_HEADER_LEN;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBinaryEncodingWitness {
    declaration: PhysicalFormatDeclaration,
}

impl PhysicalBinaryEncodingWitness {
    pub fn admit(
        declaration: PhysicalFormatDeclaration,
    ) -> Result<Self, PhysicalBinaryFormatError> {
        Ok(Self { declaration })
    }

    pub fn physical_format_canonical() -> Result<Self, PhysicalBinaryFormatError> {
        Self::admit(PhysicalFormatDeclaration::physical_format_canonical()?)
    }

    pub const fn declaration(&self) -> &PhysicalFormatDeclaration {
        &self.declaration
    }

    pub fn format_identity(&self) -> PhysicalFormatIdentity {
        self.declaration.identity()
    }

    pub fn encode_golden_format_header(&self) -> Vec<u8> {
        let byte_order = self.declaration.byte_order();
        let mut bytes = Vec::with_capacity(GOLDEN_HEADER_LEN);
        bytes.extend_from_slice(&self.declaration.identity().magic().bytes());
        bytes.extend_from_slice(
            &byte_order.write_u16(self.declaration.identity().version().value()),
        );
        bytes.push(byte_order.code());
        bytes.extend_from_slice(&byte_order.write_u32(self.declaration.page_size().bytes()));
        encode_field_widths(&self.declaration, byte_order, &mut bytes);
        encode_alignments(&self.declaration, byte_order, &mut bytes);
        bytes.push(self.declaration.reserved_field_policy().code());
        bytes.push(self.declaration.forward_compatibility().code());
        bytes
    }

    pub fn decode_golden_format_header(bytes: &[u8]) -> Result<Self, PhysicalBinaryFormatError> {
        if bytes.len() != GOLDEN_HEADER_LEN {
            return Err(PhysicalBinaryFormatError::GoldenHeaderLengthMismatch {
                expected: GOLDEN_HEADER_LEN,
                actual: bytes.len(),
            });
        }
        let byte_order = PhysicalByteOrder::LittleEndian;
        let declaration = PhysicalFormatDeclaration::builder()
            .magic(read_magic(bytes)?)
            .version(read_version(byte_order, bytes)?)
            .byte_order(read_byte_order(bytes)?)
            .page_size(read_page_size(byte_order, bytes)?)
            .field_width(read_field_width(
                byte_order,
                bytes,
                15,
                PhysicalFieldWidthKind::SegmentId,
            )?)
            .field_width(read_field_width(
                byte_order,
                bytes,
                17,
                PhysicalFieldWidthKind::PageId,
            )?)
            .field_width(read_field_width(
                byte_order,
                bytes,
                19,
                PhysicalFieldWidthKind::Generation,
            )?)
            .field_width(read_field_width(
                byte_order,
                bytes,
                21,
                PhysicalFieldWidthKind::HeaderLength,
            )?)
            .field_width(read_field_width(
                byte_order,
                bytes,
                23,
                PhysicalFieldWidthKind::PayloadLength,
            )?)
            .alignment(read_alignment(
                byte_order,
                bytes,
                25,
                PhysicalAlignmentSite::PageStart,
            )?)
            .alignment(read_alignment(
                byte_order,
                bytes,
                27,
                PhysicalAlignmentSite::FrameStart,
            )?)
            .alignment(read_alignment(
                byte_order,
                bytes,
                29,
                PhysicalAlignmentSite::SlotDirectoryOffset,
            )?)
            .alignment(read_alignment(
                byte_order,
                bytes,
                31,
                PhysicalAlignmentSite::ExtentStart,
            )?)
            .alignment(read_alignment(
                byte_order,
                bytes,
                33,
                PhysicalAlignmentSite::ManifestRecord,
            )?)
            .reserved_field_policy(read_reserved_policy(bytes)?)
            .forward_compatibility(read_forward_policy(bytes)?)
            .define()?;
        Self::admit(declaration)
    }
}

fn encode_field_widths(
    declaration: &PhysicalFormatDeclaration,
    byte_order: PhysicalByteOrder,
    bytes: &mut Vec<u8>,
) {
    for kind in PhysicalFieldWidthKind::required_for_physical_format() {
        let width = find_width(kind, declaration.field_widths())
            .expect("admitted format declaration has every required field width");
        bytes.extend_from_slice(&byte_order.write_u16(width.bits()));
    }
}

fn encode_alignments(
    declaration: &PhysicalFormatDeclaration,
    byte_order: PhysicalByteOrder,
    bytes: &mut Vec<u8>,
) {
    for site in PhysicalAlignmentSite::required_for_physical_format() {
        let alignment = find_alignment(site, declaration.alignments())
            .expect("admitted format declaration has every required alignment");
        bytes.extend_from_slice(&byte_order.write_u16(alignment.bytes()));
    }
}

fn read_magic(bytes: &[u8]) -> Result<PhysicalFormatMagic, PhysicalBinaryFormatError> {
    if bytes[0..8] == PhysicalFormatMagic::store_format_magic().bytes() {
        Ok(PhysicalFormatMagic::store_format_magic())
    } else {
        Err(PhysicalBinaryFormatError::MagicMismatch)
    }
}

fn read_version(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
) -> Result<PhysicalFormatVersion, PhysicalBinaryFormatError> {
    let version = byte_order.read_u16([bytes[8], bytes[9]]);
    if version == PhysicalFormatVersion::initial_format_version().value() {
        Ok(PhysicalFormatVersion::initial_format_version())
    } else {
        Err(PhysicalBinaryFormatError::VersionMismatch)
    }
}

fn read_byte_order(bytes: &[u8]) -> Result<PhysicalByteOrder, PhysicalBinaryFormatError> {
    if bytes[10] == PhysicalByteOrder::LittleEndian.code() {
        Ok(PhysicalByteOrder::LittleEndian)
    } else {
        Err(PhysicalBinaryFormatError::ByteOrderMismatch)
    }
}

fn read_page_size(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
) -> Result<PhysicalPageSizeClass, PhysicalBinaryFormatError> {
    PhysicalPageSizeClass::from_bytes(
        byte_order.read_u32([bytes[11], bytes[12], bytes[13], bytes[14]]),
    )
}

fn read_field_width(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    kind: PhysicalFieldWidthKind,
) -> Result<PhysicalFieldWidth, PhysicalBinaryFormatError> {
    PhysicalFieldWidth::from_bits(
        kind,
        byte_order.read_u16([bytes[offset], bytes[offset + 1]]),
    )
}

fn read_alignment(
    byte_order: PhysicalByteOrder,
    bytes: &[u8],
    offset: usize,
    site: PhysicalAlignmentSite,
) -> Result<PhysicalAlignmentClass, PhysicalBinaryFormatError> {
    PhysicalAlignmentClass::from_bytes(
        site,
        byte_order.read_u16([bytes[offset], bytes[offset + 1]]),
    )
}

fn read_reserved_policy(
    bytes: &[u8],
) -> Result<PhysicalReservedFieldPolicy, PhysicalBinaryFormatError> {
    if bytes[35] == PhysicalReservedFieldPolicy::zeroed_and_preserved().code() {
        Ok(PhysicalReservedFieldPolicy::zeroed_and_preserved())
    } else {
        Err(PhysicalBinaryFormatError::UnknownReservedFieldPolicy)
    }
}

fn read_forward_policy(
    bytes: &[u8],
) -> Result<PhysicalForwardCompatibilityPolicy, PhysicalBinaryFormatError> {
    match bytes[36] {
        1 => Ok(PhysicalForwardCompatibilityPolicy::RejectUnknownKind),
        2 => Ok(PhysicalForwardCompatibilityPolicy::PreserveUnknownBytes),
        3 => Ok(PhysicalForwardCompatibilityPolicy::MigrationReserved),
        _ => Err(PhysicalBinaryFormatError::UnsupportedForwardCompatibilityPolicy),
    }
}
