use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::loop_traversal::collect_loop_half_edges;

#[derive(Debug, Clone)]
pub struct MakeFaceKillRingHoleMutation {
    pub loop_id: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeFaceKillRingHoleOutput {
    pub new_face: SpecNodeId,
}

impl SpecMutation for MakeFaceKillRingHoleMutation {
    type Output = MakeFaceKillRingHoleOutput;

    const NAME: &'static str = "make_face_kill_ring_hole";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.loop_id)? != SpecNodeKind::Loop {
            return Err(SpecError::invalid(format!(
                "MakeFaceKillRingHoleMutation requires Loop input, got {:?}",
                draft.node_kind(self.loop_id)?
            )));
        }

        let old_face = draft.single_incoming_source(self.loop_id, RelationKind::FaceInnerLoop)?;
        let shell = draft.single_incoming_source(old_face, RelationKind::ShellOwnsFace)?;
        let half_edges = collect_loop_half_edges(draft, self.loop_id)?;

        draft.remove_relation_between(RelationKind::FaceInnerLoop, old_face, self.loop_id)?;

        let new_face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        draft.add_relation(
            RelationKind::ShellOwnsFace,
            shell,
            new_face,
            0,
            "shell-face",
        )?;
        draft.add_relation(
            RelationKind::FaceOuterLoop,
            new_face,
            self.loop_id,
            0,
            "promoted-hole-outer-loop",
        )?;

        for half_edge in half_edges {
            draft.replace_single_relation(
                RelationKind::HalfEdgeBoundsFace,
                half_edge,
                new_face,
                "promoted-hole-face",
            )?;
        }

        Ok(MutationResult {
            value: MakeFaceKillRingHoleOutput { new_face },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("promote inner loop {} to a new face", self.loop_id),
                format!("reassign promoted loop halfedges to face {}", new_face),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Promote inner loop {} to its own face", self.loop_id)
    }
}
