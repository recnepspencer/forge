mod configuration;
mod store_world;

pub use configuration::SUCCESSOR_SCOPE_ALLOCATION_BYTES;
pub use store_world::{
    PhysicalResidencyStoreWorld, PhysicalResidencyStoreWorldConstructionFailure,
};
