pub mod attributes;
pub mod bitset;
pub mod handles;
pub mod integrity;
pub mod state;
pub(crate) mod tests;

pub mod history;
pub mod naming;
pub mod operations;
pub mod queries;

// Re-exports for cleaner access
pub use handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
pub use history::lineage::Lineage;
pub use operations::operator::EulerOperator;
pub use state::{MutableDraft, TopologyState};
