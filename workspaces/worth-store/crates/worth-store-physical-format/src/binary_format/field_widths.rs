use crate::PhysicalBinaryFormatError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFieldWidthKind {
    SegmentId,
    PageId,
    Generation,
    HeaderLength,
    PayloadLength,
}

impl PhysicalFieldWidthKind {
    pub const fn required_for_physical_format() -> [Self; 5] {
        [
            Self::SegmentId,
            Self::PageId,
            Self::Generation,
            Self::HeaderLength,
            Self::PayloadLength,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFieldWidth {
    kind: PhysicalFieldWidthKind,
    bits: u16,
}

impl PhysicalFieldWidth {
    pub const fn segment_id_u64() -> Self {
        Self::new(PhysicalFieldWidthKind::SegmentId, 64)
    }

    pub const fn page_id_u64() -> Self {
        Self::new(PhysicalFieldWidthKind::PageId, 64)
    }

    pub const fn generation_u64() -> Self {
        Self::new(PhysicalFieldWidthKind::Generation, 64)
    }

    pub const fn header_length_u16() -> Self {
        Self::new(PhysicalFieldWidthKind::HeaderLength, 16)
    }

    pub const fn payload_length_u32() -> Self {
        Self::new(PhysicalFieldWidthKind::PayloadLength, 32)
    }

    pub(crate) const fn new(kind: PhysicalFieldWidthKind, bits: u16) -> Self {
        Self { kind, bits }
    }

    pub(crate) fn from_bits(
        kind: PhysicalFieldWidthKind,
        bits: u16,
    ) -> Result<Self, PhysicalBinaryFormatError> {
        let width = Self::new(kind, bits);
        if width == expected_width_for_kind(kind) {
            Ok(width)
        } else {
            Err(PhysicalBinaryFormatError::FieldWidthMismatch(kind))
        }
    }

    pub const fn kind(&self) -> PhysicalFieldWidthKind {
        self.kind
    }

    pub const fn bits(&self) -> u16 {
        self.bits
    }
}

pub(crate) const fn expected_width_for_kind(kind: PhysicalFieldWidthKind) -> PhysicalFieldWidth {
    match kind {
        PhysicalFieldWidthKind::SegmentId => PhysicalFieldWidth::segment_id_u64(),
        PhysicalFieldWidthKind::PageId => PhysicalFieldWidth::page_id_u64(),
        PhysicalFieldWidthKind::Generation => PhysicalFieldWidth::generation_u64(),
        PhysicalFieldWidthKind::HeaderLength => PhysicalFieldWidth::header_length_u16(),
        PhysicalFieldWidthKind::PayloadLength => PhysicalFieldWidth::payload_length_u32(),
    }
}
