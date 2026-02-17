//! SplitEdge — split an edge by inserting a vertex at a parameter.
//!
//! DOMAIN: Takes an existing halfedge (A→B) and inserts a midpoint vertex M,
//! producing two halfedges (A→M) and (M→B) and their twins.
//!
//! INVARIANTS:
//! - The new vertex M is on the edge at parameter `t`
//! - All twin, next, prev pointers are correctly wired
//! - Euler formula: V+1, E+1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{HalfEdgeData, VertexData};
use crate::handles::HalfEdgeId;
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Split an existing edge by inserting a midpoint vertex.
///
/// Given halfedge `edge` (A→B), creates vertex M and splits into:
/// - `edge` becomes A→M
/// - `new_edge` becomes M→B  
/// - Plus their twins: `edge.twin` becomes M→A, `new_edge_twin` becomes B→M
///
/// Handles the degenerate case where A→B is a self-loop (twin == self).
#[derive(Debug)]
pub struct SplitEdge {
    /// The halfedge to split.
    pub edge: HalfEdgeId,
    /// Parameter along the edge (0.0 = at A, 1.0 = at B). Used for geometry.
    pub parameter: f64,
}

/// Output of the SplitEdge operator.
pub struct SplitEdgeOutput {
    /// The original halfedge, now A→M.
    pub he_am: HalfEdgeId,
    /// The new halfedge M→B (or M→A for self-loop).
    pub he_mb: HalfEdgeId,
    /// The new twin halfedge B→M (degenerate: same as he_mb for self-loop).
    pub he_bm: HalfEdgeId,
    /// The original twin, now M→A (degenerate: same as he_mb for self-loop).
    pub he_ma: HalfEdgeId,
    /// The newly created midpoint vertex.
    pub new_vertex: crate::handles::VertexId,
}

impl EulerOperator for SplitEdge {
    type Output = SplitEdgeOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he_ab = self.edge;
        let ab_data = draft.arena().get_half_edge(he_ab)?;

        let he_twin = ab_data.twin;
        let ab_next = ab_data.next;
        let ab_face = ab_data.face;
        let ab_lineage = ab_data.lineage.clone();

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_prev = twin_data.prev;
        let twin_face = twin_data.face;
        let vertex_b = twin_data.origin;
        let twin_lineage = twin_data.lineage.clone();

        let is_self_loop = he_ab == he_twin;

        let vertex_lineage = Lineage::derive_from(&ab_lineage, sig.clone());
        let he_mb_lineage = Lineage::derive_from(&ab_lineage, sig.clone());

        let new_vertex = draft.arena_mut().insert_vertex(VertexData {
            outgoing: HalfEdgeId::new(u32::MAX, 0),
            lineage: Some(vertex_lineage),
        });

        // ── Self-loop early return ──────────────────────────────────
        // A self-loop (twin == self) splits into exactly 2 halfedges
        // forming a proper edge: he_ab (A→M) ↔ he_mb (M→A).
        if is_self_loop {
            let he_mb = draft.arena_mut().insert_half_edge(HalfEdgeData {
                twin: he_ab,
                next: he_ab,
                prev: he_ab,
                face: ab_face,
                origin: new_vertex,
                lineage: Some(he_mb_lineage),
            });

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he_ab)?.twin = he_mb;
            arena.get_half_edge_mut(he_ab)?.next = he_mb;
            arena.get_half_edge_mut(he_ab)?.prev = he_mb;
            arena.get_vertex_mut(new_vertex)?.outgoing = he_mb;

            let loop_id = arena.get_face(ab_face)?.outer_loop;
            arena.get_loop_mut(loop_id)?.half_edge = he_ab;

            return Ok(SplitEdgeOutput {
                he_am: he_ab,
                he_mb,
                he_bm: he_mb,
                he_ma: he_mb,
                new_vertex,
            });
        }

        // ── Normal (non-self-loop) case ─────────────────────────────
        let he_bm_lineage = Lineage::derive_from(&twin_lineage, sig.clone());

        let (he_mb, he_bm) = draft.arena_mut().insert_half_edge_pair(
            HalfEdgeData {
                twin: HalfEdgeId::new(u32::MAX, 0),
                next: ab_next,
                prev: he_ab,
                face: ab_face,
                origin: new_vertex,
                lineage: Some(he_mb_lineage),
            },
            HalfEdgeData {
                twin: HalfEdgeId::new(u32::MAX, 0),
                next: he_twin,
                prev: twin_prev,
                face: twin_face,
                origin: vertex_b,
                lineage: Some(he_bm_lineage),
            },
        );

        let arena = draft.arena_mut();

        arena.get_half_edge_mut(he_ab)?.next = he_mb;
        arena.get_half_edge_mut(he_twin)?.origin = new_vertex;
        arena.get_half_edge_mut(he_twin)?.prev = he_bm;

        if ab_next != he_ab && ab_next != he_twin {
            arena.get_half_edge_mut(ab_next)?.prev = he_mb;
        }
        if twin_prev != he_twin && twin_prev != he_ab {
            arena.get_half_edge_mut(twin_prev)?.next = he_bm;
        }

        arena.get_vertex_mut(new_vertex)?.outgoing = he_mb;

        Ok(SplitEdgeOutput {
            he_am: he_ab,
            he_mb,
            he_bm,
            he_ma: he_twin,
            new_vertex,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("split_edge")
    }
}
