use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct MakeLumpRegionMutation {
    pub body: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeLumpRegionOutput {
    pub lump: SpecNodeId,
    pub region: SpecNodeId,
}

impl std::fmt::Debug for MakeLumpRegionMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MakeLumpRegionMutation")
            .field("body", &self.body)
            .finish()
    }
}

impl SpecMutation for MakeLumpRegionMutation {
    type Output = MakeLumpRegionOutput;

    const NAME: &'static str = "make_lump_region";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.body)? != SpecNodeKind::Body {
            return Err(SpecError::invalid(format!(
                "MakeLumpRegionMutation requires Body input, got {:?}",
                draft.node_kind(self.body)?
            )));
        }

        let lump = draft.create_node(SpecNodeKind::Lump, None, "lump")?;
        let region = draft.create_node(SpecNodeKind::Region, None, "region")?;

        draft.add_relation(RelationKind::BodyOwnsLump, self.body, lump, 0, "body-lump")?;
        draft.add_relation(RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")?;

        Ok(MutationResult {
            value: MakeLumpRegionOutput { lump, region },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!("create lump/region in body {}", self.body),
                "attach new lump and empty region to existing body".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Create lump/region in body {}", self.body)
    }
}
