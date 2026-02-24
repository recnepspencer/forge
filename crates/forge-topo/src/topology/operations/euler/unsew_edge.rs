//! UnsewEdge — open a boundary by ungluing two halfedges.
//!
//! DOMAIN: Takes an edge that joins two faces together (via two halfedges
//! A -> B and B -> A) and separates them so that each halfedge becomes
//! its own boundary (radial_next == self). Creates one new Edge entity.
//!
//! INVARIANTS:
//! - ΔV=0, ΔHE=0, ΔF=0, ΔE=+1, ΔL=0
//! - The two halfedges must currently be sewn together (radial_next == each other).
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{KernelError, ErrorContext, ErrorScope, TopologyError};

use crate::handles::{HalfEdgeId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{ExecutionResult, EulerDelta};
use crate::EulerOperator;
use crate::state::MutableDraft;
use crate::arena::EdgeData;

/// Open a boundary by ungluing two halfedges, creating a new edge entity.
///
/// This is the exact inverse of SewEdge.
#[derive(Debug)]
pub struct UnsewEdge {
    /// One of the halfedges to unsew.
    pub he_a: HalfEdgeId,
    /// The other halfedge to unsew. This one will receive the new edge.
    pub he_b: HalfEdgeId,
}

/// Output of the UnsewEdge operator.
#[derive(Debug)]
pub struct UnsewEdgeOutput {
    /// The newly created edge entity (assigned to he_b).
    pub new_edge: EdgeId,
    /// The original edge entity (kept by he_a).
    pub original_edge: EdgeId,
}

impl EulerOperator for UnsewEdge {
    type Output = UnsewEdgeOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let op_name = self.signature().get_name().to_string();
        let inv_id = sig.get_invocation_id() as u64;

        // Validate inputs and extract necessary handles first
        let (original_edge, face_a, face_b) = {
            let he_a_data = draft.arena().get_half_edge(self.he_a)?;
            let he_b_data = draft.arena().get_half_edge(self.he_b)?;

            // Validation 1: Both must be sewn to each other
            if he_a_data.radial_next() != self.he_b || he_b_data.radial_next() != self.he_a {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BoundaryEdgeInSolid { halfedge_index: self.he_a.index(), shell_index: he_a_data.face().index() }, // borrowing this error kind for now
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation { op_name: op_name.clone(), invocation_id: inv_id },
                        suggested_fixes: vec![],
                        detail: format!("UnsewEdge requires two halfedges sewn together, but {} and {} are not radial twins.", self.he_a.index(), self.he_b.index())
                    })
                });
            }

            (he_a_data.edge(), he_a_data.face(), he_b_data.face())
        };

        let original_edge_lineage = draft.arena().get_edge(original_edge)?.lineage().cloned();
        let edge_lineage = Lineage::derive_from(&original_edge_lineage, sig.clone());
        let new_edge = draft.insert_edge(EdgeData::with_lineage(
            self.he_b,
            Some(edge_lineage),
        ));

        // 1. Unsew the radial pointers (they become their own twins = boundaries)
        draft.arena_mut().get_half_edge_mut(self.he_a)?.set_radial_next(self.he_a);
        draft.arena_mut().get_half_edge_mut(self.he_b)?.set_radial_next(self.he_b);

        // 2. Point he_b to the new edge
        draft.arena_mut().get_half_edge_mut(self.he_b)?.set_edge(new_edge);

        // 3. Face version bumps
        draft.arena_mut().bump_face_version(face_a)?;
        draft.arena_mut().bump_face_version(face_b)?;

        Ok(ExecutionResult {
            value: UnsewEdgeOutput {
                new_edge,
                original_edge,
            },
            declared_delta: EulerDelta { vertices: 0, half_edges: 0, faces: 0, loops: 0, edges: 1, shells: 0, solids: 0, lumps: 0, regions: 0 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("unsew_edge")
    }
}

#[cfg(test)]
mod tests {
    use crate::EulerOperator;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::topology::operations::euler::make_vertex_face::MakeVertexFace;
    use crate::topology::operations::euler::split_edge::SplitEdge;
    use crate::topology::operations::euler::sew_edge::SewEdge;
    use super::UnsewEdge;

    #[test]
    fn unsew_edge_separates_glued_boundaries() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        
        let he_v0_v1 = mvf.half_edge;
        let he_v1_v0 = se.he_mb;
        
        // Sew them first
        let sew_res = apply_op(&mut draft, SewEdge { he_a: he_v0_v1, he_b: he_v1_v0 }).unwrap().into_value();
        assert_eq!(draft.arena().edge_count(), 1);

        // Now unsew
        let unsew_res = apply_op(&mut draft, UnsewEdge { he_a: he_v0_v1, he_b: he_v1_v0 }).unwrap().into_value();
        
        // Assert final state
        let he_a_data = draft.arena().get_half_edge(he_v0_v1).unwrap();
        let he_b_data = draft.arena().get_half_edge(he_v1_v0).unwrap();
        
        // 1. Radial pointers point to themselves
        assert_eq!(he_a_data.radial_next(), he_v0_v1);
        assert_eq!(he_b_data.radial_next(), he_v1_v0);
        
        // 2. They do not share the same Edge entity anymore
        assert_ne!(he_a_data.edge(), he_b_data.edge());
        assert_eq!(he_a_data.edge(), unsew_res.original_edge);
        assert_eq!(he_b_data.edge(), unsew_res.new_edge);
        assert_eq!(unsew_res.original_edge, sew_res.edge);
        
        // 3. New edge entity was created
        assert_eq!(draft.arena().edge_count(), 2);
    }

    #[test]
    fn unsew_edge_fails_on_unsewn_edges() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        
        let he_v0_v1 = mvf.half_edge;
        let he_v1_v0 = se.he_mb;

        // Try unsewing them before they are sewn
        let res = apply_op(&mut draft, UnsewEdge { he_a: he_v0_v1, he_b: he_v1_v0 });
        assert!(matches!(
            res.unwrap_err(),
            forge_core::KernelError::TopologyViolation { err: forge_core::TopologyError::BoundaryEdgeInSolid { .. }, .. }
        ));
    }
}
