//! Topological queries for matching and grouping directed boundary edges.
//!
//! DOMAIN: Bulk querying and mapping of half-edges by their structural endpoints.
//! Uses explicit cycle guards and valid indices.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;

use crate::b_rep::TopologyArena;
use crate::handles::{HalfEdgeId, EdgeId, VertexId};

/// Result of building a directed edge map.
pub struct EdgeMapResult {
    /// Map of (origin_index, dest_index) -> list of halfedges sharing that directed geometry.
    pub forward_map: BTreeMap<(u32, u32), Vec<HalfEdgeId>>,
    /// Set of halfedges that have zero length (origin == dest).
    pub zero_length: BTreeSet<u32>,
}

/// Build a forward map from (origin, dest) -> Vec<HalfEdgeId>, classifying zero-length edges.
pub fn build_edge_map(
    arena: &TopologyArena,
    halfedges: &[HalfEdgeId],
) -> Result<EdgeMapResult, KernelError> {
    let mut forward_map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    let mut zero_length: BTreeSet<u32> = BTreeSet::new();

    for &he_id in halfedges {
        let edge_id = arena.get_half_edge(he_id)?.edge();
        let (origin, dest) = get_edge_endpoints(arena, edge_id)?;
        if origin == dest {
            zero_length.insert(he_id.index());
        } else {
            forward_map
                .entry((origin.index() as u32, dest.index() as u32))
                .or_default()
                .push(he_id);
        }
    }

    Ok(EdgeMapResult {
        forward_map,
        zero_length,
    })
}

/// Build a directed map for a subset of half-edge IDs without zero-length checks.
pub fn build_directed_map(
    arena: &TopologyArena,
    ids: &[HalfEdgeId],
) -> Result<BTreeMap<(u32, u32), Vec<HalfEdgeId>>, KernelError> {
    let mut map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    for &he_id in ids {
        let he = arena.get_half_edge(he_id)?;
        let edge_id = he.edge();
        let (origin, dest) = get_edge_endpoints(arena, edge_id)?;
        map.entry((origin.index() as u32, dest.index() as u32))
            .or_default()
            .push(he_id);
    }
    Ok(map)
}

/// Helper to fetch topological endpoints of an undirected edge.
fn get_edge_endpoints(
    arena: &TopologyArena,
    edge_id: EdgeId,
) -> Result<(VertexId, VertexId), KernelError> {
    let he_id = arena.get_edge(edge_id)?.half_edge();
    let he = arena.get_half_edge(he_id)?;
    let origin = he.origin();
    let dest = arena.get_half_edge(he.next())?.origin();
    Ok((origin, dest))
}
