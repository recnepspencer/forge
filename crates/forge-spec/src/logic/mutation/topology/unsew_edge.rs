use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::radial_traversal::{collect_radial_ring, find_previous_radial};

#[derive(Debug, Clone)]
pub struct UnsewEdgeMutation {
    pub half_edge_a: SpecNodeId,
    pub half_edge_b: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsewEdgeOutput {
    pub new_edge: SpecNodeId,
    pub original_edge: SpecNodeId,
}

impl SpecMutation for UnsewEdgeMutation {
    type Output = UnsewEdgeOutput;

    const NAME: &'static str = "unsew_edge";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if self.half_edge_a == self.half_edge_b {
            return Err(SpecError::invalid(
                "UnsewEdgeMutation requires two distinct halfedges".to_string(),
            ));
        }
        if draft.node_kind(self.half_edge_a)? != SpecNodeKind::HalfEdge
            || draft.node_kind(self.half_edge_b)? != SpecNodeKind::HalfEdge
        {
            return Err(SpecError::invalid(
                "UnsewEdgeMutation requires halfedge inputs".to_string(),
            ));
        }

        let original_edge =
            draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeUsesEdge)?;
        if draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeUsesEdge)?
            != original_edge
        {
            return Err(SpecError::invalid(
                "UnsewEdgeMutation requires both halfedges to share the same edge".to_string(),
            ));
        }

        let ring = collect_radial_ring(draft, self.half_edge_a)?;
        if !ring
            .iter()
            .copied()
            .any(|candidate| candidate == self.half_edge_b)
        {
            return Err(SpecError::invalid(
                "UnsewEdgeMutation requires half_edge_b to be present in the radial ring of half_edge_a"
                    .to_string(),
            ));
        }
        let previous = find_previous_radial(draft, self.half_edge_b)?;
        let next_radial =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeRadialNext)?;
        if previous == self.half_edge_b {
            return Err(SpecError::invalid(
                "UnsewEdgeMutation cannot unsew an already boundary self-radial halfedge"
                    .to_string(),
            ));
        }

        let new_edge = draft.create_node(SpecNodeKind::Edge, None, "unsewn-edge")?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            previous,
            next_radial,
            "unsew-prev-radial",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            self.half_edge_b,
            self.half_edge_b,
            "unsew-radial-b",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeUsesEdge,
            self.half_edge_b,
            new_edge,
            "unsew-new-edge",
        )?;

        Ok(MutationResult {
            value: UnsewEdgeOutput {
                new_edge,
                original_edge,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "unsew halfedges {} and {}",
                    self.half_edge_a, self.half_edge_b
                ),
                format!(
                    "create new edge {} for halfedge {}",
                    new_edge, self.half_edge_b
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Unsew halfedges {} and {}",
            self.half_edge_a, self.half_edge_b
        )
    }
}
