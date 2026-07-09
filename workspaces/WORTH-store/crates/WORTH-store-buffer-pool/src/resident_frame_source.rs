use worth_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalReference, PhysicalReferenceValidationWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ResidentFrameSourceKey {
    segment_id: u64,
    page_id: u64,
    slot: u16,
    physical_generation: u64,
    header_kind_tag: u8,
    payload_length: u32,
}

impl ResidentFrameSourceKey {
    pub(crate) fn from_s1_frame_witnesses(
        reference: PhysicalReferenceValidationWitness,
        header: PhysicalHeaderDecodeWitness,
    ) -> Self {
        let physical_reference = reference.reference();
        Self {
            segment_id: required_segment_id(physical_reference),
            page_id: required_page_id(physical_reference),
            slot: required_slot(physical_reference),
            physical_generation: physical_reference.generation().get(),
            header_kind_tag: header.kind().tag(),
            payload_length: header.payload_length(),
        }
    }
}

fn required_segment_id(reference: PhysicalReference) -> u64 {
    reference
        .segment_id()
        .expect("resident frame request admits page-slot reference")
        .get()
}

fn required_page_id(reference: PhysicalReference) -> u64 {
    reference
        .page_id()
        .expect("resident frame request admits page-slot reference")
        .get()
}

fn required_slot(reference: PhysicalReference) -> u16 {
    reference
        .slot()
        .expect("resident frame request admits page-slot reference")
        .get()
}
