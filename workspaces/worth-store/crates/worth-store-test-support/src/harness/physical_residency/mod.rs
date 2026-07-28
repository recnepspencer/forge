mod configuration;
mod record_chunk_world;
mod store_world;

pub use configuration::SUCCESSOR_SCOPE_ALLOCATION_BYTES;
pub use record_chunk_world::PhysicalResidencyRecordWorldFailure;
pub use store_world::{
    PhysicalResidencyStoreWorld, PhysicalResidencyStoreWorldConstructionFailure,
};
