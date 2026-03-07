use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Extend a face loop by inserting a wire edge before the anchor halfedge.
#[derive(Debug, Clone, Copy)]
pub struct MakeEdgeVertexMutation {
    pub anchor: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeEdgeVertexOutput {
    pub new_vertex: SpecNodeId,
    pub he_out: SpecNodeId,
    pub he_back: SpecNodeId,
    pub edge: SpecNodeId,
}

impl SpecMutation for MakeEdgeVertexMutation {
    type Output = MakeEdgeVertexOutput;

    const NAME: &'static str = "make_edge_vertex";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let anchor = self.anchor;
        let origin = draft.single_outgoing_target(anchor, RelationKind::HalfEdgeOriginVertex)?;
        let face = draft.single_outgoing_target(anchor, RelationKind::HalfEdgeBoundsFace)?;
        let prev = draft.single_incoming_source(anchor, RelationKind::HalfEdgeNext)?;

        let new_vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;
        let new_edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let he_out = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge_out")?;
        let he_back = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge_back")?;

        draft.replace_single_relation(RelationKind::HalfEdgeNext, prev, he_out, "prev-next")?;
        add(draft, RelationKind::HalfEdgeNext, he_out, he_back, 0, "out-next")?;
        add(draft, RelationKind::HalfEdgeNext, he_back, anchor, 0, "back-next")?;

        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            he_out,
            he_back,
            0,
            "out-radial",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            he_back,
            he_out,
            0,
            "back-radial",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeUsesEdge,
            he_out,
            new_edge,
            0,
            "out-edge",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeUsesEdge,
            he_back,
            new_edge,
            0,
            "back-edge",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            he_out,
            origin,
            0,
            "out-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            he_back,
            new_vertex,
            0,
            "back-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            he_out,
            face,
            0,
            "out-face",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            he_back,
            face,
            0,
            "back-face",
        )?;

        Ok(MutationResult {
            value: MakeEdgeVertexOutput {
                new_vertex,
                he_out,
                he_back,
                edge: new_edge,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("insert antenna before anchor {}", anchor),
                "rewire loop predecessor and create twin wire edge".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Sprout wire edge at anchor {}", self.anchor)
    }
}

fn add(
    draft: &mut SpecDraft,
    kind: RelationKind,
    source: SpecNodeId,
    target: SpecNodeId,
    ordinal: u32,
    role: &str,
) -> Result<(), SpecError> {
    draft.add_relation(kind, source, target, ordinal, role)?;
    Ok(())
}
