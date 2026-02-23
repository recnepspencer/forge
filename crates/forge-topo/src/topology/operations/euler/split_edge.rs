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

use crate::arena::{HalfEdgeData, VertexData, EdgeData};
use crate::handles::{HalfEdgeId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};
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
///
/// # Degenerate Case (Self-Loop Split)
///
/// When splitting a self-loop halfedge (one where `twin == self`),
/// the fields `he_bm` and `he_ma` will alias `he_mb`. This is
/// correct — the degenerate case has fewer distinct entities.
/// Use `is_degenerate()` to detect this case.
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

impl SplitEdgeOutput {
    /// Whether this split was on a self-loop (degenerate case).
    ///
    /// When `true`, `he_bm == he_mb` and `he_ma == he_mb`.
    pub fn is_degenerate(&self) -> bool {
        self.he_bm == self.he_mb
    }
}

impl EulerOperator for SplitEdge {
    type Output = SplitEdgeOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he_ab = self.edge;
        let ab_data = draft.arena().get_half_edge(he_ab)?;

        let he_twin = ab_data.twin();
        let ab_face = ab_data.face();
        let ab_lineage = ab_data.lineage().cloned();

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_face = twin_data.face();
        let vertex_b = twin_data.origin();
        let twin_lineage = twin_data.lineage().cloned();

        let is_self_loop = he_ab == he_twin;

        let vertex_lineage = Lineage::derive_from(&ab_lineage, sig.clone());
        let he_mb_lineage = Lineage::derive_from(&ab_lineage, sig.clone());

        let new_vertex = draft.insert_vertex(VertexData::with_lineage(
            HalfEdgeId::new(u32::MAX, 0),
            Some(vertex_lineage),
        ));
        draft.arena_mut().get_vertex_mut(new_vertex)?.set_birth_parameter(Some(self.parameter));

        // ── Self-loop early return ──────────────────────────────────
        if is_self_loop {
            let new_edge = draft.insert_edge(EdgeData::with_lineage(
                HalfEdgeId::new(u32::MAX, 0),
                Some(he_mb_lineage.clone()),
            ));

            let he_mb = draft.insert_half_edge(HalfEdgeData::with_lineage(
                he_ab,
                he_ab,
                he_ab,
                ab_face,
                new_vertex,
                new_edge,
                Some(he_mb_lineage),
            ));

            
            draft.arena_mut().get_half_edge_mut(he_ab)?.set_twin(he_mb);
            draft.arena_mut().get_half_edge_mut(he_ab)?.set_next(he_mb);
            draft.arena_mut().get_half_edge_mut(he_ab)?.set_prev(he_mb);
            draft.arena_mut().get_vertex_mut(new_vertex)?.set_outgoing(he_mb);
            draft.arena_mut().get_edge_mut(new_edge)?.set_half_edge(he_mb);

            let loop_id = draft.arena().get_face(ab_face)?.outer_loop();
            draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he_ab);

            draft.arena_mut().bump_face_version(ab_face)?;

            return Ok(ExecutionResult {
                value: SplitEdgeOutput {
                    he_am: he_ab,
                    he_mb,
                    he_bm: he_mb,
                    he_ma: he_mb,
                    new_vertex,
                },
                declared_delta: EulerDelta { vertices: 1, half_edges: 1, faces: 0, loops: 0, edges: 1, shells: 0 },
            });
        }

        // ── Normal (non-self-loop) case ─────────────────────────────
        let he_bm_lineage = Lineage::derive_from(&twin_lineage, sig.clone());

        let new_edge = draft.insert_edge(EdgeData::with_lineage(
            HalfEdgeId::new(u32::MAX, 0),
            Some(Lineage::derive_from(&ab_lineage, sig.clone())),
        ));

        let placeholder = HalfEdgeId::new(u32::MAX, 0);
        let (he_mb, he_bm) = draft.insert_half_edge_pair(
            HalfEdgeData::with_lineage(
                placeholder,
                placeholder,
                placeholder,
                ab_face,
                new_vertex,
                new_edge,
                Some(he_mb_lineage),
            ),
            HalfEdgeData::with_lineage(
                placeholder,
                placeholder,
                placeholder,
                twin_face,
                vertex_b,
                new_edge,
                Some(he_bm_lineage),
            ));

        draft.arena_mut().get_half_edge_mut(he_twin)?.set_origin(new_vertex);

        let am_old_next = draft.arena().get_half_edge(he_ab)?.next();
        draft.arena_mut().get_half_edge_mut(he_ab)?.set_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb)?.set_prev(he_ab);
        draft.arena_mut().get_half_edge_mut(he_mb)?.set_next(am_old_next);
        draft.arena_mut().get_half_edge_mut(am_old_next)?.set_prev(he_mb);

        let ma_old_prev = draft.arena().get_half_edge(he_twin)?.prev();
        draft.arena_mut().get_half_edge_mut(ma_old_prev)?.set_next(he_bm);
        draft.arena_mut().get_half_edge_mut(he_bm)?.set_prev(ma_old_prev);
        draft.arena_mut().get_half_edge_mut(he_bm)?.set_next(he_twin);
        draft.arena_mut().get_half_edge_mut(he_twin)?.set_prev(he_bm);

        draft.arena_mut().get_vertex_mut(new_vertex)?.set_outgoing(he_mb);

        let vb_outgoing = draft.arena().get_vertex(vertex_b)?.outgoing();
        if vb_outgoing == he_twin {
            draft.arena_mut().get_vertex_mut(vertex_b)?.set_outgoing(he_bm);
        }

        draft.arena_mut().bump_face_version(ab_face)?;
        if twin_face != ab_face {
            draft.arena_mut().bump_face_version(twin_face)?;
        }
        draft.arena_mut().get_edge_mut(new_edge)?.set_half_edge(he_mb);

        Ok(ExecutionResult {
            value: SplitEdgeOutput {
                he_am: he_ab,
                he_mb,
                he_bm,
                he_ma: he_twin,
                new_vertex,
            },
            declared_delta: EulerDelta { vertices: 1, half_edges: 2, faces: 0, loops: 0, edges: 1, shells: 0 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("split_edge")
    }
}
