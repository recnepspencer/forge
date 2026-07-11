//! Shared harness fixtures used by multiple permanent certification domains.

pub use crate::allocation_sentinels::AllocationSentinel;
pub use crate::hostile_readmission_json_fixtures::StoreHostileReadmissionJsonFixture;
pub use crate::json_fixture_boundary::{
    StoreHostileReadmissionJsonFixtureBoundaryOutcome,
    StoreHostileReadmissionJsonFixtureBoundaryWitness, StoreJsonFixtureBoundaryDenial,
    StoreTerminalProjectionJsonFixtureBoundaryOutcome,
    StoreTerminalProjectionJsonFixtureBoundaryWitness,
};
pub use crate::large_record_streams::LargeRecordStreamPressure;
pub use crate::memory_pressure::MemoryPressureDriverInput;
pub use crate::native_aspect_fixtures::{
    require_native_store_aspect_fixture, AspectDerivedSegmentReference,
    NativeAspectPhysicalReferenceDenial, NativeStoreAspectFixture,
};
pub use crate::resident_pressure_fixtures::{LargeStorePressureClass, LargeStorePressureFixture};
pub use crate::terminal_projection_json_fixtures::StoreTerminalProjectionJsonFixture;
