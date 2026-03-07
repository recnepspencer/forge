use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Split a self-loop seed edge into a two-halfedge loop by inserting a vertex.
///
/// This is intentionally restricted to the current seed/self-loop case.
/// Full radial-chain splitting is deferred until the spec truth schema and
/// projection parity cover broader edge-topology cases.
#[derive(Debug, Clone, Copy)]
pub struct SplitEdgeMutation {
    pub half_edge: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitEdgeOutput {
    pub he_am: SpecNodeId,
    pub he_mb: SpecNodeId,
    pub new_vertex: SpecNodeId,
    pub new_edge: SpecNodeId,
}

impl SpecMutation for SplitEdgeMutation {
    type Output = SplitEdgeOutput;

    const NAME: &'static str = "split_edge";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let he_ab = self.half_edge;
        let next = draft.single_outgoing_target(he_ab, RelationKind::HalfEdgeNext)?;
        let radial_next = draft.single_outgoing_target(he_ab, RelationKind::HalfEdgeRadialNext)?;
        let old_edge = draft.single_outgoing_target(he_ab, RelationKind::HalfEdgeUsesEdge)?;
        let face = draft.single_outgoing_target(he_ab, RelationKind::HalfEdgeBoundsFace)?;

        if next != he_ab || radial_next != he_ab {
            return Err(SpecError::invalid(
                "SplitEdgeMutation currently supports only self-loop halfedges".to_string(),
            ));
        }

        let new_vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;
        let new_edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let he_mb = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge_split")?;

        draft.replace_single_relation(RelationKind::HalfEdgeNext, he_ab, he_mb, "split-next")?;

        add(draft, RelationKind::HalfEdgeNext, he_mb, he_ab, 0, "new-next")?;
        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            he_mb,
            he_mb,
            0,
            "new-radial",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeUsesEdge,
            he_mb,
            new_edge,
            0,
            "new-edge",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            he_mb,
            new_vertex,
            0,
            "new-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            he_mb,
            face,
            0,
            "new-face",
        )?;

        draft.replace_single_relation(
            RelationKind::HalfEdgeUsesEdge,
            he_ab,
            old_edge,
            "old-edge",
        )?;

        Ok(MutationResult {
            value: SplitEdgeOutput {
                he_am: he_ab,
                he_mb,
                new_vertex,
                new_edge,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("split self-loop halfedge {}", he_ab),
                "insert midpoint vertex and new edge halfedge".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Split self-loop halfedge {}", self.half_edge)
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
