//! Radial edge query helpers.
//!
//! DOMAIN: Snapshots of radial ring membership around a halfedge's geometric edge.
//!
//! All IDs returned are snapshot-scoped: they are valid only for the arena
//! snapshot used to compute them and must be re-derived after any topology mutation.
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs), `queries/traverse` (RadialEdgeIterator)

use std::collections::BTreeMap;

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId};
use crate::topology::queries::traverse::RadialEdgeIterator;

/// Snapshot-scoped position in a radial ring.
///
/// Encodes which halfedge (by edge index) and what position in the ring it
/// occupies. This value is **invalid after any topology mutation** — it must
/// not be stored across Euler operator calls or draft commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadialUseIndex {
    /// Index of the edge (from the edge's first halfedge).
    pub edge_index: u32,
    /// Zero-based position in the radial ring walk order.
    pub position: u32,
}

/// Collect all halfedge uses around the radial ring of `he`, in ring-walk order.
///
/// The starting halfedge `he` is always the first element of the returned `Vec`.
/// Returns an error if `he` is invalid or if the radial ring is corrupt (exceeds
/// the iteration limit in `RadialEdgeIterator`).
///
/// **Snapshot-scoped**: the returned `HalfEdgeId` values are valid only for the
/// arena snapshot at the time of this call. Re-derive after any mutation.
pub fn radial_uses(
    arena: &TopologyArena,
    he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    RadialEdgeIterator::new(arena, he)?
        .collect::<Result<Vec<_>, _>>()
}

/// Collect radial uses around `he`'s ring, grouped by owning face.
///
/// Keys are `FaceId`; values are the halfedges on that face in ring-walk order
/// (most commonly exactly one per face on manifold topology, potentially more on
/// non-manifold or slit structures).
///
/// `BTreeMap` is used to provide deterministic iteration order over face IDs.
///
/// **Snapshot-scoped**: the returned handles are valid only for this arena snapshot.
pub fn radial_uses_by_face(
    arena: &TopologyArena,
    he: HalfEdgeId,
) -> Result<BTreeMap<FaceId, Vec<HalfEdgeId>>, KernelError> {
    let mut map: BTreeMap<FaceId, Vec<HalfEdgeId>> = BTreeMap::new();
    for result in RadialEdgeIterator::new(arena, he)? {
        let use_he = result?;
        let he_data = arena.get_half_edge(use_he)?;
        map.entry(he_data.face()).or_default().push(use_he);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;

    fn two_edge_chain() -> (TopologyState, HalfEdgeId) {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(
            &mut draft,
            SplitEdge { edge: mvf.half_edge, parameter: 0.5 },
        )
        .unwrap()
        .into_value();
        let state = draft.commit().unwrap();
        (state, mvf.half_edge)
    }

    /// A seed halfedge has valence 1 (self-radial): radial ring contains only itself.
    #[test]
    fn radial_uses_boundary_edge_returns_one() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let he = mvf.half_edge;
        let state = draft.commit().unwrap();

        let uses = radial_uses(state.arena(), he).unwrap();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses[0], he);
    }

    /// After a split the original halfedge is still self-radial (wire edge, valence 1).
    #[test]
    fn radial_uses_manifold_boundary_edge_after_split() {
        let (state, he1) = two_edge_chain();

        let uses1 = radial_uses(state.arena(), he1).unwrap();
        assert_eq!(uses1.len(), 1, "original he should still be self-radial after SplitEdge");
    }

    /// radial_uses_by_face on a self-radial halfedge returns a single-entry map.
    #[test]
    fn radial_uses_by_face_groups_correctly() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let he = mvf.half_edge;
        let state = draft.commit().unwrap();

        let he_data = state.arena().get_half_edge(he).unwrap();
        let face = he_data.face();

        let by_face = radial_uses_by_face(state.arena(), he).unwrap();
        assert_eq!(by_face.len(), 1);
        assert!(by_face.contains_key(&face));
        assert_eq!(by_face[&face], vec![he]);
    }

    /// Passing a stale / nonexistent handle returns an error, not a panic.
    #[test]
    fn radial_uses_invalid_handle_returns_error() {
        let arena = TopologyArena::new();
        // A handle pointing at slot 99, generation 0 — never inserted.
        let bad_he = HalfEdgeId::from_raw_parts(99, 0);
        assert!(radial_uses(&arena, bad_he).is_err());
    }
}
