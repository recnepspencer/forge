use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct DestroyBodyMutation {
    pub body: SpecNodeId,
}

impl std::fmt::Debug for DestroyBodyMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestroyBodyMutation")
            .field("body", &self.body)
            .finish()
    }
}

impl SpecMutation for DestroyBodyMutation {
    type Output = ();

    const NAME: &'static str = "destroy_body";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.body)? != SpecNodeKind::Body {
            return Err(SpecError::invalid(format!(
                "DestroyBodyMutation requires Body input, got {:?}",
                draft.node_kind(self.body)?
            )));
        }

        let lumps = draft.outgoing_targets_of_kind(self.body, RelationKind::BodyOwnsLump);
        if lumps.len() != 1 {
            return Err(SpecError::invalid(
                "DestroyBodyMutation requires exactly one lump".to_string(),
            ));
        }
        let lump = lumps[0];
        let regions = draft.outgoing_targets_of_kind(lump, RelationKind::LumpOwnsRegion);
        if regions.len() != 1 {
            return Err(SpecError::invalid(
                "DestroyBodyMutation requires exactly one region on the lump".to_string(),
            ));
        }
        let region = regions[0];
        if !draft
            .outgoing_targets_of_kind(region, RelationKind::RegionOwnsShell)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "DestroyBodyMutation requires an empty region".to_string(),
            ));
        }

        draft.remove_relation_between(RelationKind::BodyOwnsLump, self.body, lump)?;
        draft.remove_relation_between(RelationKind::LumpOwnsRegion, lump, region)?;
        draft.delete_node(region)?;
        draft.delete_node(lump)?;
        draft.delete_node(self.body)?;

        Ok(MutationResult {
            value: (),
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("destroy body {}", self.body),
                "remove empty body/lump/region hierarchy".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Destroy body {}", self.body)
    }
}
