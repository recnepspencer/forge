mod configuration;
mod observed_allocation;
mod record_chunk_world;
#[cfg(test)]
mod record_chunk_world_tests;
mod store_world;

pub use configuration::SUCCESSOR_SCOPE_ALLOCATION_BYTES;
pub use observed_allocation::{observed_store_residency, PhysicalResidencyFixtureWorkload};
pub use record_chunk_world::PhysicalResidencyRecordWorldFailure;
pub use store_world::{
    PhysicalResidencyStoreWorld, PhysicalResidencyStoreWorldConstructionFailure,
};
