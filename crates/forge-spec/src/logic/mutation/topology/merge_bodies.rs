use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct MergeBodiesMutation {
    pub target: SpecNodeId,
    pub source: SpecNodeId,
}

impl std::fmt::Debug for MergeBodiesMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MergeBodiesMutation")
            .field("target", &self.target)
            .field("source", &self.source)
            .finish()
    }
}

impl SpecMutation for MergeBodiesMutation {
    type Output = ();

    const NAME: &'static str = "merge_bodies";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.target)? != SpecNodeKind::Body
            || draft.node_kind(self.source)? != SpecNodeKind::Body
        {
            return Err(SpecError::invalid(
                "MergeBodiesMutation requires Body inputs".to_string(),
            ));
        }
        if self.target == self.source {
            return Err(SpecError::invalid(
                "MergeBodiesMutation requires distinct target/source bodies".to_string(),
            ));
        }

        for lump in draft.outgoing_targets_of_kind(self.source, RelationKind::BodyOwnsLump) {
            draft.remove_relation_between(RelationKind::BodyOwnsLump, self.source, lump)?;
            draft.add_relation(RelationKind::BodyOwnsLump, self.target, lump, 0, "merge-body-lump")?;
        }

        draft.delete_node(self.source)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("move lumps from body {} into body {}", self.source, self.target),
                format!("delete merged source body {}", self.source),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Merge body {} into body {}", self.source, self.target)
    }
}
