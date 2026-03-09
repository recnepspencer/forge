use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::loop_traversal::{collect_loop_half_edges, find_face_loop_containing_half_edge};
use super::radial_traversal::collect_radial_ring;

#[derive(Debug, Clone)]
pub struct JoinFacesNmtMutation {
    pub half_edge_survive: SpecNodeId,
    pub half_edge_kill: SpecNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinFacesNmtOutput {
    pub surviving_face: SpecNodeId,
    pub slit_edge: SpecNodeId,
    pub slit_loop: SpecNodeId,
}

impl SpecMutation for JoinFacesNmtMutation {
    type Output = JoinFacesNmtOutput;

    const NAME: &'static str = "join_faces_nmt";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.half_edge_survive)? != SpecNodeKind::HalfEdge
            || draft.node_kind(self.half_edge_kill)? != SpecNodeKind::HalfEdge
        {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires halfedge inputs".to_string(),
            ));
        }
        if self.half_edge_survive == self.half_edge_kill {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires distinct halfedges".to_string(),
            ));
        }

        let shared_edge =
            draft.single_outgoing_target(self.half_edge_survive, RelationKind::HalfEdgeUsesEdge)?;
        if draft.single_outgoing_target(self.half_edge_kill, RelationKind::HalfEdgeUsesEdge)?
            != shared_edge
        {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires both halfedges to use the same edge".to_string(),
            ));
        }

        let ring = collect_radial_ring(draft, self.half_edge_survive)?;
        if ring.len() <= 2 {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires radial valence > 2; use JoinFacesMutation for valence-2"
                    .to_string(),
            ));
        }
        if !ring
            .iter()
            .copied()
            .any(|candidate| candidate == self.half_edge_kill)
        {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires half_edge_kill to be in the same radial ring"
                    .to_string(),
            ));
        }

        let surviving_face = draft
            .single_outgoing_target(self.half_edge_survive, RelationKind::HalfEdgeBoundsFace)?;
        let removed_face =
            draft.single_outgoing_target(self.half_edge_kill, RelationKind::HalfEdgeBoundsFace)?;
        if surviving_face == removed_face {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation does not support slit collapse on a single face".to_string(),
            ));
        }

        let surviving_shell =
            draft.single_incoming_source(surviving_face, RelationKind::ShellOwnsFace)?;
        let removed_shell =
            draft.single_incoming_source(removed_face, RelationKind::ShellOwnsFace)?;
        if surviving_shell != removed_shell {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires both faces to belong to the same shell".to_string(),
            ));
        }

        let surviving_outer =
            draft.single_outgoing_target(surviving_face, RelationKind::FaceOuterLoop)?;
        let removed_outer =
            draft.single_outgoing_target(removed_face, RelationKind::FaceOuterLoop)?;
        if find_face_loop_containing_half_edge(draft, surviving_face, self.half_edge_survive)?
            != surviving_outer
        {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation currently requires half_edge_survive on the surviving outer loop"
                    .to_string(),
            ));
        }
        if find_face_loop_containing_half_edge(draft, removed_face, self.half_edge_kill)?
            != removed_outer
        {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation currently requires half_edge_kill on the removed outer loop"
                    .to_string(),
            ));
        }

        let surviving_next =
            draft.single_outgoing_target(self.half_edge_survive, RelationKind::HalfEdgeNext)?;
        let surviving_prev =
            draft.single_incoming_source(self.half_edge_survive, RelationKind::HalfEdgeNext)?;
        let removed_next =
            draft.single_outgoing_target(self.half_edge_kill, RelationKind::HalfEdgeNext)?;
        let removed_prev =
            draft.single_incoming_source(self.half_edge_kill, RelationKind::HalfEdgeNext)?;

        let protected = ring
            .iter()
            .copied()
            .filter(|candidate| {
                *candidate != self.half_edge_survive && *candidate != self.half_edge_kill
            })
            .collect::<Vec<_>>();
        if protected.is_empty() {
            return Err(SpecError::invalid(
                "JoinFacesNmtMutation requires at least one protected radial use after removing the selected pair"
                    .to_string(),
            ));
        }

        let removed_outer_half_edges = collect_loop_half_edges(draft, removed_outer)?;
        let removed_inner_loops =
            draft.outgoing_targets_of_kind(removed_face, RelationKind::FaceInnerLoop);

        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            surviving_prev,
            removed_next,
            "join-nmt-surviving-prev-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            removed_prev,
            surviving_next,
            "join-nmt-removed-prev-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            self.half_edge_survive,
            self.half_edge_kill,
            "join-nmt-slit-survive-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            self.half_edge_kill,
            self.half_edge_survive,
            "join-nmt-slit-kill-next",
        )?;

        for index in 0..protected.len() {
            let current = protected[index];
            let next = protected[(index + 1) % protected.len()];
            draft.replace_single_relation(
                RelationKind::HalfEdgeRadialNext,
                current,
                next,
                "join-nmt-protected-radial",
            )?;
        }
        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            self.half_edge_survive,
            self.half_edge_kill,
            "join-nmt-slit-radial-survive",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeRadialNext,
            self.half_edge_kill,
            self.half_edge_survive,
            "join-nmt-slit-radial-kill",
        )?;

        let slit_edge = draft.create_node(SpecNodeKind::Edge, None, "slit-edge")?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeUsesEdge,
            self.half_edge_survive,
            slit_edge,
            "join-nmt-slit-edge-survive",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeUsesEdge,
            self.half_edge_kill,
            slit_edge,
            "join-nmt-slit-edge-kill",
        )?;

        for half_edge in removed_outer_half_edges {
            if half_edge != self.half_edge_kill {
                draft.replace_single_relation(
                    RelationKind::HalfEdgeBoundsFace,
                    half_edge,
                    surviving_face,
                    "join-nmt-transfer-outer",
                )?;
            }
        }
        draft.replace_single_relation(
            RelationKind::HalfEdgeBoundsFace,
            self.half_edge_kill,
            surviving_face,
            "join-nmt-slit-face-kill",
        )?;

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
                "join-nmt-transfer-inner-loop",
            )?;
            next_inner_ordinal += 1;

            for half_edge in loop_half_edges {
                draft.replace_single_relation(
                    RelationKind::HalfEdgeBoundsFace,
                    half_edge,
                    surviving_face,
                    "join-nmt-transfer-inner-halfedge",
                )?;
            }
        }

        let surviving_entry =
            draft.single_outgoing_target(surviving_outer, RelationKind::LoopEntryHalfEdge)?;
        if surviving_entry == self.half_edge_survive {
            draft.replace_single_relation(
                RelationKind::LoopEntryHalfEdge,
                surviving_outer,
                surviving_next,
                "join-nmt-surviving-loop-entry",
            )?;
        }

        draft.remove_relation_between(RelationKind::FaceOuterLoop, removed_face, removed_outer)?;
        draft.add_relation(
            RelationKind::FaceInnerLoop,
            surviving_face,
            removed_outer,
            next_inner_ordinal,
            "join-nmt-slit-loop",
        )?;
        draft.replace_single_relation(
            RelationKind::LoopEntryHalfEdge,
            removed_outer,
            self.half_edge_survive,
            "join-nmt-slit-loop-entry",
        )?;

        draft.remove_relation_between(
            RelationKind::ShellOwnsFace,
            surviving_shell,
            removed_face,
        )?;
        draft.delete_node(removed_face)?;

        Ok(MutationResult {
            value: JoinFacesNmtOutput {
                surviving_face,
                slit_edge,
                slit_loop: removed_outer,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "join faces {} and {} across high-valence edge {}",
                    surviving_face, removed_face, shared_edge
                ),
                format!(
                    "convert selected radial pair ({}, {}) into slit loop {} on new edge {}",
                    self.half_edge_survive, self.half_edge_kill, removed_outer, slit_edge
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Join non-manifold faces via surviving halfedge {} and removed halfedge {}",
            self.half_edge_survive, self.half_edge_kill
        )
    }
}
