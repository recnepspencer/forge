use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::payload::{SpecShellKind, SpecShellOrientation};
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct PromoteShellMutation {
    pub shell: SpecNodeId,
}

impl std::fmt::Debug for PromoteShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PromoteShellMutation")
            .field("shell", &self.shell)
            .finish()
    }
}

impl SpecMutation for PromoteShellMutation {
    type Output = ();

    const NAME: &'static str = "promote_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "PromoteShellMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }

        let region = draft.single_incoming_source(self.shell, RelationKind::RegionOwnsShell)?;
        if draft.shell_kind(self.shell)? != SpecShellKind::Solid(SpecShellOrientation::Inner) {
            return Err(SpecError::invalid(
                "PromoteShellMutation requires an inner solid shell".to_string(),
            ));
        }

        for shell in draft.outgoing_targets_of_kind(region, RelationKind::RegionOwnsShell) {
            if shell != self.shell
                && draft.shell_kind(shell)? == SpecShellKind::Solid(SpecShellOrientation::Outer)
            {
                draft.set_shell_kind(shell, SpecShellKind::Solid(SpecShellOrientation::Inner))?;
            }
        }
        draft.set_shell_kind(self.shell, SpecShellKind::Solid(SpecShellOrientation::Outer))?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![format!(
                "promote shell {} to outer role within region {}",
                self.shell, region
            )],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Promote shell {} to outer role", self.shell)
    }
}
