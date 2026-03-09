use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Collapse the restricted wire edge created by `MakeEdgeVertexMutation`.
///
/// This implementation is intentionally narrow. It only supports the currently
/// proven migration case:
/// - one wire edge pair on a single face
/// - the tip vertex is used by exactly one halfedge
/// - the halfedge sequence is `prev -> he_out -> he_back -> anchor`
pub struct KillEdgeVertexMutation {
    pub half_edge: SpecNodeId,
}

impl std::fmt::Debug for KillEdgeVertexMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KillEdgeVertexMutation")
            .field("half_edge", &self.half_edge)
            .finish()
    }
}

impl SpecMutation for KillEdgeVertexMutation {
    type Output = ();

    const NAME: &'static str = "kill_edge_vertex";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.half_edge)? != SpecNodeKind::HalfEdge {
            return Err(SpecError::invalid(format!(
                "KillEdgeVertexMutation requires HalfEdge input, got {:?}",
                draft.node_kind(self.half_edge)?
            )));
        }

        let he_out = self.half_edge;
        let he_back = draft.single_outgoing_target(he_out, RelationKind::HalfEdgeRadialNext)?;
        let radial_back =
            draft.single_outgoing_target(he_back, RelationKind::HalfEdgeRadialNext)?;
        if radial_back != he_out {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires a two-halfedge wire radial pair".to_string(),
            ));
        }

        let edge = draft.single_outgoing_target(he_out, RelationKind::HalfEdgeUsesEdge)?;
        if draft.single_outgoing_target(he_back, RelationKind::HalfEdgeUsesEdge)? != edge {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires both radial halfedges to use the same edge"
                    .to_string(),
            ));
        }

        let face = draft.single_outgoing_target(he_out, RelationKind::HalfEdgeBoundsFace)?;
        if draft.single_outgoing_target(he_back, RelationKind::HalfEdgeBoundsFace)? != face {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires a single-face wire edge".to_string(),
            ));
        }

        let surviving_vertex =
            draft.single_outgoing_target(he_out, RelationKind::HalfEdgeOriginVertex)?;
        let removed_vertex =
            draft.single_outgoing_target(he_back, RelationKind::HalfEdgeOriginVertex)?;
        if draft
            .incoming_sources_of_kind(removed_vertex, RelationKind::HalfEdgeOriginVertex)
            .len()
            != 1
        {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires the tip vertex to be used by exactly one halfedge"
                    .to_string(),
            ));
        }

        let prev = draft.single_incoming_source(he_out, RelationKind::HalfEdgeNext)?;
        let next_from_out = draft.single_outgoing_target(he_out, RelationKind::HalfEdgeNext)?;
        let anchor = draft.single_outgoing_target(he_back, RelationKind::HalfEdgeNext)?;
        if next_from_out != he_back {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires contiguous halfedge order prev -> out -> back"
                    .to_string(),
            ));
        }
        if draft.single_incoming_source(he_back, RelationKind::HalfEdgeNext)? != he_out {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires he_back to be entered only from he_out"
                    .to_string(),
            ));
        }
        if draft.single_incoming_source(anchor, RelationKind::HalfEdgeNext)? != he_back {
            return Err(SpecError::invalid(
                "KillEdgeVertexMutation requires the wire pair to sit directly before anchor"
                    .to_string(),
            ));
        }

        draft.replace_single_relation(RelationKind::HalfEdgeNext, prev, anchor, "collapse-next")?;

        draft.remove_relation_between(RelationKind::HalfEdgeNext, he_out, he_back)?;
        draft.remove_relation_between(RelationKind::HalfEdgeNext, he_back, anchor)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, he_out, he_back)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, he_back, he_out)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, he_out, edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, he_back, edge)?;
        draft.remove_relation_between(
            RelationKind::HalfEdgeOriginVertex,
            he_out,
            surviving_vertex,
        )?;
        draft.remove_relation_between(
            RelationKind::HalfEdgeOriginVertex,
            he_back,
            removed_vertex,
        )?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, he_out, face)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, he_back, face)?;

        draft.delete_node(he_out)?;
        draft.delete_node(he_back)?;
        draft.delete_node(edge)?;
        draft.delete_node(removed_vertex)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "collapse restricted wire edge at halfedge {}",
                    self.half_edge
                ),
                "remove wire edge pair and tip vertex, restoring predecessor directly to anchor"
                    .to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Collapse restricted wire edge at halfedge {}",
            self.half_edge
        )
    }
}
