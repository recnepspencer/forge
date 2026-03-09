use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Destroy an isolated single-face shell seed while preserving its parent region.
///
/// This is the restricted inverse of `MakeShellFaceMutation`.
/// It requires:
/// - exactly one face in the shell
/// - that face to have one outer loop and no inner loops
/// - that loop to contain one self-loop halfedge
pub struct KillShellFaceMutation {
    pub face: SpecNodeId,
    pub vertex: SpecNodeId,
}

impl std::fmt::Debug for KillShellFaceMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KillShellFaceMutation")
            .field("face", &self.face)
            .field("vertex", &self.vertex)
            .finish()
    }
}

impl SpecMutation for KillShellFaceMutation {
    type Output = ();

    const NAME: &'static str = "kill_shell_face";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.face)? != SpecNodeKind::Face {
            return Err(SpecError::invalid(format!(
                "KillShellFaceMutation requires Face input, got {:?}",
                draft.node_kind(self.face)?
            )));
        }
        if draft.node_kind(self.vertex)? != SpecNodeKind::Vertex {
            return Err(SpecError::invalid(format!(
                "KillShellFaceMutation requires Vertex input, got {:?}",
                draft.node_kind(self.vertex)?
            )));
        }
        if !draft
            .outgoing_targets_of_kind(self.face, RelationKind::FaceInnerLoop)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "KillShellFaceMutation supports only shells without inner loops".to_string(),
            ));
        }

        let shell = draft.single_incoming_source(self.face, RelationKind::ShellOwnsFace)?;
        let region = draft.single_incoming_source(shell, RelationKind::RegionOwnsShell)?;
        let shell_faces = draft.outgoing_targets_of_kind(shell, RelationKind::ShellOwnsFace);
        if shell_faces.len() != 1 {
            return Err(SpecError::invalid(
                "KillShellFaceMutation requires a single-face shell".to_string(),
            ));
        }

        let loop_id = draft.single_outgoing_target(self.face, RelationKind::FaceOuterLoop)?;
        let half_edge = draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge)?;
        let next = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeNext)?;
        let radial = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeRadialNext)?;
        let origin = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeOriginVertex)?;
        let _edge = draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeUsesEdge)?;
        let bounds_face =
            draft.single_outgoing_target(half_edge, RelationKind::HalfEdgeBoundsFace)?;

        if next != half_edge || radial != half_edge {
            return Err(SpecError::invalid(
                "KillShellFaceMutation requires a self-loop halfedge seed".to_string(),
            ));
        }
        if origin != self.vertex {
            return Err(SpecError::invalid(format!(
                "KillShellFaceMutation vertex {} does not match halfedge origin {}",
                self.vertex, origin
            )));
        }
        if bounds_face != self.face {
            return Err(SpecError::invalid(
                "KillShellFaceMutation face boundary is inconsistent".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::RegionOwnsShell, region, shell)?;
        draft.remove_relation_between(RelationKind::ShellOwnsFace, shell, self.face)?;
        draft.remove_relation_between(RelationKind::FaceOuterLoop, self.face, loop_id)?;
        draft.remove_relation_between(RelationKind::LoopEntryHalfEdge, loop_id, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeNext, half_edge, half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, half_edge, half_edge)?;
        let edge =
            draft.remove_single_outgoing_relation(RelationKind::HalfEdgeUsesEdge, half_edge)?;
        draft.remove_relation_between(
            RelationKind::HalfEdgeOriginVertex,
            half_edge,
            self.vertex,
        )?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, half_edge, self.face)?;

        draft.delete_node(self.face)?;
        draft.delete_node(self.vertex)?;
        draft.delete_node(loop_id)?;
        draft.delete_node(half_edge)?;
        draft.delete_node(edge)?;
        draft.delete_node(shell)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy shell seed containing face {}", self.face),
                "remove shell/face/loop/self-loop halfedge seed".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy shell seed containing face {}", self.face)
    }
}
