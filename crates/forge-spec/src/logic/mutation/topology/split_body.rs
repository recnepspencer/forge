use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct SplitBodyMutation {
    pub body: SpecNodeId,
    pub lumps_to_move: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitBodyOutput {
    pub new_body: SpecNodeId,
}

impl std::fmt::Debug for SplitBodyMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SplitBodyMutation")
            .field("body", &self.body)
            .field("lumps_to_move", &self.lumps_to_move)
            .finish()
    }
}

impl SpecMutation for SplitBodyMutation {
    type Output = SplitBodyOutput;

    const NAME: &'static str = "split_body";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.body)? != SpecNodeKind::Body {
            return Err(SpecError::invalid(format!(
                "SplitBodyMutation requires Body input, got {:?}",
                draft.node_kind(self.body)?
            )));
        }
        if self.lumps_to_move.is_empty() {
            return Err(SpecError::invalid(
                "SplitBodyMutation requires at least one lump".to_string(),
            ));
        }

        let existing_lumps = draft.outgoing_targets_of_kind(self.body, RelationKind::BodyOwnsLump);
        if self.lumps_to_move.len() >= existing_lumps.len() {
            return Err(SpecError::invalid(
                "SplitBodyMutation cannot move all lumps out of the source body".to_string(),
            ));
        }
        for &lump in &self.lumps_to_move {
            if draft.node_kind(lump)? != SpecNodeKind::Lump {
                return Err(SpecError::invalid(format!(
                    "SplitBodyMutation requires Lump inputs, got {:?}",
                    draft.node_kind(lump)?
                )));
            }
            if !existing_lumps.contains(&lump) {
                return Err(SpecError::invalid(format!(
                    "SplitBodyMutation lump {} does not belong to body {}",
                    lump, self.body
                )));
            }
        }

        let new_body = draft.create_node(SpecNodeKind::Body, None, "split-body")?;
        for &lump in &self.lumps_to_move {
            draft.remove_relation_between(RelationKind::BodyOwnsLump, self.body, lump)?;
            draft.add_relation(
                RelationKind::BodyOwnsLump,
                new_body,
                lump,
                0,
                "split-body-lump",
            )?;
        }

        Ok(MutationResult {
            value: SplitBodyOutput { new_body },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create body {} from body {} split", new_body, self.body),
                format!("move {} lumps into new body", self.lumps_to_move.len()),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Split body {} by moving {} lumps",
            self.body,
            self.lumps_to_move.len()
        )
    }
}
