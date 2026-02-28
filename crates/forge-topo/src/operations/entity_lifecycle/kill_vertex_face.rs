//! KillVertexFace — destroys an isolated topological seed.
//!
//! DOMAIN: The exact inverse of `MakeVertexFace`. Destroys a face, its single
//! vertex, its single loop, its degenerate boundary halfedge, its shell, and
//! its edge. Also destroys the parent Solid, if we assume KVF is the absolute
//! final tear-down.
//!
//! Actually, MakeVertexFace creates: Vertex, Face, Loop, HalfEdge, Edge, Shell, Solid.
//! KillVertexFace must destroy all of those.
//! KVF is only valid on an isolated seed.
//!
//! INVARIANTS:
//! - ΔV=-1, ΔHE=-1, ΔF=-1, ΔL=-1, ΔE=-1, ΔS=-1, ΔSo=-1
//! - The face must have exactly one loop.
//! - That loop must have exactly one halfedge.
//! - That halfedge must be its own next, prev, and radial twin.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};

use crate::handles::{FaceId, VertexId};

use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;

/// Destroys an isolated topological seed.
///
/// This is the exact inverse of MakeVertexFace.
#[derive(Debug)]
pub struct KillVertexFace {
    /// The isolated face to destroy.
    pub face: FaceId,
    /// The isolated vertex to destroy.
    pub vertex: VertexId,
}

impl TopoOperator for KillVertexFace {
    type Output = ();

    const NAME: &'static str = "kill_vertex_face";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let op_name = Self::NAME.to_string();
        let inv_id = 0 as u64;

        // 1. Gather entities and validate isolation
        let (loop_id, he_id, edge_id, shell_id, region_id, lump_id, body_id) = {
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
                        detail: format!("KillVertexFace requires an isolated seed, but face {} has a loop with multiple halfedges.", self.face.index())
                    })
                });
            }

            if he_data.radial_next() != he_id {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::InvalidOperation { detail: "Face is not isolated (halfedge is sewn)".to_string() },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name: op_name.clone(), invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("KillVertexFace requires an isolated seed, but face {} is sewn to another face.", self.face.index())
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
                            "KillVertexFace: Provided vertex {} does not match face's vertex {}.",
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
            let region_data = draft.arena().get_region(region_id)?;
            let lump_id = region_data.lump();
            let lump_data = draft.arena().get_lump(lump_id)?;
            let body_id = lump_data.body();

            let solid_data = draft.arena().get_body(body_id)?;
            if solid_data.lump_count() > 1 {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::InvalidOperation { detail: "Solid has multiple lumps".to_string() },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name: op_name.clone(), invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("KillVertexFace: Solid {} contains multiple lumps, so destroying the solid is invalid.", body_id.index())
                    })
                });
            }

            (
                loop_id, he_id, edge_id, shell_id, region_id, lump_id, body_id,
            )
        };

        // 2. Destroy everything
        draft.remove_face(self.face)?;
        draft.remove_vertex(self.vertex)?;
        draft.remove_half_edge(he_id)?;
        draft.remove_loop(loop_id)?;
        draft.remove_edge(edge_id)?;
        draft.remove_shell(shell_id)?;
        draft.remove_region(region_id)?;
        draft.remove_lump(lump_id)?;
        draft.remove_body(body_id)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -1,
                faces: -1,
                loops: -1,
                edges: -1,
                shells: -1,
                solids: -1,
                lumps: -1,
                regions: -1,
            },
        })
    }


}

#[cfg(test)]
mod tests {
    use super::KillVertexFace;
    use crate::transactions::TopologyState;
    use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::operations::entity_lifecycle::split_edge::SplitEdge;
    use crate::operator::TopoOperator;

    #[test]
    fn kill_vertex_face_destroys_isolated_seed() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();

        assert_eq!(draft.arena().face_count(), 1);
        assert_eq!(draft.arena().vertex_count(), 1);
        assert_eq!(draft.arena().half_edge_count(), 1);
        assert_eq!(draft.arena().loop_count(), 1);
        assert_eq!(draft.arena().edge_count(), 1);
        assert_eq!(draft.arena().shell_count(), 1);
        assert_eq!(draft.arena().body_count(), 1);

        draft.execute(
            KillVertexFace {
                face: mvf.face,
                vertex: mvf.vertex,
            },
        )
        .unwrap();

        // Everything should be gone
        assert_eq!(draft.arena().face_count(), 0);
        assert_eq!(draft.arena().vertex_count(), 0);
        assert_eq!(draft.arena().half_edge_count(), 0);
        assert_eq!(draft.arena().loop_count(), 0);
        assert_eq!(draft.arena().edge_count(), 0);
        assert_eq!(draft.arena().shell_count(), 0);
        assert_eq!(draft.arena().body_count(), 0);
    }

    #[test]
    fn kill_vertex_face_fails_on_non_isolated_face() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();

        // Split the edge to make it non-isolated
        draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap();

        let res = draft.execute(
            KillVertexFace {
                face: mvf.face,
                vertex: mvf.vertex,
            },
        );
        assert!(res.is_err());
    }
}
