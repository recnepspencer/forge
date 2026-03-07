use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct MergeShellsMutation {
    pub target: SpecNodeId,
    pub source: SpecNodeId,
}

impl std::fmt::Debug for MergeShellsMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MergeShellsMutation")
            .field("target", &self.target)
            .field("source", &self.source)
            .finish()
    }
}

impl SpecMutation for MergeShellsMutation {
    type Output = ();

    const NAME: &'static str = "merge_shells";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.target)? != SpecNodeKind::Shell
            || draft.node_kind(self.source)? != SpecNodeKind::Shell
        {
            return Err(SpecError::invalid(
                "MergeShellsMutation requires Shell inputs".to_string(),
            ));
        }
        if self.target == self.source {
            return Err(SpecError::invalid(
                "MergeShellsMutation requires distinct target/source shells".to_string(),
            ));
        }

        let target_region = draft.single_incoming_source(self.target, RelationKind::RegionOwnsShell)?;
        let source_region = draft.single_incoming_source(self.source, RelationKind::RegionOwnsShell)?;
        if target_region != source_region {
            return Err(SpecError::invalid(
                "MergeShellsMutation requires both shells to belong to the same region".to_string(),
            ));
        }

        for face in draft.outgoing_targets_of_kind(self.source, RelationKind::ShellOwnsFace) {
            draft.remove_relation_between(RelationKind::ShellOwnsFace, self.source, face)?;
            draft.add_relation(
                RelationKind::ShellOwnsFace,
                self.target,
                face,
                0,
                "merge-shell-face",
            )?;
        }

        draft.remove_relation_between(RelationKind::RegionOwnsShell, source_region, self.source)?;
        draft.delete_node(self.source)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("move faces from shell {} into shell {}", self.source, self.target),
                format!("delete merged source shell {}", self.source),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Merge shell {} into shell {}", self.source, self.target)
    }
}
