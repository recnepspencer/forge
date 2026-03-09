use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::payload::SpecShellKind;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Create a new shell under an existing region, seeded with a single self-loop face.
///
#[derive(Debug, Clone, Copy)]
pub struct MakeShellFaceMutation {
    pub region: SpecNodeId,
    pub kind: SpecShellKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeShellFaceOutput {
    pub shell: SpecNodeId,
    pub face: SpecNodeId,
    pub loop_id: SpecNodeId,
    pub half_edge: SpecNodeId,
    pub edge: SpecNodeId,
    pub vertex: SpecNodeId,
}

impl SpecMutation for MakeShellFaceMutation {
    type Output = MakeShellFaceOutput;

    const NAME: &'static str = "make_shell_face";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let region_kind = draft.node_kind(self.region)?;
        if region_kind != SpecNodeKind::Region {
            return Err(SpecError::invalid(format!(
                "MakeShellFaceMutation requires Region input, got {:?}",
                region_kind
            )));
        }

        let shell = draft.create_shell(self.kind, "shell")?;
        let face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        let loop_id = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
        let half_edge = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge")?;
        let edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;

        add(
            draft,
            RelationKind::RegionOwnsShell,
            self.region,
            shell,
            0,
            "region-shell",
        )?;
        add(
            draft,
            RelationKind::ShellOwnsFace,
            shell,
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
            value: MakeShellFaceOutput {
                shell,
                face,
                loop_id,
                half_edge,
                edge,
                vertex,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create shell seed in region {}", self.region),
                "wire shell/face/loop/self-loop halfedge seed".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Create shell seed in region {}", self.region)
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
