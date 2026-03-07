use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::wire_face_cycle::create_face_cycle;

#[derive(Debug, Clone)]
pub struct MakeFaceInShellFromVerticesMutation {
    pub shell: SpecNodeId,
    pub vertices: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeFaceInShellFromVerticesOutput {
    pub face: SpecNodeId,
    pub loop_id: SpecNodeId,
    pub half_edges: Vec<SpecNodeId>,
    pub edges: Vec<SpecNodeId>,
}

impl SpecMutation for MakeFaceInShellFromVerticesMutation {
    type Output = MakeFaceInShellFromVerticesOutput;

    const NAME: &'static str = "make_face_in_shell_from_vertices";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "MakeFaceInShellFromVerticesMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }

        let face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        draft.add_relation(
            RelationKind::ShellOwnsFace,
            self.shell,
            face,
            0,
            "shell-face",
        )?;

        let wired = create_face_cycle(draft, face, &self.vertices, "face-in-shell")?;

        Ok(MutationResult {
            value: MakeFaceInShellFromVerticesOutput {
                face,
                loop_id: wired.loop_id,
                half_edges: wired.half_edges,
                edges: wired.edges,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "create {}-vertex face in shell {}",
                    self.vertices.len(),
                    self.shell
                ),
                "wire face/loop/halfedge cycle from existing vertices".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Create {}-vertex face in shell {}",
            self.vertices.len(),
            self.shell
        )
    }
}
