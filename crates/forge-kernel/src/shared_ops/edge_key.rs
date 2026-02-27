//! Canonical undirected edge key utilities.
//!
//! DOMAIN: Produce a stable `(u32, u32)` key from two VertexIds for use
//! in maps and sets that need edge identity independent of direction.
//!
//! CONSUMERS: Boolean split (EdgeCutMap provenance), forge-topo polygon
//! adjacent-pair guards, any operation that indexes edges by their
//! vertex pair.

use forge_topo::handles::VertexId;

/// Create a canonical (sorted) edge key from two vertex IDs.
///
/// The pair is always stored as `(min_index, max_index)` so the same
/// undirected edge produces the same key regardless of which vertex is
/// passed first.
pub fn make_edge_key(v1: VertexId, v2: VertexId) -> (u32, u32) {
    let a = v1.index();
    let b = v2.index();
    if a <= b { (a, b) } else { (b, a) }
}
