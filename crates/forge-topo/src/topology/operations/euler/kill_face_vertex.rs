//! KillFaceVertex — inverse of MakeFaceVertex.
//!
//! DOMAIN: Removes a disjoint face, its single vertex, loop, self-loop
//! halfedge, and edge from a shell.
//!
//! INVARIANTS:
//! - ΔV=-1, ΔHE=-1, ΔF=-1, ΔL=-1, ΔE=-1
//! - The face must have exactly one loop containing exactly one self-loop halfedge.
//! - The halfedge must be self-radial, self-next, and self-prev.
//! - Does NOT destroy the parent shell/solid — only the face and its entities.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::handles::FaceId;
use crate::lineage::OpSignature;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Removes a disjoint face and its single vertex from a shell.
#[derive(Debug)]
pub struct KillFaceVertex {
    /// The face to remove.
    pub face: FaceId,
}

/// Output of the KillFaceVertex operator.
pub struct KfvOutput {}

impl EulerOperator for KillFaceVertex {
    type Output = KfvOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let face_data = draft.arena().get_face(self.face)?;
        let loop_id = face_data.outer_loop();

        if !face_data.inner_loops().is_empty() {
            return Err(KernelError::InvalidInput {
                message: "KillFaceVertex: face must have no inner loops".to_string(),
                context: None,
            });
        }

        let loop_data = draft.arena().get_loop(loop_id)?;
        let he_id = loop_data.half_edge();

        let he_data = draft.arena().get_half_edge(he_id)?;
        let vertex_id = he_data.origin();
        let edge_id = he_data.edge();

        if he_data.next() != he_id || he_data.prev() != he_id {
            return Err(KernelError::InvalidInput {
                message:
                    "KillFaceVertex: face must contain exactly one self-loop halfedge".to_string(),
                context: None,
            });
        }

        if he_data.radial_next() != he_id {
            return Err(KernelError::InvalidInput {
                message: "KillFaceVertex: halfedge must be self-radial (boundary)".to_string(),
                context: None,
            });
        }

        draft.remove_half_edge(he_id)?;
        draft.remove_edge(edge_id)?;
        draft.remove_loop(loop_id)?;
        draft.remove_vertex(vertex_id)?;
        draft.remove_face(self.face)?;

        Ok(ExecutionResult {
            value: KfvOutput {},
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -1,
                faces: -1,
                loops: -1,
                edges: -1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_face_vertex")
    }
}
