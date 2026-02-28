pub mod attributes;
pub mod bitset;
pub mod handles;
pub mod validators;
pub mod utils;
pub mod state;
pub(crate) mod tests;

pub mod history;
pub mod naming;
pub mod operations;
pub mod queries;

pub use handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
pub use history::lineage::Lineage;
pub use state::{MutableDraft, TopologyState};
