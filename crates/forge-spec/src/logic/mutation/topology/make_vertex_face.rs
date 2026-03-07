use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Create the seed topology in spec-graph truth:
/// one body, lump, region, shell, face, loop, self-loop halfedge, edge, and vertex.
///
/// Shell-kind/orientation semantics are intentionally deferred until the truth
/// schema owns them explicitly.
#[derive(Debug, Default, Clone, Copy)]
pub struct MakeVertexFaceMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeVertexFaceOutput {
    pub body: SpecNodeId,
    pub lump: SpecNodeId,
    pub region: SpecNodeId,
    pub shell: SpecNodeId,
    pub face: SpecNodeId,
    pub loop_id: SpecNodeId,
    pub half_edge: SpecNodeId,
    pub edge: SpecNodeId,
    pub vertex: SpecNodeId,
}

impl SpecMutation for MakeVertexFaceMutation {
    type Output = MakeVertexFaceOutput;

    const NAME: &'static str = "make_vertex_face";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let body = draft.create_node(SpecNodeKind::Body, None, "body")?;
        let lump = draft.create_node(SpecNodeKind::Lump, None, "lump")?;
        let region = draft.create_node(SpecNodeKind::Region, None, "region")?;
        let shell = draft.create_node(SpecNodeKind::Shell, None, "shell")?;
        let face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        let loop_id = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
        let half_edge = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge")?;
        let edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;

        add(draft, RelationKind::BodyOwnsLump, body, lump, 0, "body-lump")?;
        add(draft, RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")?;
        add(draft, RelationKind::RegionOwnsShell, region, shell, 0, "region-shell")?;
        add(draft, RelationKind::ShellOwnsFace, shell, face, 0, "shell-face")?;
        add(draft, RelationKind::FaceOuterLoop, face, loop_id, 0, "face-outer-loop")?;
        add(draft, RelationKind::LoopEntryHalfEdge, loop_id, half_edge, 0, "loop-entry")?;
        add(draft, RelationKind::HalfEdgeNext, half_edge, half_edge, 0, "halfedge-next")?;
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
            value: MakeVertexFaceOutput {
                body,
                lump,
                region,
                shell,
                face,
                loop_id,
                half_edge,
                edge,
                vertex,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                "create seed body/lump/region/shell/face/loop/halfedge/edge/vertex".to_string(),
                "wire self-loop halfedge and containment relations".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        "Create initial graph-native topology seed".to_string()
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
