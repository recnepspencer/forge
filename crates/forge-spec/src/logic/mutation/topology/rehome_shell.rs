use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct RehomeShellMutation {
    pub shell: SpecNodeId,
    pub target_region: SpecNodeId,
}

impl std::fmt::Debug for RehomeShellMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RehomeShellMutation")
            .field("shell", &self.shell)
            .field("target_region", &self.target_region)
            .finish()
    }
}

impl SpecMutation for RehomeShellMutation {
    type Output = ();

    const NAME: &'static str = "rehome_shell";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.shell)? != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "RehomeShellMutation requires Shell input, got {:?}",
                draft.node_kind(self.shell)?
            )));
        }
        if draft.node_kind(self.target_region)? != SpecNodeKind::Region {
            return Err(SpecError::invalid(format!(
                "RehomeShellMutation requires Region target, got {:?}",
                draft.node_kind(self.target_region)?
            )));
        }

        let source_region = draft.single_incoming_source(self.shell, RelationKind::RegionOwnsShell)?;
        if source_region == self.target_region {
            return Err(SpecError::invalid(
                "RehomeShellMutation requires a different target region".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::RegionOwnsShell, source_region, self.shell)?;
        draft.add_relation(
            RelationKind::RegionOwnsShell,
            self.target_region,
            self.shell,
            0,
            "rehome-shell",
        )?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("detach shell {} from region {}", self.shell, source_region),
                format!("attach shell {} to region {}", self.shell, self.target_region),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Move shell {} to region {}", self.shell, self.target_region)
    }
}
