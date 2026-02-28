//! Face fragment utilities.
//!
//! DOMAIN: Inspecting face lineage to determine if a face is a fragment
//! of an initially disjoint operation (like `make_edge_face`).

use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

/// Determine if a face was created as a split fragment of a `make_edge_face` operation.
///
/// TODO(lineage-phase-3): Re-implement once MutableDraft lineage is wired.
pub fn is_make_edge_face_fragment(_arena: &TopologyArena, _face_id: FaceId) -> bool {
    // Lineage was stripped from FaceData; this needs MutableDraft-based lookup.
    false
}
