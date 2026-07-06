mod frontier_match;
mod pre_decode_bridge;
mod reference_edges;

pub(crate) use frontier_match::verify_generation_frontier_match;
pub use pre_decode_bridge::{
    classify_physical_pre_decode_damage, reject_physical_evidence_as_blob_corruption_authority,
};
pub use reference_edges::{BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges};