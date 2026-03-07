use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Destroy the exact isolated seed created by `MakeVertexFaceMutation`.
pub struct KillVertexFaceMutation {
    pub face: SpecNodeId,
    pub vertex: SpecNodeId,
}

impl std::fmt::Debug for KillVertexFaceMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KillVertexFaceMutation")
            .field("face", &self.face)
            .field("vertex", &self.vertex)
            .finish()
    }
}

impl SpecMutation for KillVertexFaceMutation {
    type Output = ();

    const NAME: &'static str = "kill_vertex_face";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.face)? != SpecNodeKind::Face {
            return Err(SpecError::invalid(format!(
                "KillVertexFaceMutation requires Face input, got {:?}",
                draft.node_kind(self.face)?
            )));
        }
        if draft.node_kind(self.vertex)? != SpecNodeKind::Vertex {
            return Err(SpecError::invalid(format!(
                "KillVertexFaceMutation requires Vertex input, got {:?}",
                draft.node_kind(self.vertex)?
            )));
        }

        let shell = draft.single_incoming_source(self.face, RelationKind::ShellOwnsFace)?;
        let region = draft.single_incoming_source(shell, RelationKind::RegionOwnsShell)?;
        let lump = draft.single_incoming_source(region, RelationKind::LumpOwnsRegion)?;
        let body = draft.single_incoming_source(lump, RelationKind::BodyOwnsLump)?;

        if draft.outgoing_targets_of_kind(shell, RelationKind::ShellOwnsFace).len() != 1
            || draft.outgoing_targets_of_kind(region, RelationKind::RegionOwnsShell).len() != 1
            || draft.outgoing_targets_of_kind(lump, RelationKind::LumpOwnsRegion).len() != 1
            || draft.outgoing_targets_of_kind(body, RelationKind::BodyOwnsLump).len() != 1
        {
            return Err(SpecError::invalid(
                "KillVertexFaceMutation requires an isolated single-seed containment chain"
                    .to_string(),
            ));
        }
        if !draft
            .outgoing_targets_of_kind(self.face, RelationKind::FaceInnerLoop)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "KillVertexFaceMutation supports only faces without inner loops".to_string(),
            ));
        }

        let loop_id = draft.single_outgoing_target(self.face, RelationKind::FaceOuterLoop)?;
        let half_edge = draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge)?;
        let next = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeNext)?;
        let radial = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeRadialNext)?;
        let origin = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeOriginVertex)?;
        let edge = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeUsesEdge)?;
        let bounds_face = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeBoundsFace)?;

        if next != half_edge || radial != half_edge {
            return Err(SpecError::invalid(
                "KillVertexFaceMutation requires a self-loop halfedge seed".to_string(),
            ));
        }
        if origin != self.vertex || bounds_face != self.face {
            return Err(SpecError::invalid(
                "KillVertexFaceMutation face/vertex inputs do not match the seed halfedge".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::BodyOwnsLump, body, lump)?;
        draft.remove_relation_between(RelationKind::LumpOwnsRegion, lump, region)?;
        draft.remove_relation_between(RelationKind::RegionOwnsShell, region, shell)?;
        draft.remove_relation_between(RelationKind::ShellOwnsFace, shell, self.face)?;
        draft.remove_relation_between(RelationKind::FaceOuterLoop, self.face, loop_id)?;
        draft.remove_relation_between(RelationKind::LoopEntryHalfEdge, loop_id, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeNext, half_edge, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, half_edge, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, half_edge, edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeOriginVertex, half_edge, self.vertex)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, half_edge, self.face)?;

        draft.delete_node(self.face)?;
        draft.delete_node(self.vertex)?;
        draft.delete_node(loop_id)?;
        draft.delete_node(half_edge)?;
        draft.delete_node(edge)?;
        draft.delete_node(shell)?;
        draft.delete_node(region)?;
        draft.delete_node(lump)?;
        draft.delete_node(body)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy isolated seed containing face {}", self.face),
                "remove body/lump/region/shell/face/loop/halfedge/edge/vertex seed".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy isolated seed containing face {}", self.face)
    }
}
