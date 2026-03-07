use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct ExtractLumpMutation {
    pub lump: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractLumpOutput {
    pub new_body: SpecNodeId,
}

impl std::fmt::Debug for ExtractLumpMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtractLumpMutation")
            .field("lump", &self.lump)
            .finish()
    }
}

impl SpecMutation for ExtractLumpMutation {
    type Output = ExtractLumpOutput;

    const NAME: &'static str = "extract_lump";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.lump)? != SpecNodeKind::Lump {
            return Err(SpecError::invalid(format!(
                "ExtractLumpMutation requires Lump input, got {:?}",
                draft.node_kind(self.lump)?
            )));
        }

        let source_body = draft.single_incoming_source(self.lump, RelationKind::BodyOwnsLump)?;
        if draft.outgoing_targets_of_kind(source_body, RelationKind::BodyOwnsLump).len() <= 1 {
            return Err(SpecError::invalid(
                "ExtractLumpMutation cannot extract the last lump from a body".to_string(),
            ));
        }

        let new_body = draft.create_node(SpecNodeKind::Body, None, "extract-body")?;
        draft.remove_relation_between(RelationKind::BodyOwnsLump, source_body, self.lump)?;
        draft.add_relation(
            RelationKind::BodyOwnsLump,
            new_body,
            self.lump,
            0,
            "extract-lump-body",
        )?;

        Ok(MutationResult {
            value: ExtractLumpOutput { new_body },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create new body {} for extracted lump {}", new_body, self.lump),
                format!("move lump {} from body {} to body {}", self.lump, source_body, new_body),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Extract lump {} into a new body", self.lump)
    }
}
