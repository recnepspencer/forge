use crate::{
    PhysicalFrameKind, PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderReservedField,
    PhysicalPageKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderDecodeDenialKind {
    HeaderTooShort,
    UnknownHeaderKind,
    WrongHeaderFamily,
    UnexpectedPageKind,
    UnexpectedFrameKind,
    UnsupportedVersion,
    HeaderLengthMismatch,
    PayloadLengthMismatch,
    InvalidGeneration,
    OwnerCoordinateMismatch,
    InvalidPublicationState,
    ReservedFieldMisuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalHeaderDecodeDenial {
    kind: PhysicalHeaderDecodeDenialKind,
    observed_kind_tag: Option<u8>,
    expected_page_kind: Option<PhysicalPageKind>,
    expected_frame_kind: Option<PhysicalFrameKind>,
    expected_length: Option<usize>,
    actual_length: Option<usize>,
    reserved_field: Option<PhysicalHeaderReservedField>,
    counters: PhysicalHeaderDecodeCounterSnapshot,
}

impl PhysicalHeaderDecodeDenial {
    pub(crate) const fn new(
        kind: PhysicalHeaderDecodeDenialKind,
        counters: PhysicalHeaderDecodeCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            observed_kind_tag: None,
            expected_page_kind: None,
            expected_frame_kind: None,
            expected_length: None,
            actual_length: None,
            reserved_field: None,
            counters,
        }
    }

    pub(crate) const fn with_observed_kind_tag(mut self, tag: u8) -> Self {
        self.observed_kind_tag = Some(tag);
        self
    }

    pub(crate) const fn with_expected_page_kind(mut self, kind: PhysicalPageKind) -> Self {
        self.expected_page_kind = Some(kind);
        self
    }

    pub(crate) const fn with_expected_frame_kind(mut self, kind: PhysicalFrameKind) -> Self {
        self.expected_frame_kind = Some(kind);
        self
    }

    pub(crate) const fn with_lengths(mut self, expected: usize, actual: usize) -> Self {
        self.expected_length = Some(expected);
        self.actual_length = Some(actual);
        self
    }

    pub(crate) const fn with_reserved_field(mut self, field: PhysicalHeaderReservedField) -> Self {
        self.reserved_field = Some(field);
        self
    }

    pub const fn kind(self) -> PhysicalHeaderDecodeDenialKind {
        self.kind
    }

    pub const fn counters(self) -> PhysicalHeaderDecodeCounterSnapshot {
        self.counters
    }

    pub const fn observed_kind_tag(self) -> Option<u8> {
        self.observed_kind_tag
    }

    pub const fn expected_length(self) -> Option<usize> {
        self.expected_length
    }

    pub const fn actual_length(self) -> Option<usize> {
        self.actual_length
    }

    pub const fn reserved_field(self) -> Option<PhysicalHeaderReservedField> {
        self.reserved_field
    }
}
