//! Harness helpers that must exercise production-owned capabilities through
//! public lifecycle facades.

pub use super::blob::*;
pub use super::fixtures::{
    require_native_store_aspect_fixture, AllocationSentinel, AspectDerivedSegmentReference,
    LargeRecordStreamPressure, LargeStorePressureClass, LargeStorePressureFixture,
    MemoryPressureDriverInput, NativeAspectPhysicalReferenceDenial, NativeStoreAspectFixture,
    StoreHostileReadmissionJsonFixture, StoreHostileReadmissionJsonFixtureBoundaryOutcome,
    StoreHostileReadmissionJsonFixtureBoundaryWitness, StoreJsonFixtureBoundaryDenial,
    StoreTerminalProjectionJsonFixture, StoreTerminalProjectionJsonFixtureBoundaryOutcome,
    StoreTerminalProjectionJsonFixtureBoundaryWitness,
};
pub use super::physical_isolation::*;
pub use super::physical_simulation::{
    admitted_ci_certification_driver_contracts, admitted_developer_smoke_driver_contracts,
    ci_certification_replay_seed, ci_certification_state_space_budget,
    deterministic_ci_certification_schedule, deterministic_developer_smoke_schedule,
    developer_smoke_replay_seed, developer_smoke_state_space_budget,
    production_backed_physical_fixture_materialization, unbound_production_driver,
};
pub use super::pressure::*;
pub use super::recovery::*;
pub use super::security_scope::*;
