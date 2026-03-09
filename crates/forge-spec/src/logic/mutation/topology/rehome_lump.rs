use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct RehomeLumpMutation {
    pub lump: SpecNodeId,
    pub target_body: SpecNodeId,
}

impl std::fmt::Debug for RehomeLumpMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RehomeLumpMutation")
            .field("lump", &self.lump)
            .field("target_body", &self.target_body)
            .finish()
    }
}

impl SpecMutation for RehomeLumpMutation {
    type Output = ();

    const NAME: &'static str = "rehome_lump";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.lump)? != SpecNodeKind::Lump {
            return Err(SpecError::invalid(format!(
                "RehomeLumpMutation requires Lump input, got {:?}",
                draft.node_kind(self.lump)?
            )));
        }
        if draft.node_kind(self.target_body)? != SpecNodeKind::Body {
            return Err(SpecError::invalid(format!(
                "RehomeLumpMutation requires Body target, got {:?}",
                draft.node_kind(self.target_body)?
            )));
        }

        let source_body = draft.single_incoming_source(self.lump, RelationKind::BodyOwnsLump)?;
        if source_body == self.target_body {
            return Err(SpecError::invalid(
                "RehomeLumpMutation requires a different target body".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::BodyOwnsLump, source_body, self.lump)?;
        draft.add_relation(
            RelationKind::BodyOwnsLump,
            self.target_body,
            self.lump,
            0,
            "rehome-lump",
        )?;

        if draft
            .outgoing_targets_of_kind(source_body, RelationKind::BodyOwnsLump)
            .is_empty()
        {
            draft.delete_node(source_body)?;
        }

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("detach lump {} from body {}", self.lump, source_body),
                format!("attach lump {} to body {}", self.lump, self.target_body),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Move lump {} to body {}", self.lump, self.target_body)
    }
}
