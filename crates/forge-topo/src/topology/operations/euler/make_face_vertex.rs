//! MakeFaceVertex — create a new face with a single vertex inside an existing shell.
//!
//! DOMAIN: Create a disjoint face, vertex, loop, halfedge, and edge within an existing shell.
//!
//! INVARIANTS:
//! - ΔV=+1, ΔHE=+1, ΔF=+1, ΔL=+1, ΔE=+1
//! - The new halfedge is a self-loop: `twin == next == prev == self`.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{EdgeData, FaceData, HalfEdgeData, LoopData, VertexData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Creates a new face with a single vertex inside an existing shell.
#[derive(Debug)]
pub struct MakeFaceVertex {
    /// The parent shell that will own the new face.
    pub shell: ShellId,
}

/// Output of the MakeFaceVertex operator.
pub struct MfvOutput {
    /// The created vertex.
    pub vertex: VertexId,
    /// The created face.
    pub face: FaceId,
    /// The created halfedge (self-loop).
    pub half_edge: HalfEdgeId,
    /// The created loop.
    pub loop_id: LoopId,
    /// The created edge (self-loop edge).
    pub edge: EdgeId,
}

impl EulerOperator for MakeFaceVertex {
    type Output = MfvOutput;

    const NAME: &'static str = "make_face_vertex";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);




        let vertex = draft.insert_vertex(VertexData::new(
            placeholder_he,
        ));

        let face = draft.insert_face(FaceData::new(
            placeholder_loop,
            self.shell,
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        let edge = draft.insert_edge(EdgeData::new(placeholder_he));

        let he = draft.insert_half_edge(HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            edge,
        ));

        draft.arena_mut().get_half_edge_mut(he)?.set_radial_next(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_next(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_prev(he);
        draft.arena_mut().get_vertex_mut(vertex)?.set_outgoing(he);
        draft
            .arena_mut()
            .get_face_mut(face)?
            .set_outer_loop(loop_id);
        draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he);
        draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);

        Ok(ExecutionResult {
            value: MfvOutput {
                face,
                vertex,
                half_edge: he,
                loop_id,
                edge,
            },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: 1,
                faces: 1,
                loops: 1,
                edges: 1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
