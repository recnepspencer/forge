use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct DestroyShellMutation {
    pub shell: SpecNodeId,
}

impl std::fmt::Debug for DestroyShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestroyShellMutation")
            .field("shell", &self.shell)
            .finish()
    }
}

impl SpecMutation for DestroyShellMutation {
    type Output = ();

    const NAME: &'static str = "destroy_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "DestroyShellMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }

        if !draft
            .outgoing_targets_of_kind(self.shell, RelationKind::ShellOwnsFace)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "DestroyShellMutation requires an empty shell".to_string(),
            ));
        }

        let region = draft.single_incoming_source(self.shell, RelationKind::RegionOwnsShell)?;
        draft.remove_relation_between(RelationKind::RegionOwnsShell, region, self.shell)?;
        draft.delete_node(self.shell)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy shell {}", self.shell),
                "remove empty shell from existing region".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy shell {}", self.shell)
    }
}
