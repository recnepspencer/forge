use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::SpecNodeKind;
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone, Copy)]
pub struct MakeIsolatedVertexMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeIsolatedVertexOutput {
    pub vertex: SpecNodeId,
}

impl SpecMutation for MakeIsolatedVertexMutation {
    type Output = MakeIsolatedVertexOutput;

    const NAME: &'static str = "make_isolated_vertex";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex")?;

        Ok(MutationResult {
            value: MakeIsolatedVertexOutput { vertex },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec!["create isolated vertex".to_string()],
        })
    }

    fn semantic_summary(&self) -> String {
        "Create isolated vertex".to_string()
    }
}
