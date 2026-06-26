use crate::{ExtentRecordCounterSnapshot, PhysicalHeaderDecodeDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtentRecordDenialKind {
    MissingExtentMembership,
    ExtentReferenceMismatch,
    ExtentLengthMismatch,
    MovedSlotMisuse,
    HeaderDecodeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentRecordDenial {
    kind: ExtentRecordDenialKind,
    expected_length: Option<usize>,
    actual_length: Option<usize>,
    counters: ExtentRecordCounterSnapshot,
    header_denial: Option<PhysicalHeaderDecodeDenial>,
}

impl ExtentRecordDenial {
    pub(crate) const fn new(
        kind: ExtentRecordDenialKind,
        counters: ExtentRecordCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            expected_length: None,
            actual_length: None,
            counters,
            header_denial: None,
        }
    }

    pub(crate) const fn with_lengths(mut self, expected: usize, actual: usize) -> Self {
        self.expected_length = Some(expected);
        self.actual_length = Some(actual);
        self
    }

    pub(crate) const fn with_header_denial(mut self, denial: PhysicalHeaderDecodeDenial) -> Self {
        self.header_denial = Some(denial);
        self
    }

    pub const fn kind(self) -> ExtentRecordDenialKind {
        self.kind
    }

    pub const fn expected_length(self) -> Option<usize> {
        self.expected_length
    }

    pub const fn actual_length(self) -> Option<usize> {
        self.actual_length
    }

    pub const fn counters(self) -> ExtentRecordCounterSnapshot {
        self.counters
    }

    pub const fn header_denial(self) -> Option<PhysicalHeaderDecodeDenial> {
        self.header_denial
    }
}
