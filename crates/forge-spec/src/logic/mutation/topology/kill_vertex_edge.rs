use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Collapse the midpoint vertex created by the restricted `SplitEdgeMutation`.
///
/// This implementation only supports the currently proven seed/self-loop split:
/// - the removed vertex originates exactly one halfedge
/// - that halfedge points directly back to the original halfedge
/// - both halfedges are self-radial
pub struct KillVertexEdgeMutation {
    pub vertex: SpecNodeId,
}

impl std::fmt::Debug for KillVertexEdgeMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KillVertexEdgeMutation")
            .field("vertex", &self.vertex)
            .finish()
    }
}

impl SpecMutation for KillVertexEdgeMutation {
    type Output = ();

    const NAME: &'static str = "kill_vertex_edge";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.vertex)? != SpecNodeKind::Vertex {
            return Err(SpecError::invalid(format!(
                "KillVertexEdgeMutation requires Vertex input, got {:?}",
                draft.node_kind(self.vertex)?
            )));
        }

        let outgoing =
            draft.incoming_sources_of_kind(self.vertex, RelationKind::HalfEdgeOriginVertex);
        if outgoing.len() != 1 {
            return Err(SpecError::invalid(
                "KillVertexEdgeMutation requires a vertex used by exactly one halfedge".to_string(),
            ));
        }

        let he_mb = outgoing[0];
        let he_am = draft.single_outgoing_target(he_mb, RelationKind::HalfEdgeNext)?;
        let next_am = draft.single_outgoing_target(he_am, RelationKind::HalfEdgeNext)?;
        if next_am != he_mb {
            return Err(SpecError::invalid(
                "KillVertexEdgeMutation requires the restricted two-halfedge split pattern"
                    .to_string(),
            ));
        }

        let radial_mb = draft.single_outgoing_target(he_mb, RelationKind::HalfEdgeRadialNext)?;
        let radial_am = draft.single_outgoing_target(he_am, RelationKind::HalfEdgeRadialNext)?;
        if radial_mb != he_mb || radial_am != he_am {
            return Err(SpecError::invalid(
                "KillVertexEdgeMutation requires self-radial halfedges".to_string(),
            ));
        }

        let face_mb = draft.single_outgoing_target(he_mb, RelationKind::HalfEdgeBoundsFace)?;
        let face_am = draft.single_outgoing_target(he_am, RelationKind::HalfEdgeBoundsFace)?;
        if face_mb != face_am {
            return Err(SpecError::invalid(
                "KillVertexEdgeMutation requires both halfedges to bound the same face".to_string(),
            ));
        }

        let new_edge = draft.single_outgoing_target(he_mb, RelationKind::HalfEdgeUsesEdge)?;
        let old_edge = draft.single_outgoing_target(he_am, RelationKind::HalfEdgeUsesEdge)?;
        if new_edge == old_edge {
            return Err(SpecError::invalid(
                "KillVertexEdgeMutation requires distinct old/new edges".to_string(),
            ));
        }

        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            he_am,
            he_am,
            "restore-self-loop",
        )?;

        draft.remove_relation_between(RelationKind::HalfEdgeNext, he_mb, he_am)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, he_mb, he_mb)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, he_mb, new_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeOriginVertex, he_mb, self.vertex)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, he_mb, face_mb)?;

        draft.delete_node(he_mb)?;
        draft.delete_node(new_edge)?;
        draft.delete_node(self.vertex)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("collapse restricted split vertex {}", self.vertex),
                "remove split halfedge, new edge, and midpoint vertex".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Collapse restricted split vertex {}", self.vertex)
    }
}
