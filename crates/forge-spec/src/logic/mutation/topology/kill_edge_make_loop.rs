use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone)]
pub struct KillEdgeMakeLoopMutation {
    pub half_edge: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KillEdgeMakeLoopOutput {
    pub new_loop: SpecNodeId,
}

impl SpecMutation for KillEdgeMakeLoopMutation {
    type Output = KillEdgeMakeLoopOutput;

    const NAME: &'static str = "kill_edge_make_loop";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.half_edge)? != SpecNodeKind::HalfEdge {
            return Err(SpecError::invalid(
                "KillEdgeMakeLoopMutation requires HalfEdge input".to_string(),
            ));
        }

        let twin = draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeRadialNext)?;
        if draft.single_outgoing_target(twin, RelationKind::HalfEdgeRadialNext)? != self.half_edge {
            return Err(SpecError::invalid(
                "KillEdgeMakeLoopMutation requires a two-halfedge radial pair".to_string(),
            ));
        }

        let face = draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeBoundsFace)?;
        if draft.single_outgoing_target(twin, RelationKind::HalfEdgeBoundsFace)? != face {
            return Err(SpecError::invalid(
                "KillEdgeMakeLoopMutation requires a same-face bridge edge".to_string(),
            ));
        }

        let edge = draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeUsesEdge)?;
        if draft.single_outgoing_target(twin, RelationKind::HalfEdgeUsesEdge)? != edge {
            return Err(SpecError::invalid(
                "KillEdgeMakeLoopMutation requires both halfedges to use the same edge"
                    .to_string(),
            ));
        }

        let he_prev = draft.single_incoming_source(self.half_edge, RelationKind::HalfEdgeNext)?;
        let he_next = draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeNext)?;
        let twin_prev = draft.single_incoming_source(twin, RelationKind::HalfEdgeNext)?;
        let twin_next = draft.single_outgoing_target(twin, RelationKind::HalfEdgeNext)?;

        let outer_loop = draft.single_outgoing_target(face, RelationKind::FaceOuterLoop)?;

        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            he_prev,
            twin_next,
            "keml-outer-next",
        )?;
        draft.replace_single_relation(
            RelationKind::HalfEdgeNext,
            twin_prev,
            he_next,
            "keml-inner-next",
        )?;

        let new_loop = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
        let ordinal = draft.outgoing_targets_of_kind(face, RelationKind::FaceInnerLoop).len() as u32;
        draft.add_relation(
            RelationKind::FaceInnerLoop,
            face,
            new_loop,
            ordinal,
            "keml-inner-loop",
        )?;
        draft.add_relation(
            RelationKind::LoopEntryHalfEdge,
            new_loop,
            he_next,
            0,
            "keml-loop-entry",
        )?;

        let outer_entry = draft.single_outgoing_target(outer_loop, RelationKind::LoopEntryHalfEdge)?;
        if outer_entry == self.half_edge || outer_entry == twin {
            draft.replace_single_relation(
                RelationKind::LoopEntryHalfEdge,
                outer_loop,
                twin_next,
                "keml-outer-loop-entry",
            )?;
        }

        draft.remove_relation_between(RelationKind::HalfEdgeNext, self.half_edge, he_next)?;
        draft.remove_relation_between(RelationKind::HalfEdgeNext, twin, twin_next)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, self.half_edge, twin)?;
        draft.remove_relation_between(RelationKind::HalfEdgeRadialNext, twin, self.half_edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, self.half_edge, edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeUsesEdge, twin, edge)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, self.half_edge, face)?;
        draft.remove_relation_between(RelationKind::HalfEdgeBoundsFace, twin, face)?;

        let vertex_a =
            draft.single_outgoing_target(self.half_edge, RelationKind::HalfEdgeOriginVertex)?;
        let vertex_b = draft.single_outgoing_target(twin, RelationKind::HalfEdgeOriginVertex)?;
        draft.remove_relation_between(
            RelationKind::HalfEdgeOriginVertex,
            self.half_edge,
            vertex_a,
        )?;
        draft.remove_relation_between(RelationKind::HalfEdgeOriginVertex, twin, vertex_b)?;

        draft.delete_node(self.half_edge)?;
        draft.delete_node(twin)?;
        draft.delete_node(edge)?;

        Ok(MutationResult {
            value: KillEdgeMakeLoopOutput { new_loop },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("remove bridge edge at halfedge {}", self.half_edge),
                format!("restore inner loop {}", new_loop),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Remove bridge edge at halfedge {} and create inner loop",
            self.half_edge
        )
    }
}
