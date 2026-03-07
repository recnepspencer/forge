use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone)]
pub struct SewEdgeMutation {
    pub half_edge_a: SpecNodeId,
    pub half_edge_b: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SewEdgeOutput {
    pub edge: SpecNodeId,
    pub removed_edge: SpecNodeId,
}

impl SpecMutation for SewEdgeMutation {
    type Output = SewEdgeOutput;

    const NAME: &'static str = "sew_edge";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if self.half_edge_a == self.half_edge_b {
            return Err(SpecError::invalid(
                "SewEdgeMutation cannot sew a halfedge to itself".to_string(),
            ));
        }
        if draft.node_kind(self.half_edge_a)? != SpecNodeKind::HalfEdge
            || draft.node_kind(self.half_edge_b)? != SpecNodeKind::HalfEdge
        {
            return Err(SpecError::invalid(
                "SewEdgeMutation requires halfedge inputs".to_string(),
            ));
        }

        let radial_a =
            draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeRadialNext)?;
        let radial_b =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeRadialNext)?;
        if radial_a != self.half_edge_a || radial_b != self.half_edge_b {
            return Err(SpecError::invalid(
                "SewEdgeMutation requires both halfedges to be boundary self-radial".to_string(),
            ));
        }

        let a_next = draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeNext)?;
        let b_next = draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeNext)?;
        let a_origin =
            draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeOriginVertex)?;
        let b_origin =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeOriginVertex)?;
        let a_dest = draft.single_outgoing_target(a_next, RelationKind::HalfEdgeOriginVertex)?;
        let b_dest = draft.single_outgoing_target(b_next, RelationKind::HalfEdgeOriginVertex)?;
        if a_origin != b_dest || b_origin != a_dest {
            return Err(SpecError::invalid(
                "SewEdgeMutation requires antiparallel halfedges".to_string(),
            ));
        }

        let edge = draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeUsesEdge)?;
        let removed_edge =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeUsesEdge)?;

        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            self.half_edge_a,
            self.half_edge_b,
            "sew-radial-a",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            self.half_edge_b,
            self.half_edge_a,
            "sew-radial-b",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeUsesEdge,
            self.half_edge_b,
            edge,
            "sew-shared-edge",
        )?;
        draft.delete_node(removed_edge)?;

        Ok(MutationResult {
            value: SewEdgeOutput { edge, removed_edge },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "sew boundary halfedges {} and {}",
                    self.half_edge_a, self.half_edge_b
                ),
                format!("remove redundant edge {}", removed_edge),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Sew boundary halfedges {} and {}",
            self.half_edge_a, self.half_edge_b
        )
    }
}
