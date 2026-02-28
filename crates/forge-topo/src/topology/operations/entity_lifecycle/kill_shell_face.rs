//! KillShellFace — destroys a disconnected shell within an existing solid.
//!
//! DOMAIN: Takes a FaceId and destroys its isolated topological seed
//! topology (Face, Vertex, Loop, HalfEdge, Edge, Shell). This is the exact
//! inverse of `MakeShellFace`.
//!
//! INVARIANTS:
//! - ΔV=-1, ΔHE=-1, ΔF=-1, ΔL=-1, ΔE=-1, ΔS=-1, ΔSo=0
//! - The face must belong to an isolated shell (only 1 face total).
//! - The solid must survive (its count of shells decreases by 1).
//! - The face must have exactly one loop.
//! - That loop must have exactly one halfedge.
//! - That halfedge must be its own next, prev, and radial twin.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};

use crate::handles::{FaceId, VertexId};
use crate::lineage::OpSignature;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Destroys a disconnected shell within an existing solid.
#[derive(Debug)]
pub struct KillShellFace {
    /// The isolated face pointing to the shell to destroy.
    pub face: FaceId,
    /// The isolated vertex to destroy alongside the face.
    pub vertex: VertexId,
}

impl EulerOperator for KillShellFace {
    type Output = ();

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let op_name = self.signature().get_name().to_string();
        let inv_id = sig.get_invocation_id() as u64;

        // 1. Gather entities and validate isolation
        let (loop_id, he_id, edge_id, shell_id, region_id) = {
            let face_data = draft.arena().get_face(self.face)?;

            // Must have exactly one loop
            let loop_id = face_data.outer_loop();
            let loop_data = draft.arena().get_loop(loop_id)?;

            // Must have exactly one halfedge
            let he_id = loop_data.half_edge();
            let he_data = draft.arena().get_half_edge(he_id)?;

            if he_data.next() != he_id || he_data.prev() != he_id {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::InvalidOperation { detail: "Face is not isolated (has multiple halfedges)".to_string() },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name: op_name.clone(), invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("KillShellFace requires an isolated seed, but face {} has a loop with multiple halfedges.", self.face.index())
                    })
                });
            }

            if he_data.radial_next() != he_id {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::InvalidOperation { detail: "Face is not isolated (halfedge is sewn)".to_string() },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name: op_name.clone(), invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("KillShellFace requires an isolated seed, but face {} is sewn to another face.", self.face.index())
                    })
                });
            }

            if he_data.origin() != self.vertex {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::InvalidOperation {
                        detail: "Vertex mismatch".to_string(),
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.clone(),
                            invocation_id: inv_id,
                        },
                        suggested_fixes: vec![],
                        detail: format!(
                            "KillShellFace: Provided vertex {} does not match face's vertex {}.",
                            self.vertex.index(),
                            he_data.origin().index()
                        ),
                    }),
                });
            }

            let edge_id = he_data.edge();
            let shell_id = face_data.shell();
            let shell_data = draft.arena().get_shell(shell_id)?;
            let region_id = shell_data.region();

            (loop_id, he_id, edge_id, shell_id, region_id)
        };

        // 2. Unlink the shell from the region
        let region_data = draft.arena_mut().get_region_mut(region_id)?;
        if !region_data.remove_shell(shell_id) {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::InvalidOperation {
                    detail: "Shell not found in region".to_string(),
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Operation {
                        op_name: op_name.clone(),
                        invocation_id: inv_id,
                    },
                    suggested_fixes: vec![],
                    detail: format!(
                        "KillShellFace: Shell {} was not found in parent Region {}.",
                        shell_id.index(),
                        region_id.index()
                    ),
                }),
            });
        }

        // 3. Destroy everything except the solid
        draft.remove_face(self.face)?;
        draft.remove_vertex(self.vertex)?;
        draft.remove_half_edge(he_id)?;
        draft.remove_loop(loop_id)?;
        draft.remove_edge(edge_id)?;
        draft.remove_shell(shell_id)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -1,
                faces: -1,
                loops: -1,
                edges: -1,
                shells: -1,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_shell_face")
    }
}

#[cfg(test)]
mod tests {
    use super::KillShellFace;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::topology::operations::euler::make_shell_face::MakeShellFace;
    use crate::topology::operations::euler::make_vertex_face::MakeVertexFace;
    use crate::EulerOperator;

    #[test]
    fn kill_shell_face_destroys_isolated_shell() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let region = draft.arena().get_shell(mvf.shell).unwrap().region();
        let msf = apply_op(&mut draft, MakeShellFace { region })
            .unwrap()
            .into_value();

        assert_eq!(draft.arena().face_count(), 2);
        assert_eq!(draft.arena().vertex_count(), 2);
        assert_eq!(draft.arena().half_edge_count(), 2);
        assert_eq!(draft.arena().loop_count(), 2);
        assert_eq!(draft.arena().edge_count(), 2);
        assert_eq!(draft.arena().shell_count(), 2);
        assert_eq!(draft.arena().body_count(), 1);

        let region_data = draft.arena().get_region(region).unwrap();
        assert_eq!(region_data.shell_count(), 2);

        apply_op(
            &mut draft,
            KillShellFace {
                face: msf.face,
                vertex: msf.vertex,
            },
        )
        .unwrap();

        assert_eq!(draft.arena().face_count(), 1);
        assert_eq!(draft.arena().vertex_count(), 1);
        assert_eq!(draft.arena().half_edge_count(), 1);
        assert_eq!(draft.arena().loop_count(), 1);
        assert_eq!(draft.arena().edge_count(), 1);
        assert_eq!(draft.arena().shell_count(), 1);
        assert_eq!(draft.arena().body_count(), 1);

        let region_data = draft.arena().get_region(region).unwrap();
        assert_eq!(region_data.shell_count(), 1);
        assert!(region_data.shells().contains(&mvf.shell));
        assert!(!region_data.shells().contains(&msf.shell));
    }
}
