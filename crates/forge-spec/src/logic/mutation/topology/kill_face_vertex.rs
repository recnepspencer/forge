use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Destroy a disjoint single-face seed while preserving its parent shell.
///
/// This is the restricted inverse of `MakeFaceVertexMutation`.
pub struct KillFaceVertexMutation {
    pub face: SpecNodeId,
}

impl std::fmt::Debug for KillFaceVertexMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KillFaceVertexMutation")
            .field("face", &self.face)
            .finish()
    }
}

impl SpecMutation for KillFaceVertexMutation {
    type Output = ();

    const NAME: &'static str = "kill_face_vertex";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.face)? != SpecNodeKind::Face {
            return Err(SpecError::invalid(format!(
                "KillFaceVertexMutation requires Face input, got {:?}",
                draft.node_kind(self.face)?
            )));
        }
        if !draft
            .outgoing_targets_of_kind(self.face, RelationKind::FaceInnerLoop)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "KillFaceVertexMutation requires a face without inner loops".to_string(),
            ));
        }

        let shell = draft.single_incoming_source(self.face, RelationKind::ShellOwnsFace)?;
        let shell_faces = draft.outgoing_targets_of_kind(shell, RelationKind::ShellOwnsFace);
        if shell_faces.len() < 2 {
            return Err(SpecError::invalid(
                "KillFaceVertexMutation requires a shell with at least one surviving sibling face"
                    .to_string(),
            ));
        }

        let loop_id = draft.single_outgoing_target(self.face, RelationKind::FaceOuterLoop)?;
        let half_edge = draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge)?;
        let next = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeNext)?;
        let radial = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeRadialNext)?;
        let vertex = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeOriginVertex)?;
        let edge = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeUsesEdge)?;
        let bounds_face = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeBoundsFace)?;

        if next != half_edge || radial != half_edge {
            return Err(SpecError::invalid(
                "KillFaceVertexMutation requires a self-loop halfedge seed".to_string(),
            ));
        }
        if bounds_face != self.face {
            return Err(SpecError::invalid(
                "KillFaceVertexMutation face boundary is inconsistent".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::ShellOwnsFace, shell, self.face)?;
        draft.remove_relation_between(RelationKind::FaceOuterLoop, self.face, loop_id)?;
        draft.remove_relation_between(RelationKind::LoopEntryHalfEdge, loop_id, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeNext, half_edge, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, half_edge, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, half_edge, edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeOriginVertex, half_edge, vertex)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, half_edge, self.face)?;

        draft.delete_node(self.face)?;
        draft.delete_node(loop_id)?;
        draft.delete_node(half_edge)?;
        draft.delete_node(edge)?;
        draft.delete_node(vertex)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy disjoint face seed {}", self.face),
                "remove face/loop/self-loop halfedge seed while preserving parent shell"
                    .to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy disjoint face seed {}", self.face)
    }
}
