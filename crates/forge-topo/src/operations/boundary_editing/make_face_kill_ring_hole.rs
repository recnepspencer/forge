//! MakeFaceKillRingHole — promotes an inner loop (ring/hole) to its own face.
//!
//! DOMAIN: Takes an inner loop of an existing face and converts it into the
//! outer loop of a new, disjoint face within the same shell.
//!
//! INVARIANTS:
//! - ΔF=+1, ΔL=0 (loop moves, not created or destroyed)
//! - The loop must be an inner loop (hole) of its current face.
//! - The new face inherits lineage derived from the parent face.
//! - All halfedges on the promoted loop get their `face` pointer updated.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::b_rep::FaceData;
use crate::handles::{FaceId, LoopId};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::validators::invariant_id::InvariantContract;

/// Promotes an inner loop (hole) into the outer loop of a new face.
///
/// # Disjoint Shell Violation
/// This operator extracts the hole into an independent face but topologically
/// leaves it within the **same** `Shell` entity. Because a `Shell` is defined
/// as a single contiguous component, this operation temporarily puts the B-Rep
/// in an invalid "disjoint shell" state. The caller must subsequently use
/// `SplitShell` to move the newly created disjoint face into its own `Shell`.
#[derive(Debug)]
pub struct MakeFaceKillRingHole {
    /// The inner loop (hole) to promote.
    pub loop_id: LoopId,
}

/// Output of the MakeFaceKillRingHole operator.
pub struct MfkrhOutput {
    /// The newly created face whose outer loop is the promoted hole.
    pub new_face: FaceId,
}

impl TopoOperator for MakeFaceKillRingHole {
    type Output = MfkrhOutput;

    const NAME: &'static str = "make_face_kill_ring_hole";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!("Promote inner loop {} to its own face", self.loop_id.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let loop_data = draft.arena().get_loop(self.loop_id)?;
        let old_face = loop_data.face();
        let start_he = loop_data.half_edge();

        let old_face_data = draft.arena().get_face(old_face)?;
        if old_face_data.outer_loop() == self.loop_id {
            return Err(KernelError::InvalidInput {
                message: "MakeFaceKillRingHole requires an inner loop, not the outer loop"
                    .to_string(),
                context: None,
            });
        }

        let is_inner = old_face_data.inner_loops().contains(&self.loop_id);
        if !is_inner {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MakeFaceKillRingHole: loop {} is not an inner loop of face {}",
                    self.loop_id.index(),
                    old_face.index()
                ),
                context: None,
            });
        }

        let shell = old_face_data.shell();
        let new_face = draft.insert_face(FaceData::new(
            self.loop_id,
            shell,
        ));

        draft
            .arena_mut()
            .get_face_mut(old_face)?
            .remove_inner_loop(self.loop_id);

        draft
            .arena_mut()
            .get_loop_mut(self.loop_id)?
            .set_face(new_face);

        let bound = draft.arena().half_edge_count();
        let mut current = start_he;
        for step in 0..=bound {
            draft
                .arena_mut()
                .reassign_halfedge_face(current, new_face)?;
            current = draft.arena().get_half_edge(current)?.next();
            if current == start_he {
                break;
            }
            if step == bound {
                return Err(KernelError::InternalError {
                    message: "MakeFaceKillRingHole: loop traversal exceeded bound".to_string(),
                    context: None,
                });
            }
        }

        Ok(ExecutionResult {
            value: MfkrhOutput { new_face },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 1,
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
