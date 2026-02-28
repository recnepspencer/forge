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

use crate::b_rep::TopologyArena;
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
pub fn radial_uses(arena: &TopologyArena, he: HalfEdgeId) -> Result<Vec<HalfEdgeId>, KernelError> {
    RadialEdgeIterator::new(arena, he)?.collect::<Result<Vec<_>, _>>()
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
    use crate::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::handles::{EdgeId, LoopId, ShellId};
    use crate::transactions::TopologyState;
    use forge_core::KernelError;

    // ── helpers ─────────────────────────────────────────────────────────

    /// Build a bare seed halfedge in a draft arena.
    fn seed_state() -> (TopologyState, HalfEdgeId) {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();
        (state, mvf.half_edge)
    }

    /// Directly wire a 3-ring around `start`:  start → mid → extra → start
    /// Returns `(start, mid, extra)`.
    ///
    /// Caller must ensure the arena is in a MutableDraft and the halfedges
    /// already have valid pointers for all fields except radial_next.
    fn wire_3_ring(
        draft: &mut crate::transactions::MutableDraft,
        start: HalfEdgeId,
        mid: HalfEdgeId,
    ) -> HalfEdgeId {
        let face = draft.arena().get_half_edge(start).unwrap().face();
        let origin = draft.arena().get_half_edge(start).unwrap().origin();
        let edge = draft.arena().get_half_edge(start).unwrap().edge();

        // Unify edge entity: mid must share start's edge for a valid ring.
        draft
            .arena_mut()
            .get_half_edge_mut(mid)
            .unwrap()
            .set_edge(edge);

        // Insert ghost halfedge to form the 3rd ring member.
        let extra = draft.insert_half_edge(HalfEdgeData::new(
            start, // radial_next sentinel — overwritten below
            start, // next (same face, safe dummy)
            start, // prev
            face, origin, edge,
        ));

        // Wire ring: start → mid → extra → start
        draft
            .arena_mut()
            .get_half_edge_mut(start)
            .unwrap()
            .set_radial_next(mid);
        draft
            .arena_mut()
            .get_half_edge_mut(mid)
            .unwrap()
            .set_radial_next(extra);
        draft
            .arena_mut()
            .get_half_edge_mut(extra)
            .unwrap()
            .set_radial_next(start);
        extra
    }

    // ── 1. Boundary / manifold cases ────────────────────────────────────

    /// Self-radial halfedge (boundary edge, valence 1): ring contains exactly itself.
    #[test]
    fn radial_uses_self_radial_returns_exactly_one() {
        let (state, he) = seed_state();
        let uses = radial_uses(state.arena(), he).unwrap();
        assert_eq!(uses.len(), 1, "valence-1 edge must return exactly [he]");
        assert_eq!(uses[0], he, "first element must be the starting halfedge");
    }

    /// After SplitEdge the resulting halfedges are both self-radial (wire edges).
    #[test]
    fn radial_uses_split_result_is_still_self_radial() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        // Read both halfedge IDs before commit
        let he_am = se.he_am;
        let he_mb = se.he_mb;
        let state = draft.commit().unwrap();

        let uses_am = radial_uses(state.arena(), he_am).unwrap();
        let uses_mb = radial_uses(state.arena(), he_mb).unwrap();
        assert_eq!(uses_am.len(), 1, "he_am post-split must be self-radial");
        assert_eq!(uses_mb.len(), 1, "he_mb post-split must be self-radial");
        // Crucially: the two halfedges are NOT in each other's rings
        assert!(
            !uses_am.contains(&he_mb),
            "he_mb must not appear in he_am's radial ring post-split"
        );
    }

    // ── 2. NMT / valence-3 cases (the real risk area for Epic B) ────────

    /// radial_uses on a valence-3 ring returns all three members in ring-walk order.
    ///
    /// This is the critical property for merge planning: if the ring walk drops
    /// an element or oscillates, merge step ordering becomes non-deterministic.
    #[test]
    fn radial_uses_valence_3_ring_returns_all_three_members() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let start = se.he_am;
        let mid = se.he_mb;
        let extra = wire_3_ring(&mut draft, start, mid);

        // query *before* commit to avoid ManifoldStrict rejection
        let uses = radial_uses(draft.arena(), start).unwrap();
        assert_eq!(
            uses.len(),
            3,
            "valence-3 ring must return 3 members, got {:?}",
            uses.len()
        );

        // Ring walk must be start → mid → extra → back to start
        assert_eq!(uses[0], start);
        assert_eq!(uses[1], mid);
        assert_eq!(uses[2], extra);
    }

    /// radial_uses is closed: the last element's radial_next is the first element.
    ///
    /// Failing this means the iterator either stops early or doesn't close,
    /// which would leave some uses unreachable by the merge planner.
    #[test]
    fn radial_uses_valence_3_ring_is_closed() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let extra = wire_3_ring(&mut draft, se.he_am, se.he_mb);

        let uses = radial_uses(draft.arena(), se.he_am).unwrap();
        // The ring must close: radial_next of the last element = first element
        let last = *uses.last().unwrap();
        assert_eq!(last, extra);
        let last_data = draft.arena().get_half_edge(last).unwrap();
        assert_eq!(
            last_data.radial_next(),
            se.he_am,
            "last ring member must point back to start — ring is not closed"
        );
    }

    /// Starting radial_uses from any member of the same ring must return exactly
    /// the same set (different rotation, same members).
    ///
    /// This is the re-derivation invariant: MergeStepPlan can start from any
    /// halfedge around the edge and get a complete picture of the radial uses.
    #[test]
    fn radial_uses_result_is_invariant_to_start_member() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let extra = wire_3_ring(&mut draft, se.he_am, se.he_mb);

        let uses_from_start = radial_uses(draft.arena(), se.he_am).unwrap();
        let uses_from_mid = radial_uses(draft.arena(), se.he_mb).unwrap();
        let uses_from_extra = radial_uses(draft.arena(), extra).unwrap();

        // Each rotation must contain all 3 members exactly once
        let expected: std::collections::BTreeSet<_> = [se.he_am, se.he_mb, extra].into();
        let set_start: std::collections::BTreeSet<_> = uses_from_start.into_iter().collect();
        let set_mid: std::collections::BTreeSet<_> = uses_from_mid.into_iter().collect();
        let set_extra: std::collections::BTreeSet<_> = uses_from_extra.into_iter().collect();

        assert_eq!(set_start, expected, "starting from start: wrong member set");
        assert_eq!(set_mid, expected, "starting from mid:   wrong member set");
        assert_eq!(set_extra, expected, "starting from extra: wrong member set");
    }

    /// radial_uses_by_face on a valence-3 ring with all halfedges from the same
    /// face returns one entry with three halfedges — this is the slit detection case.
    ///
    /// If a face appears multiple times (slit topology), the grouping must
    /// accumulate all of its halfedges into one entry, not silently discard duplicates.
    #[test]
    fn radial_uses_by_face_detects_same_face_slit() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let extra = wire_3_ring(&mut draft, se.he_am, se.he_mb);

        // All three halfedges belong to the same face (no MEF was called).
        let face = draft.arena().get_half_edge(se.he_am).unwrap().face();

        let by_face = radial_uses_by_face(draft.arena(), se.he_am).unwrap();
        // Should be one face key with 3 halfedges (slit: same face, multiple uses)
        assert_eq!(
            by_face.len(),
            1,
            "all 3 halfedges on same face: map must have 1 entry"
        );
        let hes = &by_face[&face];
        assert_eq!(
            hes.len(),
            3,
            "all 3 ring members must appear under the face's entry"
        );

        // Extra check: the extra ghost halfedge must appear in the face's list
        assert!(
            hes.contains(&extra),
            "ghost halfedge must be recorded under the owning face"
        );
    }

    /// radial_uses_by_face determinism: calling with the same arena state twice
    /// must produce identical BTreeMap output (same keys, same ordering of values).
    ///
    /// This is the audit/trace requirement: MergePlan steps recorded from
    /// radial_uses_by_face must be replay-identical.
    #[test]
    fn radial_uses_by_face_is_deterministic() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _ = wire_3_ring(&mut draft, se.he_am, se.he_mb);

        // Call twice on the same arena without any mutation in between.
        let by_face_1 = radial_uses_by_face(draft.arena(), se.he_am).unwrap();
        let by_face_2 = radial_uses_by_face(draft.arena(), se.he_am).unwrap();
        assert_eq!(
            by_face_1, by_face_2,
            "determinism violated: two calls differ"
        );
    }

    // ── 3. Error / safety cases ──────────────────────────────────────────

    /// A nonexistent handle must return Err rather than panicking or returning
    /// empty-Ok. This guards against silent data loss in merge planning.
    #[test]
    fn radial_uses_nonexistent_handle_returns_err() {
        let arena = TopologyArena::new();
        let bad = HalfEdgeId::new(99_999, 0);
        let result = radial_uses(&arena, bad);
        assert!(
            result.is_err(),
            "nonexistent handle must return Err, not Ok([])"
        );
        assert!(
            matches!(
                result.unwrap_err(),
                KernelError::TopologyViolation { .. } | KernelError::InternalError { .. }
            ),
            "error kind must be TopologyViolation or InternalError"
        );
    }

    /// A radial ring that points to a valid-index but wrong-generation handle
    /// (simulating a stale handle) must return Err on iteration.
    #[test]
    fn radial_uses_stale_radial_next_returns_err_on_iteration() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();

        // Point radial_next to a slot with the wrong generation — stale handle
        let stale = HalfEdgeId::new(
            mvf.half_edge.index(),
            mvf.half_edge.generation().wrapping_add(1), // wrong generation
        );
        draft
            .arena_mut()
            .get_half_edge_mut(mvf.half_edge)
            .unwrap()
            .set_radial_next(stale);

        let result = radial_uses(draft.arena(), mvf.half_edge);
        assert!(
            result.is_err(),
            "stale radial_next handle must propagate as Err, not silently stop"
        );
    }

    /// radial_uses_by_face on an empty arena with a bad handle must return Err.
    #[test]
    fn radial_uses_by_face_bad_handle_returns_err() {
        let arena = TopologyArena::new();
        let bad = HalfEdgeId::new(0, 0);
        assert!(radial_uses_by_face(&arena, bad).is_err());
    }
}
