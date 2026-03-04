//! MakeLoopInFaceFromVertices — build an inner loop on an existing face.
//!
//! DOMAIN: Create a disconnected boundary loop (typically a hole loop)
//! on an existing face by connecting a pre-existing ordered sequence
//! of vertices.
//!
//! INVARIANTS:
//! - The face must exist in the arena.
//! - The vertices must exist in the arena.
//! - Creates exactly N half-edges, N edges, 1 loop.
//! - Adds the loop to `face.inner_loops`.
//! - Does NOT create a new face/shell/region/lump/body.

use forge_core::KernelError;

use crate::b_rep::{EdgeData, HalfEdgeData, LoopData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;


/// Creates a new inner loop on an existing face by connecting a sequence of vertices.
#[derive(Debug)]
pub struct MakeLoopInFaceFromVertices {
    /// The face that will own the new inner loop.
    pub face: FaceId,
    /// Ordered list of existing vertices to connect into a closed loop.
    pub vertices: Vec<VertexId>,
}

/// Output of the MakeLoopInFaceFromVertices operator.
pub struct MlifvOutput {
    /// The created loop.
    pub loop_id: LoopId,
    /// The created halfedges (in same order as vertices).
    pub half_edges: Vec<HalfEdgeId>,
    /// The created edges (in same order as halfedges).
    pub edges: Vec<EdgeId>,
}

impl TopoOperator for MakeLoopInFaceFromVertices {
    type Output = MlifvOutput;

    const NAME: &'static str = "make_loop_in_face_from_vertices";

    const INVARIANT_CONTRACT: InvariantContract = crate::conservative_contract!();

    fn semantic_summary(&self) -> String {
        format!(
            "Create {}-sided inner loop (hole) in face {}",
            self.vertices.len(), self.face.index()
        )
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let n = self.vertices.len();
        if n < 3 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MakeLoopInFaceFromVertices: at least 3 vertices required, got {}",
                    n
                ),
                context: None,
            });
        }

        if draft.arena().get_face(self.face).is_err() {
            return Err(KernelError::InvalidInput {
                message: format!("MakeLoopInFaceFromVertices: face {} not found", self.face),
                context: None,
            });
        }

        for &v in &self.vertices {
            if draft.arena().get_vertex(v).is_err() {
                return Err(KernelError::InvalidInput {
                    message: format!("MakeLoopInFaceFromVertices: vertex {} not found", v),
                    context: None,
                });
            }
        }

        let placeholder_he = HalfEdgeId::DANGLING;
        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, self.face));
        draft
            .arena_mut()
            .get_face_mut(self.face)?
            .add_inner_loop(loop_id);

        let mut half_edges = Vec::with_capacity(n);
        let mut edges = Vec::with_capacity(n);

        for _ in 0..n {


            let edge =
                draft.insert_edge(EdgeData::new(placeholder_he));
            let he = draft.insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                self.face,
                VertexId::DANGLING,
                edge,
            ));

            draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);
            half_edges.push(he);
            edges.push(edge);
        }

        for i in 0..n {
            let next_i = (i + 1) % n;
            let prev_i = if i == 0 { n - 1 } else { i - 1 };

            let he = half_edges[i];
            let next_he = half_edges[next_i];
            let prev_he = half_edges[prev_i];
            let v = self.vertices[i];

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he)?.set_origin(v);
            arena.get_half_edge_mut(he)?.set_radial_next(he);
            arena.get_half_edge_mut(he)?.set_next(next_he);
            arena.get_half_edge_mut(he)?.set_prev(prev_he);

            let orig_out = arena.get_vertex(v)?.outgoing();
            if orig_out == HalfEdgeId::DANGLING {
                arena.get_vertex_mut(v)?.set_outgoing(he);
            }
        }

        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(half_edges[0]);

        Ok(ExecutionResult {
            value: MlifvOutput {
                loop_id,
                half_edges,
                edges,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: n as i32,
                faces: 0,
                loops: 1,
                edges: n as i32,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
