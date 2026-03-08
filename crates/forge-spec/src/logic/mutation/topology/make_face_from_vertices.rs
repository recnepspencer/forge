use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::payload::SpecShellKind;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::wire_face_cycle::create_face_cycle;

#[derive(Debug, Clone)]
pub struct MakeFaceFromVerticesMutation {
    pub vertices: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeFaceFromVerticesOutput {
    pub body: SpecNodeId,
    pub lump: SpecNodeId,
    pub region: SpecNodeId,
    pub shell: SpecNodeId,
    pub face: SpecNodeId,
    pub loop_id: SpecNodeId,
    pub half_edges: Vec<SpecNodeId>,
    pub edges: Vec<SpecNodeId>,
}

impl SpecMutation for MakeFaceFromVerticesMutation {
    type Output = MakeFaceFromVerticesOutput;

    const NAME: &'static str = "make_face_from_vertices";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let body = draft.create_node(SpecNodeKind::Body, None, "body")?;
        let lump = draft.create_node(SpecNodeKind::Lump, None, "lump")?;
        let region = draft.create_node(SpecNodeKind::Region, None, "region")?;
        let shell = draft.create_shell(SpecShellKind::Sheet, "shell")?;
        let face = draft.create_node(SpecNodeKind::Face, None, "face")?;

        draft.add_relation(RelationKind::BodyOwnsLump, body, lump, 0, "body-lump")?;
        draft.add_relation(RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")?;
        draft.add_relation(RelationKind::RegionOwnsShell, region, shell, 0, "region-shell")?;
        draft.add_relation(RelationKind::ShellOwnsFace, shell, face, 0, "shell-face")?;

        let wired = create_face_cycle(draft, face, &self.vertices, "face-from-vertices")?;

        Ok(MutationResult {
            value: MakeFaceFromVerticesOutput {
                body,
                lump,
                region,
                shell,
                face,
                loop_id: wired.loop_id,
                half_edges: wired.half_edges,
                edges: wired.edges,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "create new body/lump/region/shell for {}-vertex face",
                    self.vertices.len()
                ),
                "wire face/loop/halfedge cycle from existing vertices".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Create {}-vertex face from existing vertices", self.vertices.len())
    }
}
