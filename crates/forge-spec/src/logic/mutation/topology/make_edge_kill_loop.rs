use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::loop_traversal::find_face_loop_containing_half_edge;

#[derive(Debug, Clone)]
pub struct MakeEdgeKillLoopMutation {
    pub half_edge_a: SpecNodeId,
    pub half_edge_b: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeEdgeKillLoopOutput {
    pub half_edge_ab: SpecNodeId,
    pub half_edge_ba: SpecNodeId,
    pub edge: SpecNodeId,
    pub killed_loop: SpecNodeId,
}

impl SpecMutation for MakeEdgeKillLoopMutation {
    type Output = MakeEdgeKillLoopOutput;

    const NAME: &'static str = "make_edge_kill_loop";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.half_edge_a)? != SpecNodeKind::HalfEdge
            || draft.node_kind(self.half_edge_b)? != SpecNodeKind::HalfEdge
        {
            return Err(SpecError::invalid(
                "MakeEdgeKillLoopMutation requires halfedge inputs".to_string(),
            ));
        }

        let face_a =
            draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeBoundsFace)?;
        let face_b =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeBoundsFace)?;
        if face_a != face_b {
            return Err(SpecError::invalid(
                "MakeEdgeKillLoopMutation requires both halfedges to bound the same face"
                    .to_string(),
            ));
        }

        let face = face_a;
        let outer_loop = draft.single_outgoing_target(face, RelationKind::FaceOuterLoop)?;
        let loop_a = find_face_loop_containing_half_edge(draft, face, self.half_edge_a)?;
        let loop_b = find_face_loop_containing_half_edge(draft, face, self.half_edge_b)?;

        if loop_a == loop_b {
            return Err(SpecError::invalid(
                "MakeEdgeKillLoopMutation requires halfedges on different loops".to_string(),
            ));
        }
        if loop_a != outer_loop {
            return Err(SpecError::invalid(
                "MakeEdgeKillLoopMutation currently requires half_edge_a on the outer loop"
                    .to_string(),
            ));
        }
        if loop_b == outer_loop {
            return Err(SpecError::invalid(
                "MakeEdgeKillLoopMutation requires half_edge_b on an inner loop".to_string(),
            ));
        }

        let prev_a = draft.single_incoming_source(self.half_edge_a, RelationKind::HalfEdgeNext)?;
        let prev_b = draft.single_incoming_source(self.half_edge_b, RelationKind::HalfEdgeNext)?;
        let vertex_a =
            draft.single_outgoing_target(self.half_edge_a, RelationKind::HalfEdgeOriginVertex)?;
        let vertex_b =
            draft.single_outgoing_target(self.half_edge_b, RelationKind::HalfEdgeOriginVertex)?;

        let edge = draft.create_node(SpecNodeKind::Edge, None, "bridge_edge")?;
        let half_edge_ab =
            draft.create_node(SpecNodeKind::HalfEdge, None, "bridge_half_edge_ab")?;
        let half_edge_ba =
            draft.create_node(SpecNodeKind::HalfEdge, None, "bridge_half_edge_ba")?;

        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            prev_a,
            half_edge_ab,
            "bridge-prev-a-next",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeNext,
            half_edge_ab,
            self.half_edge_b,
            0,
            "bridge-ab-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            prev_b,
            half_edge_ba,
            "bridge-prev-b-next",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeNext,
            half_edge_ba,
            self.half_edge_a,
            0,
            "bridge-ba-next",
        )?;

        draft.add_relation(
            RelationKind::HalfEdgeRadialNext,
            half_edge_ab,
            half_edge_ba,
            0,
            "bridge-ab-radial",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeRadialNext,
            half_edge_ba,
            half_edge_ab,
            0,
            "bridge-ba-radial",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeUsesEdge,
            half_edge_ab,
            edge,
            0,
            "bridge-ab-edge",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeUsesEdge,
            half_edge_ba,
            edge,
            0,
            "bridge-ba-edge",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeOriginVertex,
            half_edge_ab,
            vertex_a,
            0,
            "bridge-ab-origin",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeOriginVertex,
            half_edge_ba,
            vertex_b,
            0,
            "bridge-ba-origin",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeBoundsFace,
            half_edge_ab,
            face,
            0,
            "bridge-ab-face",
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeBoundsFace,
            half_edge_ba,
            face,
            0,
            "bridge-ba-face",
        )?;

        draft.replace_single_relation(
            RelationKind::LoopEntryHalfEdge,
            loop_a,
            half_edge_ab,
            "bridge-surviving-loop-entry",
        )?;
        let loop_b_entry = draft.single_outgoing_target(loop_b, RelationKind::LoopEntryHalfEdge)?;
        draft.remove_relation_between(RelationKind::LoopEntryHalfEdge, loop_b, loop_b_entry)?;
        draft.remove_relation_between(RelationKind::FaceInnerLoop, face, loop_b)?;
        draft.delete_node(loop_b)?;

        Ok(MutationResult {
            value: MakeEdgeKillLoopOutput {
                half_edge_ab,
                half_edge_ba,
                edge,
                killed_loop: loop_b,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "bridge outer loop halfedge {} to inner loop halfedge {}",
                    self.half_edge_a, self.half_edge_b
                ),
                format!("kill inner loop {}", loop_b),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Bridge halfedge {} to halfedge {} and kill inner loop",
            self.half_edge_a, self.half_edge_b
        )
    }
}
