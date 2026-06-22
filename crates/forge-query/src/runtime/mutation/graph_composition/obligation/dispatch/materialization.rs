use super::super::execution::execute_selected_graph_obligation;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationSelection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationMaterializedDispatch {
    selection_digest: String,
    inputs: Vec<ForgeQueryGraphObligationExecutionInput>,
    dispatch_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationMaterializedDispatch {
    pub fn from_selection(selection: ForgeQueryGraphObligationSelection) -> Self {
        let selection_digest = selection.selection_digest().to_string();
        let inputs = selection
            .matched_registrations()
            .iter()
            .cloned()
            .map(|registration| {
                ForgeQueryGraphObligationExecutionInput::from_selected_registration(
                    selection_digest.clone(),
                    registration,
                )
            })
            .collect::<Vec<_>>();
        Self::new(selection_digest, inputs)
    }

    fn new(
        selection_digest: String,
        mut inputs: Vec<ForgeQueryGraphObligationExecutionInput>,
    ) -> Self {
        inputs.sort_by(|left, right| left.input_digest().cmp(right.input_digest()));
        let input_digests = inputs
            .iter()
            .map(ForgeQueryGraphObligationExecutionInput::input_evidence_digest)
            .collect::<Vec<_>>();
        let dispatch_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationMaterializedDispatch,
        )
        .field_value(ForgeQueryEvidenceTag::new("selection"), &selection_digest)
        .field_usize(ForgeQueryEvidenceTag::new("inputs"), inputs.len())
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("input"), input_digests)
        .seal();
        Self {
            selection_digest,
            inputs,
            dispatch_digest,
        }
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn inputs(&self) -> &[ForgeQueryGraphObligationExecutionInput] {
        &self.inputs
    }

    pub fn dispatch_digest(&self) -> &str {
        self.dispatch_digest.as_str()
    }

    pub fn selected_result_envelope(&self) -> ForgeQueryGraphObligationExecutionResultEnvelope {
        ForgeQueryGraphObligationExecutionResultEnvelope::new(
            self.inputs
                .iter()
                .cloned()
                .map(execute_selected_graph_obligation)
                .collect(),
        )
    }
}
