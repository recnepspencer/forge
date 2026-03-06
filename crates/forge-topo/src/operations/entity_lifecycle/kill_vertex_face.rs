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

use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::validators::invariant_id::InvariantContract;

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

    const INVARIANT_CONTRACT: InvariantContract =
        crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!(
            "Destroy isolated face {} and vertex {}",
            self.face.index(),
            self.vertex.index()
        )
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let op_name = Self::NAME.to_string();
        let inv_id = 0 as u64;

        // 1. Gather entities and validate isolation
        let (
            loop_id,
            he_id,
            edge_id,
            shell_id,
            region_id,
            lump_id,
            body_id,
            region_shell_count,
            lump_region_count,
            body_lump_count,
        ) = {
            let face_data = draft.arena().get_face(self.face)?;

            // Must have exactly one loop
            let loop_id = face_data.loops.outer();
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

            // Check how many children each container has to determine
            // how far up the hierarchy to cascade destruction.
            let region_shell_count = region_data.shell_count();
            let lump_region_count = lump_data.region_count();
            let body_lump_count = draft.arena().get_body(body_id)?.lump_count();

            (
                loop_id,
                he_id,
                edge_id,
                shell_id,
                region_id,
                lump_id,
                body_id,
                region_shell_count,
                lump_region_count,
                body_lump_count,
            )
        };

        // 2. Destroy the face and its immediate entities (always happens)
        draft.remove_face(self.face)?;
        draft.remove_vertex(self.vertex)?;
        draft.remove_half_edge(he_id)?;
        draft.remove_loop(loop_id)?;
        draft.remove_edge(edge_id)?;
        draft.remove_shell(shell_id)?;

        // 3. Cascade up the hierarchy only when each container has exactly 1 child.
        let mut delta_regions = 0i32;
        let mut delta_lumps = 0i32;
        let mut delta_solids = 0i32;

        if region_shell_count <= 1 {
            // Region has no other shells — safe to destroy
            draft
                .arena_mut()
                .get_lump_mut(lump_id)?
                .remove_region(region_id);
            draft.remove_region(region_id)?;
            delta_regions = -1;

            if lump_region_count <= 1 {
                // Lump has no other regions — safe to destroy
                draft
                    .arena_mut()
                    .get_body_mut(body_id)?
                    .remove_lump(lump_id);
                draft.remove_lump(lump_id)?;
                delta_lumps = -1;

                if body_lump_count <= 1 {
                    // Body has no other lumps — safe to destroy
                    draft.remove_body(body_id)?;
                    delta_solids = -1;
                }
            }
        } else {
            // Region has other shells — just unlink this shell, don't cascade
            draft
                .arena_mut()
                .get_region_mut(region_id)?
                .remove_shell(shell_id);
        }

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -1,
                faces: -1,
                loops: -1,
                edges: -1,
                shells: -1,
                solids: delta_solids,
                lumps: delta_lumps,
                regions: delta_regions,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::KillVertexFace;
    use crate::b_rep::ShellKind;
    use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::operations::entity_lifecycle::split_edge::SplitEdge;
    use crate::operator::TopoOperator;
    use crate::transactions::TopologyState;

    #[test]
    fn kill_vertex_face_destroys_isolated_seed() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        assert_eq!(draft.arena().face_count(), 1);
        assert_eq!(draft.arena().vertex_count(), 1);
        assert_eq!(draft.arena().half_edge_count(), 1);
        assert_eq!(draft.arena().loop_count(), 1);
        assert_eq!(draft.arena().edge_count(), 1);
        assert_eq!(draft.arena().shell_count(), 1);
        assert_eq!(draft.arena().body_count(), 1);

        draft
            .execute(KillVertexFace {
                face: mvf.face,
                vertex: mvf.vertex,
            })
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

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        // Split the edge to make it non-isolated
        draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap();

        let res = draft.execute(KillVertexFace {
            face: mvf.face,
            vertex: mvf.vertex,
        });
        assert!(res.is_err());
    }
}
