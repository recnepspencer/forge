mod encoding;
mod reference_index;
mod runtime_storage;
mod segment_occupancy;
mod state;

pub(crate) use runtime_storage::PhysicalStoreRuntimeStorage;
pub use state::PhysicalStoreRuntime;
