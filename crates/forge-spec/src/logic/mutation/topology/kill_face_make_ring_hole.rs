use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::loop_traversal::collect_loop_half_edges;

#[derive(Debug, Clone)]
pub struct KillFaceMakeRingHoleMutation {
    pub face_to_kill: SpecNodeId,
    pub target_face: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KillFaceMakeRingHoleOutput;

impl SpecMutation for KillFaceMakeRingHoleMutation {
    type Output = KillFaceMakeRingHoleOutput;

    const NAME: &'static str = "kill_face_make_ring_hole";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if self.face_to_kill == self.target_face {
            return Err(SpecError::invalid(
                "KillFaceMakeRingHoleMutation cannot demote a face into itself".to_string(),
            ));
        }

        if draft.node_kind(self.face_to_kill)? != SpecNodeKind::Face
            || draft.node_kind(self.target_face)? != SpecNodeKind::Face
        {
            return Err(SpecError::invalid(
                "KillFaceMakeRingHoleMutation requires face inputs".to_string(),
            ));
        }

        let face_to_kill_shell =
            draft.single_incoming_source(self.face_to_kill, RelationKind::ShellOwnsFace)?;
        let target_shell =
            draft.single_incoming_source(self.target_face, RelationKind::ShellOwnsFace)?;
        if face_to_kill_shell != target_shell {
            return Err(SpecError::invalid(
                "KillFaceMakeRingHoleMutation requires faces in the same shell".to_string(),
            ));
        }

        if !draft
            .outgoing_targets_of_kind(self.face_to_kill, RelationKind::FaceInnerLoop)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "KillFaceMakeRingHoleMutation requires face_to_kill to have no inner loops"
                    .to_string(),
            ));
        }

        let loop_id =
            draft.single_outgoing_target(self.face_to_kill, RelationKind::FaceOuterLoop)?;
        let half_edges = collect_loop_half_edges(draft, loop_id)?;
        let ordinal = draft
            .outgoing_targets_of_kind(self.target_face, RelationKind::FaceInnerLoop)
            .len() as u32;

        draft.remove_relation_between(RelationKind::FaceOuterLoop, self.face_to_kill, loop_id)?;
        draft.remove_relation_between(
            RelationKind::ShellOwnsFace,
            target_shell,
            self.face_to_kill,
        )?;
        draft.add_relation(
            RelationKind::FaceInnerLoop,
            self.target_face,
            loop_id,
            ordinal,
            "demoted-face-inner-loop",
        )?;

        for half_edge in half_edges {
            draft.replace_single_relation(
                RelationKind::HalfEdgeBoundsFace,
                half_edge,
                self.target_face,
                "demoted-hole-face",
            )?;
        }

        draft.delete_node(self.face_to_kill)?;

        Ok(MutationResult {
            value: KillFaceMakeRingHoleOutput,
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "demote face {} to inner loop of face {}",
                    self.face_to_kill, self.target_face
                ),
                format!(
                    "reassign demoted loop halfedges back to face {}",
                    self.target_face
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Demote face {} into hole of face {}",
            self.face_to_kill, self.target_face
        )
    }
}
