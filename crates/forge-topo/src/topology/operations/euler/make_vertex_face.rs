//! MakeVertexFace — create the topological seed.
//!
//! DOMAIN: Creates the initial vertex + face + loop + degenerate halfedge
//! + shell + edge from which all topology is grown.
//!
//! INVARIANTS:
//! - Creates exactly 1 vertex, 1 face, 1 loop, 1 halfedge (self-loop), 1 shell, 1 edge
//! - The halfedge is its own twin, next, and prev
//! - All entities carry root lineage from the provided `OpSignature`
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{FaceData, HalfEdgeData, LoopData, VertexData, ShellData, EdgeData, ShellOrientation};
use crate::handles::{HalfEdgeId, LoopId, ShellId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{ExecutionResult, EulerDelta};
use crate::EulerOperator;
use crate::state::MutableDraft;

/// Creates the topological seed: one vertex, one face, one loop, one selfloop halfedge,
/// one shell, and one edge.
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
    /// The created shell.
    pub shell: ShellId,
    /// The created edge (self-loop edge).
    pub edge: EdgeId,
}

impl EulerOperator for MakeVertexFace {
    type Output = MvfOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let vertex_lineage = Lineage::root(0, sig.clone());
        let face_lineage = Lineage::root(1, sig.clone());
        let he_lineage = Lineage::root(2, sig.clone());
        let shell_lineage = Lineage::root(3, sig.clone());
        let edge_lineage = Lineage::root(4, sig.clone());

        

        let vertex = draft.insert_vertex(VertexData::with_lineage(
            placeholder_he,
            Some(vertex_lineage),
        ));

        let shell = draft.insert_shell(ShellData::with_lineage(
            crate::handles::FaceId::new(u32::MAX, 0),
            ShellOrientation::Outer,
            Some(shell_lineage),
        ));

        let face = draft.insert_face(FaceData::with_lineage(
            placeholder_loop,
            shell,
            Some(face_lineage),
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        let edge = draft.insert_edge(EdgeData::with_lineage(
            placeholder_he,
            Some(edge_lineage),
        ));

        let he = draft.insert_half_edge(HalfEdgeData::with_lineage(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            edge,
            Some(he_lineage),
        ));

        
        draft.arena_mut().get_half_edge_mut(he)?.set_twin(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_next(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_prev(he);
        draft.arena_mut().get_vertex_mut(vertex)?.set_outgoing(he);
        draft.arena_mut().get_face_mut(face)?.set_outer_loop(loop_id);
        draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he);
        draft.arena_mut().get_shell_mut(shell)?.set_representative_face(face);
        draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);

        Ok(ExecutionResult {
            value: MvfOutput {
                face,
                vertex,
                half_edge: he,
                loop_id,
                shell,
                edge,
            },
            declared_delta: EulerDelta { vertices: 1, half_edges: 1, faces: 1, loops: 1, edges: 1, shells: 1 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_vertex_face")
    }
}
