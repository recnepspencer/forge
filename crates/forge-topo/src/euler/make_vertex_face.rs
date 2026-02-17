//! MVF — Make Vertex Face.
//!
//! DOMAIN: Creates the initial topological seed — a single vertex inside
//! a single face bounded by a degenerate loop.
//!
//! This is the starting point for all solid construction via Euler operators.
//! From here, `MakeEdgeFace` can split the face and introduce edges.

use forge_core::KernelError;
use crate::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Create a single vertex inside a single face with a degenerate loop.
///
/// The degenerate loop consists of a single halfedge that points to itself
/// for next, prev, and twin. This is the topological seed from which all
/// other operators build structure.
///
/// # Returns
/// `MvfOutput` — the created vertex, face, loop, and halfedge.
#[derive(Debug)]
pub struct MakeVertexFace {
    /// Feature ID for lineage tracking (0 = default).
    pub feature_id: u64,
}

/// Output of the MakeVertexFace operator.
pub struct MvfOutput {
    /// The created vertex.
    pub vertex: VertexId,
    /// The created face.
    pub face: FaceId,
    /// The created loop.
    pub loop_id: LoopId,
    /// The degenerate halfedge forming the loop.
    pub half_edge: HalfEdgeId,
}

impl EulerOperator for MakeVertexFace {
    type Output = MvfOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let root_lineage = Lineage::root(self.feature_id, sig.clone());
        let arena = draft.arena_mut();

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let vertex = arena.insert_vertex(VertexData {
            outgoing: placeholder_he,
            lineage: Some(root_lineage.clone()),
        });

        let face = arena.insert_face(FaceData {
            outer_loop: placeholder_loop,
            lineage: Some(root_lineage.clone()),
        });

        let loop_id = arena.insert_loop(LoopData {
            half_edge: placeholder_he,
            face,
        });

        let half_edge = arena.insert_half_edge(HalfEdgeData {
            twin: placeholder_he,
            next: placeholder_he,
            prev: placeholder_he,
            face,
            origin: vertex,
            lineage: Some(root_lineage),
        });

        arena.get_half_edge_mut(half_edge)?.twin = half_edge;
        arena.get_half_edge_mut(half_edge)?.next = half_edge;
        arena.get_half_edge_mut(half_edge)?.prev = half_edge;

        arena.get_vertex_mut(vertex)?.outgoing = half_edge;
        arena.get_face_mut(face)?.outer_loop = loop_id;
        arena.get_loop_mut(loop_id)?.half_edge = half_edge;

        Ok(MvfOutput { vertex, face, loop_id, half_edge })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_vertex_face")
    }
}
