use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct DestroyLumpMutation {
    pub lump: SpecNodeId,
}

impl std::fmt::Debug for DestroyLumpMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestroyLumpMutation")
            .field("lump", &self.lump)
            .finish()
    }
}

impl SpecMutation for DestroyLumpMutation {
    type Output = ();

    const NAME: &'static str = "destroy_lump";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.lump)? != SpecNodeKind::Lump {
            return Err(SpecError::invalid(format!(
                "DestroyLumpMutation requires Lump input, got {:?}",
                draft.node_kind(self.lump)?
            )));
        }

        let body = draft.single_incoming_source(self.lump, RelationKind::BodyOwnsLump)?;
        let regions = draft.outgoing_targets_of_kind(self.lump, RelationKind::LumpOwnsRegion);
        if regions.len() != 1 {
            return Err(SpecError::invalid(
                "DestroyLumpMutation requires exactly one region".to_string(),
            ));
        }
        let region = regions[0];
        if !draft
            .outgoing_targets_of_kind(region, RelationKind::RegionOwnsShell)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "DestroyLumpMutation requires an empty region".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::BodyOwnsLump, body, self.lump)?;
        draft.remove_relation_between(RelationKind::LumpOwnsRegion, self.lump, region)?;
        draft.delete_node(region)?;
        draft.delete_node(self.lump)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy lump {}", self.lump),
                "remove empty lump and region from existing body".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy lump {}", self.lump)
    }
}
