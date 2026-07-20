mod encoding;
mod model_storage;
mod reference_index;
mod segment_occupancy;
mod state;

pub(crate) use model_storage::InMemoryPhysicalFormatModelStorage;
pub use state::InMemoryPhysicalFormatModel;
