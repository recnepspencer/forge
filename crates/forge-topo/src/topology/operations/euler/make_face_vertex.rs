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
use crate::lineage::{Lineage, OpSignature};
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

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let vertex_lineage = Lineage::root(0, sig.clone());
        let face_lineage = Lineage::root(1, sig.clone());
        let he_lineage = Lineage::root(2, sig.clone());
        let edge_lineage = Lineage::root(3, sig.clone());

        let vertex = draft.insert_vertex(VertexData::with_lineage(
            placeholder_he,
            Some(vertex_lineage),
        ));

        let face = draft.insert_face(FaceData::with_lineage(
            placeholder_loop,
            self.shell,
            Some(face_lineage),
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        let edge = draft.insert_edge(EdgeData::with_lineage(placeholder_he, Some(edge_lineage)));

        let he = draft.insert_half_edge(HalfEdgeData::with_lineage(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            edge,
            Some(he_lineage),
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

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_face_vertex")
    }
}
