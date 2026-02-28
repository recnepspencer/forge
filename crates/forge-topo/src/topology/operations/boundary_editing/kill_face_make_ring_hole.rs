//! KillFaceMakeRingHole — demotes an independent face into a hole (inner loop).
//!
//! DOMAIN: Takes a face whose outer loop is disjoint and converts it into an
//! inner loop (hole) of another existing face in the same shell.
//!
//! INVARIANTS:
//! - ΔF=-1, ΔL=0 (loop moves, not created or destroyed)
//! - Both faces must belong to the same shell.
//! - The face to kill must have no inner loops of its own.
//! - All halfedges on the demoted loop get their `face` pointer updated.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::handles::FaceId;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::operator::TopoOperator;

/// Demotes a face's outer loop into an inner loop (hole) of another face.
#[derive(Debug)]
pub struct KillFaceMakeRingHole {
    /// The face to destroy (its outer loop becomes a hole).
    pub face_to_kill: FaceId,
    /// The face that will receive the hole.
    pub target_face: FaceId,
}

/// Output of the KillFaceMakeRingHole operator.
pub struct KfmrhOutput {}

impl TopoOperator for KillFaceMakeRingHole {
    type Output = KfmrhOutput;

    const NAME: &'static str = "kill_face_make_ring_hole";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let killed_face_data = draft.arena().get_face(self.face_to_kill)?;
        let target_face_data = draft.arena().get_face(self.target_face)?;

        if killed_face_data.shell() != target_face_data.shell() {
            return Err(KernelError::InvalidInput {
                message: "KillFaceMakeRingHole requires faces to be in the same shell".to_string(),
                context: None,
            });
        }

        if !killed_face_data.inner_loops().is_empty() {
            return Err(KernelError::InvalidInput {
                message: "KillFaceMakeRingHole: face to kill must have no inner loops".to_string(),
                context: None,
            });
        }

        let loop_id = killed_face_data.outer_loop();
        let start_he = draft.arena().get_loop(loop_id)?.half_edge();

        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_face(self.target_face);

        draft
            .arena_mut()
            .get_face_mut(self.target_face)?
            .add_inner_loop(loop_id);

        let bound = draft.arena().half_edge_count();
        let mut current = start_he;
        for step in 0..=bound {
            draft
                .arena_mut()
                .reassign_halfedge_face(current, self.target_face)?;
            current = draft.arena().get_half_edge(current)?.next();
            if current == start_he {
                break;
            }
            if step == bound {
                return Err(KernelError::InternalError {
                    message: "KillFaceMakeRingHole: loop traversal exceeded bound".to_string(),
                    context: None,
                });
            }
        }

        draft.remove_face(self.face_to_kill)?;

        Ok(ExecutionResult {
            value: KfmrhOutput {},
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: -1,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
