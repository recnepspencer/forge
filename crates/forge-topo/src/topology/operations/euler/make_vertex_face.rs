//! MakeVertexFace — create the topological seed.
//!
//! DOMAIN: Creates the initial vertex + face + loop + degenerate halfedge
//! from which all topology is grown.
//!
//! INVARIANTS:
//! - Creates exactly 1 vertex, 1 face, 1 loop, 1 halfedge (self-loop)
//! - The halfedge is its own twin, next, and prev
//! - All entities carry root lineage from the provided `OpSignature`
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use crate::handles::{HalfEdgeId, LoopId};
use crate::lineage::{Lineage, OpSignature};
use crate::EulerOperator;
use crate::state::MutableDraft;

/// Creates the topological seed: one vertex, one face, one loop, one selfloop halfedge.
///
/// This is always the first operator applied to an empty draft.
/// The halfedge is a degenerate self-loop: `twin == next == prev == self`.
#[derive(Debug)]
pub struct MakeVertexFace;

/// Output of the MakeVertexFace operator.
pub struct MvfOutput {
    /// The created vertex.
    pub vertex: crate::handles::VertexId,
    /// The created face.
    pub face: crate::handles::FaceId,
    /// The created halfedge (self-loop).
    pub half_edge: HalfEdgeId,
    /// The created loop.
    pub loop_id: crate::handles::LoopId,
}

impl EulerOperator for MakeVertexFace {
    type Output = MvfOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let vertex_lineage = Lineage::root(0, sig.clone());
        let face_lineage = Lineage::root(1, sig.clone());
        let he_lineage = Lineage::root(2, sig.clone());

        let arena = draft.arena_mut();

        let vertex = arena.insert_vertex(VertexData::with_lineage(
            placeholder_he,
            Some(vertex_lineage),
        ));

        let face = arena.insert_face(FaceData::with_lineage(
            placeholder_loop,
            Some(face_lineage),
        ));

        let loop_id = arena.insert_loop(LoopData::new(placeholder_he, face));

        let he = arena.insert_half_edge(HalfEdgeData::with_lineage(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            Some(he_lineage),
        ));

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he)?.set_twin(he);
        arena.get_half_edge_mut(he)?.set_next(he);
        arena.get_half_edge_mut(he)?.set_prev(he);
        arena.get_vertex_mut(vertex)?.set_outgoing(he);
        arena.get_face_mut(face)?.set_outer_loop(loop_id);
        arena.get_loop_mut(loop_id)?.set_half_edge(he);

        Ok(MvfOutput {
            vertex,
            face,
            half_edge: he,
            loop_id,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_vertex_face")
    }
}
