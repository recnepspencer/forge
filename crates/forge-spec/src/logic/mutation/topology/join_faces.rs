use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::loop_traversal::{collect_loop_half_edges, find_face_loop_containing_half_edge};

#[derive(Debug, Clone)]
pub struct JoinFacesMutation {
    pub half_edge: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinFacesOutput {
    pub surviving_face: SpecNodeId,
}

impl SpecMutation for JoinFacesMutation {
    type Output = JoinFacesOutput;

    const NAME: &'static str = "join_faces";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.half_edge)? != SpecNodeKind::HalfEdge {
            return Err(SpecError::invalid(format!(
                "JoinFacesMutation requires HalfEdge input, got {:?}",
                draft.node_kind(self.half_edge)?
            )));
        }

        let twin =
            draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeRadialNext)?;
        if twin == self.half_edge
            || draft.single_outgoing_target(twin, RelationKind::HalfEdgeRadialNext)?
                != self.half_edge
        {
            return Err(SpecError::invalid(
                "JoinFacesMutation requires radial valence 2 at the removed edge".to_string(),
            ));
        }

        let surviving_face =
            draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeBoundsFace)?;
        let removed_face = draft.single_outgoing_target(twin, RelationKind::HalfEdgeBoundsFace)?;
        if surviving_face == removed_face {
            return Err(SpecError::invalid(
                "JoinFacesMutation requires two distinct incident faces".to_string(),
            ));
        }

        let surviving_shell =
            draft.single_incoming_source(surviving_face, RelationKind::ShellOwnsFace)?;
        let removed_shell =
            draft.single_incoming_source(removed_face, RelationKind::ShellOwnsFace)?;
        if surviving_shell != removed_shell {
            return Err(SpecError::invalid(
                "JoinFacesMutation requires both faces to belong to the same shell".to_string(),
            ));
        }

        let surviving_outer =
            draft.single_outgoing_target(surviving_face, RelationKind::FaceOuterLoop)?;
        let removed_outer =
            draft.single_outgoing_target(removed_face, RelationKind::FaceOuterLoop)?;
        if find_face_loop_containing_half_edge(draft, surviving_face, self.half_edge)?
            != surviving_outer
        {
            return Err(SpecError::invalid(
                "JoinFacesMutation currently requires the surviving halfedge to lie on the outer loop"
                    .to_string(),
            ));
        }
        if find_face_loop_containing_half_edge(draft, removed_face, twin)? != removed_outer {
            return Err(SpecError::invalid(
                "JoinFacesMutation currently requires the removed halfedge to lie on the removed face outer loop"
                    .to_string(),
            ));
        }

        let surviving_next =
            draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeNext)?;
        let surviving_prev =
            draft.single_incoming_source(self.half_edge, RelationKind::HalfEdgeNext)?;
        let removed_next = draft.single_outgoing_target(twin, RelationKind::HalfEdgeNext)?;
        let removed_prev = draft.single_incoming_source(twin, RelationKind::HalfEdgeNext)?;
        let shared_edge =
            draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeUsesEdge)?;

        let removed_outer_half_edges = collect_loop_half_edges(draft, removed_outer)?;
        let removed_inner_loops =
            draft.outgoing_targets_of_kind(removed_face, RelationKind::FaceInnerLoop);

        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            surviving_prev,
            removed_next,
            "join-surviving-prev-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            removed_prev,
            surviving_next,
            "join-removed-prev-next",
        )?;

        for half_edge in removed_outer_half_edges {
            if half_edge != twin {
                draft.replace_single_relation(
                    RelationKind::HalfEdgeBoundsFace,
                    half_edge,
                    surviving_face,
                    "join-face-transfer-outer",
                )?;
            }
        }

        let mut next_inner_ordinal = draft
            .outgoing_targets_of_kind(surviving_face, RelationKind::FaceInnerLoop)
            .len() as u32;
        for loop_id in removed_inner_loops {
            let loop_half_edges = collect_loop_half_edges(draft, loop_id)?;
            draft.remove_relation_between(RelationKind::FaceInnerLoop, removed_face, loop_id)?;
            draft.add_relation(
                RelationKind::FaceInnerLoop,
                surviving_face,
                loop_id,
                next_inner_ordinal,
                "join-face-transfer-inner-loop",
            )?;
            next_inner_ordinal += 1;

            for half_edge in loop_half_edges {
                draft.replace_single_relation(
                    RelationKind::HalfEdgeBoundsFace,
                    half_edge,
                    surviving_face,
                    "join-face-transfer-inner-halfedge",
                )?;
            }
        }

        let surviving_entry =
            draft.single_outgoing_target(surviving_outer, RelationKind::LoopEntryHalfEdge)?;
        if surviving_entry == self.half_edge {
            draft.replace_single_relation(
                RelationKind::LoopEntryHalfEdge,
                surviving_outer,
                surviving_next,
                "join-face-outer-loop-entry",
            )?;
        }

        draft.remove_relation_between(RelationKind::FaceOuterLoop, removed_face, removed_outer)?;
        draft.remove_relation_between(RelationKind::LoopEntryHalfEdge, removed_outer, twin)?;
        draft.remove_relation_between(
            RelationKind::ShellOwnsFace,
            surviving_shell,
            removed_face,
        )?;

        remove_half_edge_pair(draft, self.half_edge, twin)?;
        draft.delete_node(shared_edge)?;
        draft.delete_node(removed_outer)?;
        draft.delete_node(removed_face)?;

        Ok(MutationResult {
            value: JoinFacesOutput { surviving_face },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "join faces {} and {} across halfedge {}",
                    surviving_face, removed_face, self.half_edge
                ),
                format!(
                    "delete shared edge {} and absorbed face {}",
                    shared_edge, removed_face
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Join faces across shared halfedge {}", self.half_edge)
    }
}

fn remove_half_edge_pair(
    draft: &mut SpecDraft,
    half_edge_a: SpecNodeId,
    half_edge_b: SpecNodeId,
) -> Result<(), SpecError> {
    draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, half_edge_a, half_edge_b)?;
    draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, half_edge_b, half_edge_a)?;
    remove_half_edge_node(draft, half_edge_a)?;
    remove_half_edge_node(draft, half_edge_b)
}

fn remove_half_edge_node(draft: &mut SpecDraft, half_edge: SpecNodeId) -> Result<(), SpecError> {
    draft.remove_single_outgoing_relation(RelationKind::HalfEdgeNext, half_edge)?;
    draft.remove_single_outgoing_relation(RelationKind::HalfEdgeUsesEdge, half_edge)?;
    draft.remove_single_outgoing_relation(RelationKind::HalfEdgeOriginVertex, half_edge)?;
    draft.remove_single_outgoing_relation(RelationKind::HalfEdgeBoundsFace, half_edge)?;
    draft.delete_node(half_edge)
}
