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

use crate::b_rep::{EdgeData, HalfEdgeData, VertexData};
use crate::handles::{EdgeId, HalfEdgeId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;


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

impl TopoOperator for MakeEdgeVertex {
    type Output = MevOutput;

    const NAME: &'static str = "make_edge_vertex";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let anchor = self.anchor;
        let anchor_data = draft.arena().get_half_edge(anchor)?;

        let origin = anchor_data.origin();
        let face = anchor_data.face();
        let prev = anchor_data.prev();
        let new_vertex = draft.insert_vertex(VertexData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));

        let new_edge = draft.insert_edge(EdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));

        let sentinel = HalfEdgeId::new(u32::MAX, 0);

        let (he_out, he_back) = draft.insert_radial_pair(
            HalfEdgeData::new(
                sentinel, // twin → set below
                sentinel, // next → set below
                sentinel, // prev → set below
                face,
                origin,
                new_edge,
            ),
            HalfEdgeData::new(
                sentinel, // twin → set below
                sentinel, // next → set below
                sentinel, // prev → set below
                face,
                new_vertex,
                new_edge,
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
        draft
            .arena_mut()
            .get_vertex_mut(new_vertex)?
            .set_outgoing(he_back);
        draft
            .arena_mut()
            .get_edge_mut(new_edge)?
            .set_half_edge(he_out);

        // ── Face version bump ───────────────────────────────────────
        draft.arena_mut().bump_face_version(face)?;

        Ok(ExecutionResult {
            value: MevOutput {
                new_vertex,
                he_out,
                he_back,
                edge: new_edge,
            },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: 2,
                faces: 0,
                loops: 0,
                edges: 1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
