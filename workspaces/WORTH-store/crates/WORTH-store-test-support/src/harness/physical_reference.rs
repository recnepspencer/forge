//! Harness-only physical reference for certification courtroom replay.
//!
//! `HarnessPhysicalReference` is synthetic evidence â€” not production admission authority.

use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};

/// Synthetic physical reference admitted only for harness courtroom replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HarnessPhysicalReference(PhysicalReference);

impl HarnessPhysicalReference {
    /// Construct a harness-only slot reference for certification scenarios.
    pub fn for_courtroom_replay(slot_index: u16) -> Self {
        let cell = PhysicalGenerationAuthority::s1()
            .slot_cell(
                PhysicalSegmentId::from_raw(1).expect("harness segment id is non-zero"),
                PhysicalPageId::from_raw(1).expect("harness page id is non-zero"),
                PhysicalRecordSlot::from_raw(slot_index).expect("harness slot index is non-zero"),
            )
            .with_slot_generation(
                PhysicalGeneration::from_raw(1).expect("harness generation is non-zero"),
            );

        Self(
            PhysicalReferenceAuthority::s1()
                .admit_page_slot(cell)
                .reference(),
        )
    }

    /// Expose the underlying reference only inside test-support for courtroom wiring.
    pub(crate) fn as_physical_reference(self) -> PhysicalReference {
        self.0
    }
}

/// Construct a harness-only physical reference at the given slot index.
pub fn harness_physical_reference(slot_index: u16) -> HarnessPhysicalReference {
    HarnessPhysicalReference::for_courtroom_replay(slot_index)
}

#[deprecated(
    since = "0.0.0",
    note = "use harness_physical_reference â€” test support must not imply production authority"
)]
pub fn test_physical_reference(slot_index: u16) -> HarnessPhysicalReference {
    harness_physical_reference(slot_index)
}
