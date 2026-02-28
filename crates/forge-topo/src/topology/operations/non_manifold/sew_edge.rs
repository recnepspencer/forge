//! SewEdge — close a boundary by gluing two halfedges together.
//!
//! DOMAIN: Takes two self-radial boundary halfedges that span the same
//! vertex pair (A \u2192 B and B \u2192 A) and glues them into radial twins.
//! Kills one Edge entity.
//!
//! INVARIANTS:
//! - \u0394V=0, \u0394HE=0, \u0394F=0, \u0394E=-1, \u0394L=0
//! - The two halfedges must currently be boundaries (radial_next == self).
//! - The two halfedges must form an antiparallel pair:
//!   `he_a.origin() == he_b.next().origin()` AND `he_b.origin() == he_a.next().origin()`.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};

use crate::handles::{EdgeId, HalfEdgeId};

use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::operator::TopoOperator;

/// Close a boundary by gluing two boundary halfedges together, removing an edge entity.
///
/// This is required to turn an open surface into a closed manifold.
#[derive(Debug)]
pub struct SewEdge {
    /// The first boundary halfedge. Wil absorb the edge entity.
    pub he_a: HalfEdgeId,
    /// The second boundary halfedge. Its edge entity will be removed.
    pub he_b: HalfEdgeId,
}

/// Output of the SewEdge operator.
#[derive(Debug)]
pub struct SewEdgeOutput {
    /// The surviving edge entity.
    pub edge: EdgeId,
    /// The edge entity that was removed.
    pub removed_edge: EdgeId,
}

impl TopoOperator for SewEdge {
    type Output = SewEdgeOutput;

    const NAME: &'static str = "sew_edge";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let op_name = Self::NAME.to_string();
        let inv_id = 0u64;

        // Validate inputs and extract necessary handles first
        let (edge_to_keep, edge_to_remove, face_a, face_b) = {
            let he_a_data = draft.arena().get_half_edge(self.he_a)?;
            let he_b_data = draft.arena().get_half_edge(self.he_b)?;

            // Validation 1: Both must be boundaries (radial_next == self)
            if he_a_data.radial_next() != self.he_a {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BoundaryEdgeInSolid {
                        halfedge_index: self.he_a.index(),
                        shell_index: he_a_data.face().index(),
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.clone(),
                            invocation_id: inv_id,
                        },
                        suggested_fixes: vec![],
                        detail: format!(
                            "SewEdge requires boundary halfedges, but he_a ({}) is already sewn.",
                            self.he_a.index()
                        ),
                    }),
                });
            }
            if he_b_data.radial_next() != self.he_b {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BoundaryEdgeInSolid {
                        halfedge_index: self.he_b.index(),
                        shell_index: he_b_data.face().index(),
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.clone(),
                            invocation_id: inv_id,
                        },
                        suggested_fixes: vec![],
                        detail: format!(
                            "SewEdge requires boundary halfedges, but he_b ({}) is already sewn.",
                            self.he_b.index()
                        ),
                    }),
                });
            }

            // Validation 2: Must be antiparallel (A->B and B->A)
            let he_a_next = draft.arena().get_half_edge(he_a_data.next())?;
            let he_b_next = draft.arena().get_half_edge(he_b_data.next())?;

            let v1 = he_a_data.origin();
            let v2 = he_a_next.origin();

            if he_b_data.origin() != v2 || he_b_next.origin() != v1 {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::OrientationInconsistency { face_index: he_b_data.face().index() },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name, invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("SewEdge requires antiparallel halfedges. he_a is {}->{}, but he_b is {}->{}", v1.index(), v2.index(), he_b_data.origin().index(), he_b_next.origin().index())
                    })
                });
            }

            (
                he_a_data.edge(),
                he_b_data.edge(),
                he_a_data.face(),
                he_b_data.face(),
            )
        };

        // 1. Sew the radial pointers
        draft
            .arena_mut()
            .get_half_edge_mut(self.he_a)?
            .set_radial_next(self.he_b);
        draft
            .arena_mut()
            .get_half_edge_mut(self.he_b)?
            .set_radial_next(self.he_a);

        // 2. Point he_b to the surviving edge
        draft
            .arena_mut()
            .get_half_edge_mut(self.he_b)?
            .set_edge(edge_to_keep);

        // 3. Remove the obsolete edge entity
        draft.remove_edge(edge_to_remove)?;

        // 4. Face version bumps
        draft.arena_mut().bump_face_version(face_a)?;
        draft.arena_mut().bump_face_version(face_b)?;

        Ok(ExecutionResult {
            value: SewEdgeOutput {
                edge: edge_to_keep,
                removed_edge: edge_to_remove,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: -1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
