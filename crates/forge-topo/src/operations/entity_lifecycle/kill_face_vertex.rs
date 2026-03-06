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
use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::validators::invariant_id::InvariantContract;

/// Removes a disjoint face and its single vertex from a shell.
#[derive(Debug)]
pub struct KillFaceVertex {
    /// The face to remove.
    pub face: FaceId,
}

/// Output of the KillFaceVertex operator.
pub struct KfvOutput {}

impl TopoOperator for KillFaceVertex {
    type Output = KfvOutput;

    const NAME: &'static str = "kill_face_vertex";

    const INVARIANT_CONTRACT: InvariantContract =
        crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!("Destroy face {} and its isolated vertex", self.face.index())
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let face_data = draft.arena().get_face(self.face)?;
        let loop_id = face_data.loops.outer();

        if !face_data.loops.inners().is_empty() {
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
                message: "KillFaceVertex: face must contain exactly one self-loop halfedge"
                    .to_string(),
                context: None,
            });
        }

        if he_data.radial_next() != he_id {
            return Err(KernelError::InvalidInput {
                message: "KillFaceVertex: halfedge must be self-radial (boundary)".to_string(),
                context: None,
            });
        }

        // Capture the parent shell before removing the face.
        let shell_id = draft.arena().get_face(self.face)?.shell();

        draft.remove_half_edge(he_id)?;
        draft.remove_edge(edge_id)?;
        draft.remove_loop(loop_id)?;
        draft.remove_vertex(vertex_id)?;
        draft.remove_face(self.face)?;

        // Update shell representative_face if we just removed it.
        let shell_data = draft.arena().get_shell(shell_id)?;
        if shell_data.representative_face() == self.face {
            let remaining = draft.arena().faces_of_shell(shell_id);
            if let Some(&new_repr) = remaining.first() {
                draft
                    .arena_mut()
                    .get_shell_mut(shell_id)?
                    .set_representative_face(new_repr);
            } else {
                // Shell is now empty — the caller should use KillShellFace
                // or KillVertexFace to properly tear down the hierarchy.
                return Err(KernelError::InvalidInput {
                    message:
                        "KillFaceVertex left shell empty. Use KillShellFace to destroy the shell."
                            .to_string(),
                    context: None,
                });
            }
        }

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
}
