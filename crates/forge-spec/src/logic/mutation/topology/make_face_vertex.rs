use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Create a disjoint seed face inside an existing shell.
#[derive(Debug, Clone, Copy)]
pub struct MakeFaceVertexMutation {
    pub shell: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeFaceVertexOutput {
    pub face: SpecNodeId,
    pub loop_id: SpecNodeId,
    pub half_edge: SpecNodeId,
    pub edge: SpecNodeId,
    pub vertex: SpecNodeId,
}

impl SpecMutation for MakeFaceVertexMutation {
    type Output = MakeFaceVertexOutput;

    const NAME: &'static str = "make_face_vertex";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "MakeFaceVertexMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }

        let face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        let loop_id = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
        let half_edge = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge")?;
        let edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;

        add(
            draft,
            RelationKind::ShellOwnsFace,
            self.shell,
            face,
            0,
            "shell-face",
        )?;
        add(
            draft,
            RelationKind::FaceOuterLoop,
            face,
            loop_id,
            0,
            "face-outer-loop",
        )?;
        add(
            draft,
            RelationKind::LoopEntryHalfEdge,
            loop_id,
            half_edge,
            0,
            "loop-entry",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeNext,
            half_edge,
            half_edge,
            0,
            "halfedge-next",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            half_edge,
            half_edge,
            0,
            "halfedge-radial",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeUsesEdge,
            half_edge,
            edge,
            0,
            "halfedge-edge",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            half_edge,
            vertex,
            0,
            "halfedge-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            half_edge,
            face,
            0,
            "halfedge-face",
        )?;

        Ok(MutationResult {
            value: MakeFaceVertexOutput {
                face,
                loop_id,
                half_edge,
                edge,
                vertex,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create disjoint face seed in shell {}", self.shell),
                "wire face/loop/self-loop halfedge seed under existing shell".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Create disjoint face seed in shell {}", self.shell)
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
