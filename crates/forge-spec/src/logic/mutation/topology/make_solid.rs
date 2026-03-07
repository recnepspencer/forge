use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone, Copy)]
pub struct MakeSolidMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeSolidOutput {
    pub body: SpecNodeId,
    pub lump: SpecNodeId,
    pub region: SpecNodeId,
}

impl SpecMutation for MakeSolidMutation {
    type Output = MakeSolidOutput;

    const NAME: &'static str = "make_solid";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let body = draft.create_node(SpecNodeKind::Body, None, "body")?;
        let lump = draft.create_node(SpecNodeKind::Lump, None, "lump")?;
        let region = draft.create_node(SpecNodeKind::Region, None, "region")?;

        draft.add_relation(RelationKind::BodyOwnsLump, body, lump, 0, "body-lump")?;
        draft.add_relation(RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")?;

        Ok(MutationResult {
            value: MakeSolidOutput { body, lump, region },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                "create solid container hierarchy".to_string(),
                "wire body -> lump -> region".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        "Create solid container hierarchy".to_string()
    }
}
