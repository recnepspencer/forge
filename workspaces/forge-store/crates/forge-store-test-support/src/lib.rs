#![forbid(unsafe_code)]

mod allocation_sentinels;
mod hostile_readmission_json_fixtures;
mod json_fixture_boundary;
mod large_record_streams;
mod memory_pressure;
mod native_aspect_fixture_authoring;
mod native_aspect_fixtures;
mod resident_pressure_fixtures;
mod s4_recovery_physics;
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
pub use s4_recovery_physics::{
    deterministic_s4_fresh_runtime_driver, deterministic_s4_recovery_artifacts,
    duplicate_role_s4_recovery_artifacts, incomplete_s4_recovery_artifacts,
    malformed_s4_recovery_record, reordered_s4_recovery_artifacts,
    runtime_disagreement_s4_recovery_artifacts, runtime_state_mismatch_s4_recovery_artifacts,
    ExecutedS4CrashHarnessDenial, ExecutedS4CrashHarnessTranscript, FaultSchedulerDriver,
    FreshRuntimeRecoveryDriver, RecoveryRuntimePosture, ScheduledFault, StorageBoundaryEvent,
    StorageBoundaryInterposerDriver,
};
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
