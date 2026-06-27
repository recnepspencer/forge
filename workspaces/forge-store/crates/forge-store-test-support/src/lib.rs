#![forbid(unsafe_code)]

mod allocation_sentinels;
mod hostile_readmission_json_fixtures;
mod json_fixture_boundary;
mod large_record_streams;
mod memory_pressure;
mod native_aspect_fixture_authoring;
mod native_aspect_fixtures;
mod resident_pressure_fixtures;
mod terminal_projection_json_fixtures;

use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};

pub use allocation_sentinels::AllocationSentinel;
pub use hostile_readmission_json_fixtures::StoreHostileReadmissionJsonFixture;
pub use json_fixture_boundary::{
    StoreHostileReadmissionJsonFixtureBoundaryOutcome,
    StoreHostileReadmissionJsonFixtureBoundaryWitness, StoreJsonFixtureBoundaryDenial,
    StoreTerminalProjectionJsonFixtureBoundaryOutcome,
    StoreTerminalProjectionJsonFixtureBoundaryWitness,
};
pub use large_record_streams::LargeRecordStreamPressure;
pub use memory_pressure::MemoryPressureDriverInput;
pub use native_aspect_fixtures::{require_native_store_aspect_fixture, NativeStoreAspectFixture};
pub use resident_pressure_fixtures::{LargeStorePressureClass, LargeStorePressureFixture};
pub use terminal_projection_json_fixtures::StoreTerminalProjectionJsonFixture;

pub fn test_physical_reference(slot_index: u16) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).expect("test segment id is non-zero"),
            PhysicalPageId::from_raw(1).expect("test page id is non-zero"),
            PhysicalRecordSlot::from_raw(slot_index).expect("test slot index is non-zero"),
        )
        .with_slot_generation(
            PhysicalGeneration::from_raw(1).expect("test generation is non-zero"),
        );

    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}
