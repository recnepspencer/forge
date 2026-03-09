use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::payload::{SpecShellKind, SpecShellOrientation};
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct ExtractShellMutation {
    pub shell: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractShellOutput {
    pub new_region: SpecNodeId,
}

impl std::fmt::Debug for ExtractShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtractShellMutation")
            .field("shell", &self.shell)
            .finish()
    }
}

impl SpecMutation for ExtractShellMutation {
    type Output = ExtractShellOutput;

    const NAME: &'static str = "extract_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "ExtractShellMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }

        let source_region =
            draft.single_incoming_source(self.shell, RelationKind::RegionOwnsShell)?;
        let lump = draft.single_incoming_source(source_region, RelationKind::LumpOwnsRegion)?;
        let shell_kind = draft.shell_kind(self.shell)?;

        match shell_kind {
            SpecShellKind::Solid(SpecShellOrientation::Outer) => {
                return Err(SpecError::invalid(
                    "ExtractShellMutation cannot extract an outer solid shell".to_string(),
                ));
            }
            SpecShellKind::Solid(SpecShellOrientation::Inner) => {
                draft.set_shell_kind(
                    self.shell,
                    SpecShellKind::Solid(SpecShellOrientation::Outer),
                )?;
            }
            SpecShellKind::Sheet | SpecShellKind::Wire => {}
        }

        let new_region = draft.create_node(SpecNodeKind::Region, None, "extract-region")?;
        draft.add_relation(
            RelationKind::LumpOwnsRegion,
            lump,
            new_region,
            0,
            "extract-shell-region",
        )?;
        draft.remove_relation_between(RelationKind::RegionOwnsShell, source_region, self.shell)?;
        draft.add_relation(
            RelationKind::RegionOwnsShell,
            new_region,
            self.shell,
            0,
            "extract-shell-move",
        )?;

        Ok(MutationResult {
            value: ExtractShellOutput { new_region },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "create new region {} in lump {} for extracted shell {}",
                    new_region, lump, self.shell
                ),
                format!(
                    "move shell {} from region {} to region {}",
                    self.shell, source_region, new_region
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Extract shell {} into a new region", self.shell)
    }
}
