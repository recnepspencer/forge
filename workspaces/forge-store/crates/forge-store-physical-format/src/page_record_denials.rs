use crate::{PageRecordCounterSnapshot, PhysicalHeaderDecodeDenial, PhysicalRecordSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageRecordDenialKind {
    SlotDirectoryTooShort,
    SlotDirectoryLengthMismatch,
    PageReferenceMismatch,
    SlotOutOfRange,
    SlotEntryMismatch,
    SlotGenerationMismatch,
    DeletedSlot,
    FreeSlot,
    ReservedSlot,
    MovedSlotWithoutAdmittedReference,
    FrameOutOfBounds,
    FrameLengthMismatch,
    HeaderDecodeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRecordDenial {
    kind: PageRecordDenialKind,
    slot: Option<PhysicalRecordSlot>,
    expected_length: Option<usize>,
    actual_length: Option<usize>,
    counters: PageRecordCounterSnapshot,
    header_denial: Option<PhysicalHeaderDecodeDenial>,
}

impl PageRecordDenial {
    pub(crate) const fn new(
        kind: PageRecordDenialKind,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            slot: None,
            expected_length: None,
            actual_length: None,
            counters,
            header_denial: None,
        }
    }

    pub(crate) const fn with_slot(mut self, slot: PhysicalRecordSlot) -> Self {
        self.slot = Some(slot);
        self
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

    pub const fn kind(self) -> PageRecordDenialKind {
        self.kind
    }

    pub const fn slot(self) -> Option<PhysicalRecordSlot> {
        self.slot
    }

    pub const fn expected_length(self) -> Option<usize> {
        self.expected_length
    }

    pub const fn actual_length(self) -> Option<usize> {
        self.actual_length
    }

    pub const fn counters(self) -> PageRecordCounterSnapshot {
        self.counters
    }

    pub const fn header_denial(self) -> Option<PhysicalHeaderDecodeDenial> {
        self.header_denial
    }
}
