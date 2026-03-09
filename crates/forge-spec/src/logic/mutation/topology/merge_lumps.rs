use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct MergeLumpsMutation {
    pub target: SpecNodeId,
    pub source: SpecNodeId,
}

impl std::fmt::Debug for MergeLumpsMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MergeLumpsMutation")
            .field("target", &self.target)
            .field("source", &self.source)
            .finish()
    }
}

impl SpecMutation for MergeLumpsMutation {
    type Output = ();

    const NAME: &'static str = "merge_lumps";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.target)? != SpecNodeKind::Lump
            || draft.node_kind(self.source)? != SpecNodeKind::Lump
        {
            return Err(SpecError::invalid(
                "MergeLumpsMutation requires Lump inputs".to_string(),
            ));
        }
        if self.target == self.source {
            return Err(SpecError::invalid(
                "MergeLumpsMutation requires distinct target/source lumps".to_string(),
            ));
        }

        let target_body = draft.single_incoming_source(self.target, RelationKind::BodyOwnsLump)?;
        let source_body = draft.single_incoming_source(self.source, RelationKind::BodyOwnsLump)?;
        if target_body != source_body {
            return Err(SpecError::invalid(
                "MergeLumpsMutation requires both lumps to belong to the same body".to_string(),
            ));
        }

        for region in draft.outgoing_targets_of_kind(self.source, RelationKind::LumpOwnsRegion) {
            draft.remove_relation_between(RelationKind::LumpOwnsRegion, self.source, region)?;
            draft.add_relation(
                RelationKind::LumpOwnsRegion,
                self.target,
                region,
                0,
                "merge-lump-region",
            )?;
        }

        draft.remove_relation_between(RelationKind::BodyOwnsLump, source_body, self.source)?;
        draft.delete_node(self.source)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "move regions from lump {} into lump {}",
                    self.source, self.target
                ),
                format!("delete merged source lump {}", self.source),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Merge lump {} into lump {}", self.source, self.target)
    }
}
