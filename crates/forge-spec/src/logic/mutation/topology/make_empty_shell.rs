use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct MakeEmptyShellMutation {
    pub region: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeEmptyShellOutput {
    pub shell: SpecNodeId,
}

impl std::fmt::Debug for MakeEmptyShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MakeEmptyShellMutation")
            .field("region", &self.region)
            .finish()
    }
}

impl SpecMutation for MakeEmptyShellMutation {
    type Output = MakeEmptyShellOutput;

    const NAME: &'static str = "make_empty_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.region)? != SpecNodeKind::Region {
            return Err(SpecError::invalid(format!(
                "MakeEmptyShellMutation requires Region input, got {:?}",
                draft.node_kind(self.region)?
            )));
        }

        let shell = draft.create_node(SpecNodeKind::Shell, None, "shell")?;
        draft.add_relation(
            RelationKind::RegionOwnsShell,
            self.region,
            shell,
            0,
            "region-shell",
        )?;

        Ok(MutationResult {
            value: MakeEmptyShellOutput { shell },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create empty shell in region {}", self.region),
                "attach shell without faces to existing region".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Create empty shell in region {}", self.region)
    }
}
