use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct SplitLumpMutation {
    pub lump: SpecNodeId,
    pub regions_to_move: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitLumpOutput {
    pub new_lump: SpecNodeId,
}

impl std::fmt::Debug for SplitLumpMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SplitLumpMutation")
            .field("lump", &self.lump)
            .field("regions_to_move", &self.regions_to_move)
            .finish()
    }
}

impl SpecMutation for SplitLumpMutation {
    type Output = SplitLumpOutput;

    const NAME: &'static str = "split_lump";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.lump)? != SpecNodeKind::Lump {
            return Err(SpecError::invalid(format!(
                "SplitLumpMutation requires Lump input, got {:?}",
                draft.node_kind(self.lump)?
            )));
        }
        if self.regions_to_move.is_empty() {
            return Err(SpecError::invalid(
                "SplitLumpMutation requires at least one region".to_string(),
            ));
        }

        let body = draft.single_incoming_source(self.lump, RelationKind::BodyOwnsLump)?;
        let existing_regions = draft.outgoing_targets_of_kind(self.lump, RelationKind::LumpOwnsRegion);
        if self.regions_to_move.len() >= existing_regions.len() {
            return Err(SpecError::invalid(
                "SplitLumpMutation cannot move all regions out of the source lump".to_string(),
            ));
        }
        for &region in &self.regions_to_move {
            if draft.node_kind(region)? != SpecNodeKind::Region {
                return Err(SpecError::invalid(format!(
                    "SplitLumpMutation requires Region inputs, got {:?}",
                    draft.node_kind(region)?
                )));
            }
            if !existing_regions.contains(&region) {
                return Err(SpecError::invalid(format!(
                    "SplitLumpMutation region {} does not belong to lump {}",
                    region, self.lump
                )));
            }
        }

        let new_lump = draft.create_node(SpecNodeKind::Lump, None, "split-lump")?;
        draft.add_relation(RelationKind::BodyOwnsLump, body, new_lump, 0, "split-lump-body")?;

        for &region in &self.regions_to_move {
            draft.remove_relation_between(RelationKind::LumpOwnsRegion, self.lump, region)?;
            draft.add_relation(RelationKind::LumpOwnsRegion, new_lump, region, 0, "split-lump-region")?;
        }

        Ok(MutationResult {
            value: SplitLumpOutput { new_lump },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create lump {} in body {}", new_lump, body),
                format!("move {} regions from lump {}", self.regions_to_move.len(), self.lump),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Split lump {} by moving {} regions",
            self.lump,
            self.regions_to_move.len()
        )
    }
}
