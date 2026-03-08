use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::payload::{SpecShellKind, SpecShellOrientation};
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct DemoteShellMutation {
    pub region: SpecNodeId,
}

impl std::fmt::Debug for DemoteShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemoteShellMutation")
            .field("region", &self.region)
            .finish()
    }
}

impl SpecMutation for DemoteShellMutation {
    type Output = ();

    const NAME: &'static str = "demote_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.region)? != SpecNodeKind::Region {
            return Err(SpecError::invalid(format!(
                "DemoteShellMutation requires Region input, got {:?}",
                draft.node_kind(self.region)?
            )));
        }

        let mut outer_shell = None;
        for shell in draft.outgoing_targets_of_kind(self.region, RelationKind::RegionOwnsShell) {
            if draft.shell_kind(shell)? == SpecShellKind::Solid(SpecShellOrientation::Outer) {
                outer_shell = Some(shell);
                break;
            }
        }
        let Some(shell) = outer_shell else {
            return Err(SpecError::invalid(
                "DemoteShellMutation requires a region with an outer solid shell".to_string(),
            ));
        };

        draft.set_shell_kind(shell, SpecShellKind::Solid(SpecShellOrientation::Inner))?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![format!(
                "demote outer shell {} to inner role within region {}",
                shell, self.region
            )],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Demote the outer shell of region {}", self.region)
    }
}
