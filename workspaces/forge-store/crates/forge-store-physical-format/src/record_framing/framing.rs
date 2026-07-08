use crate::{
    PageRecordCounterSnapshot, PhysicalFrameKind, PhysicalGenerationOwner, PhysicalPayloadView,
    PhysicalReference, PhysicalReferenceAdmissionWitness, PhysicalReferenceValidationWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPlacementClass {
    PageLocalSlot,
    ExtentBackedReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramedRecordPayload<'a> {
    bytes: &'a [u8],
}

impl<'a> FramedRecordPayload<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramedRecordView<'a> {
    frame_kind: PhysicalFrameKind,
    payload: FramedRecordPayload<'a>,
    placement: RecordPlacementWitness,
}

impl<'a> FramedRecordView<'a> {
    pub(crate) const fn new(
        frame_kind: PhysicalFrameKind,
        payload: FramedRecordPayload<'a>,
        placement: RecordPlacementWitness,
    ) -> Self {
        Self {
            frame_kind,
            payload,
            placement,
        }
    }

    pub const fn frame_kind(self) -> PhysicalFrameKind {
        self.frame_kind
    }

    pub const fn payload(self) -> FramedRecordPayload<'a> {
        self.payload
    }

    pub const fn placement(self) -> RecordPlacementWitness {
        self.placement
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPlacementWitness {
    reference: PhysicalReference,
    placement_class: RecordPlacementClass,
    counters: PageRecordCounterSnapshot,
}

impl RecordPlacementWitness {
    pub(crate) const fn new(
        reference: PhysicalReference,
        placement_class: RecordPlacementClass,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            placement_class,
            counters,
        }
    }

    pub(crate) const fn from_admission(
        admission: PhysicalReferenceAdmissionWitness,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self::new(
            admission.reference(),
            RecordPlacementClass::PageLocalSlot,
            counters,
        )
    }

    pub(crate) const fn from_validation(
        validation: PhysicalReferenceValidationWitness,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self::new(
            validation.reference(),
            RecordPlacementClass::PageLocalSlot,
            counters,
        )
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn placement_class(self) -> RecordPlacementClass {
        self.placement_class
    }

    pub const fn counters(self) -> PageRecordCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPagePayload<'a> {
    payload: PhysicalPayloadView<'a>,
}

impl<'a> RecordPagePayload<'a> {
    pub(crate) const fn new(payload: PhysicalPayloadView<'a>) -> Self {
        Self { payload }
    }

    pub const fn bytes(self) -> &'a [u8] {
        self.payload.as_bytes()
    }

    pub const fn page_owner(self) -> PhysicalGenerationOwner {
        self.payload.witness().owner()
    }

    pub const fn header_payload(self) -> PhysicalPayloadView<'a> {
        self.payload
    }
}
