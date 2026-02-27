//! Face fragment utilities.
//!
//! DOMAIN: Inspecting face lineage to determine if a face is a fragment
//! of an initially disjoint operation (like `make_edge_face`).

use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

/// Determine if a face was created as a split fragment of a `make_edge_face` operation.
pub fn is_make_edge_face_fragment(arena: &TopologyArena, face_id: FaceId) -> bool {
    arena
        .get_face(face_id)
        .ok()
        .and_then(|f| f.lineage())
        .map(|lin| {
            lin.get_creation_op()
                .get_name()
                .starts_with("make_edge_face")
        })
        .unwrap_or(false)
}
