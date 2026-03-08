use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct SplitShellMutation {
    pub shell: SpecNodeId,
    pub faces_to_move: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitShellOutput {
    pub new_shell: SpecNodeId,
}

impl std::fmt::Debug for SplitShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SplitShellMutation")
            .field("shell", &self.shell)
            .field("faces_to_move", &self.faces_to_move)
            .finish()
    }
}

impl SpecMutation for SplitShellMutation {
    type Output = SplitShellOutput;

    const NAME: &'static str = "split_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "SplitShellMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }
        if self.faces_to_move.is_empty() {
            return Err(SpecError::invalid(
                "SplitShellMutation requires at least one face".to_string(),
            ));
        }

        let region = draft.single_incoming_source(self.shell, RelationKind::RegionOwnsShell)?;
        let shell_kind = draft.shell_kind(self.shell)?;
        let source_faces = draft.outgoing_targets_of_kind(self.shell, RelationKind::ShellOwnsFace);
        for &face in &self.faces_to_move {
            if draft.node_kind(face)? != SpecNodeKind::Face {
                return Err(SpecError::invalid(format!(
                    "SplitShellMutation requires Face inputs, got {:?}",
                    draft.node_kind(face)?
                )));
            }
            if !source_faces.contains(&face) {
                return Err(SpecError::invalid(format!(
                    "SplitShellMutation face {} does not belong to shell {}",
                    face, self.shell
                )));
            }
        }

        let new_shell = draft.create_shell(shell_kind, "split-shell")?;
        draft.add_relation(
            RelationKind::RegionOwnsShell,
            region,
            new_shell,
            0,
            "split-shell-region",
        )?;

        for &face in &self.faces_to_move {
            draft.remove_relation_between(RelationKind::ShellOwnsFace, self.shell, face)?;
            draft.add_relation(
                RelationKind::ShellOwnsFace,
                new_shell,
                face,
                0,
                "split-shell-face",
            )?;
        }

        if draft.outgoing_targets_of_kind(self.shell, RelationKind::ShellOwnsFace).is_empty() {
            draft.remove_relation_between(RelationKind::RegionOwnsShell, region, self.shell)?;
            draft.delete_node(self.shell)?;
        }

        Ok(MutationResult {
            value: SplitShellOutput { new_shell },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "create shell {} in region {} and move {} faces from shell {}",
                    new_shell,
                    region,
                    self.faces_to_move.len(),
                    self.shell
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Split shell {} by moving {} faces",
            self.shell,
            self.faces_to_move.len()
        )
    }
}
