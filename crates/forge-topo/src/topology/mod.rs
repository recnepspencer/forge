pub mod state;
pub mod handles;
pub mod attributes;
pub mod integrity;
pub(crate) mod tests;

pub mod operations;
pub mod queries;
pub mod history;

// Re-exports for cleaner access
pub use state::{TopologyState, MutableDraft};
pub use handles::{FaceId, VertexId, HalfEdgeId, LoopId};
pub use operations::operator::EulerOperator;
pub use history::lineage::Lineage;
