//! MakeEdgeVertex — extend a vertex by sprouting a new edge and vertex.
//!
//! DOMAIN: Given an anchor halfedge, inserts a new edge+vertex at the
//! anchor's origin, creating an "antenna" within the face loop.
//!
//! INVARIANTS:
//! - Creates 1 vertex, 2 halfedges, 1 edge
//! - Euler formula: V+1, E+1 (net: same V-E+F)
//! - Both new halfedges are twins on the SAME face (wire edge)
//! - The existing vertex's outgoing pointer is NOT modified
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{HalfEdgeData, VertexData, EdgeData};
use crate::handles::{HalfEdgeId, VertexId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};
use crate::state::MutableDraft;

/// Extend a vertex by sprouting a new edge and vertex (antenna).
///
/// The anchor halfedge defines the exact topological wedge for insertion:
/// the new edge is spliced between `anchor.prev` and `anchor` in the
/// face loop. This eliminates the wedge ambiguity that arises when a
/// vertex touches the same face multiple times.
///
/// # Wire Edges
///
/// Both new halfedges belong to the **same** face. This is a valid
/// topological construction (wire edge / antenna), not a manifold defect.
///
/// # Degenerate Case
///
/// When the anchor is a self-loop halfedge (from `MakeVertexFace`),
/// the result is a 3-halfedge loop: `anchor → he_out → he_back → anchor`.
#[derive(Debug)]
pub struct MakeEdgeVertex {
    /// The anchor halfedge. The new edge sprouts at `anchor.origin()`,
    /// inserted between `anchor.prev` and `anchor` in the face loop.
    pub anchor: HalfEdgeId,
}

/// Output of the MakeEdgeVertex operator.
pub struct MevOutput {
    /// The newly created tip vertex.
    pub new_vertex: VertexId,
    /// Halfedge from anchor's origin → new_vertex.
    pub he_out: HalfEdgeId,
    /// Halfedge from new_vertex → anchor's origin (twin of he_out).
    pub he_back: HalfEdgeId,
    /// The newly created edge entity.
    pub edge: EdgeId,
}

impl EulerOperator for MakeEdgeVertex {
    type Output = MevOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let anchor = self.anchor;
        let anchor_data = draft.arena().get_half_edge(anchor)?;

        let origin = anchor_data.origin();
        let face = anchor_data.face();
        let prev = anchor_data.prev();
        let anchor_lineage = anchor_data.lineage().cloned();

        let vertex_lineage = Lineage::derive_from(&anchor_lineage, sig.clone());
        let he_out_lineage = Lineage::derive_from(&anchor_lineage, sig.clone());
        let he_back_lineage = Lineage::derive_from(&anchor_lineage, sig.clone());
        let edge_lineage = Lineage::derive_from(&anchor_lineage, sig.clone());

        let new_vertex = draft.insert_vertex(VertexData::with_lineage(
            HalfEdgeId::new(u32::MAX, 0),
            Some(vertex_lineage),
        ));

        let new_edge = draft.insert_edge(EdgeData::with_lineage(
            HalfEdgeId::new(u32::MAX, 0),
            Some(edge_lineage),
        ));

        let placeholder = HalfEdgeId::new(u32::MAX, 0);

        let (he_out, he_back) = draft.insert_radial_pair(
            HalfEdgeData::with_lineage(
                placeholder, // twin → set below
                placeholder, // next → set below
                placeholder, // prev → set below
                face,
                origin,
                new_edge,
                Some(he_out_lineage),
            ),
            HalfEdgeData::with_lineage(
                placeholder, // twin → set below
                placeholder, // next → set below
                placeholder, // prev → set below
                face,
                new_vertex,
                new_edge,
                Some(he_back_lineage),
            ),
        );

        // ── Splice into the face loop ───────────────────────────────
        // Before: ... → prev → anchor → ...
        // After:  ... → prev → he_out → he_back → anchor → ...
        let arena = draft.arena_mut();

        arena.get_half_edge_mut(prev)?.set_next(he_out);
        arena.get_half_edge_mut(he_out)?.set_prev(prev);

        arena.get_half_edge_mut(he_out)?.set_next(he_back);
        arena.get_half_edge_mut(he_back)?.set_prev(he_out);

        arena.get_half_edge_mut(he_back)?.set_next(anchor);
        arena.get_half_edge_mut(anchor)?.set_prev(he_back);

        // ── Entity ownership pointers ───────────────────────────────
        draft.arena_mut().get_vertex_mut(new_vertex)?.set_outgoing(he_back);
        draft.arena_mut().get_edge_mut(new_edge)?.set_half_edge(he_out);

        // ── Face version bump ───────────────────────────────────────
        draft.arena_mut().bump_face_version(face)?;

        Ok(ExecutionResult {
            value: MevOutput {
                new_vertex,
                he_out,
                he_back,
                edge: new_edge,
            },
            declared_delta: EulerDelta { vertices: 1, half_edges: 2, faces: 0, loops: 0, edges: 1, shells: 0, solids: 0, lumps: 0, regions: 0 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_edge_vertex")
    }
}
