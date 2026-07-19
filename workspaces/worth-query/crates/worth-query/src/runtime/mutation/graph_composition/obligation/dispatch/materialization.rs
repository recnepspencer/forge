use super::super::execution::execute_selected_graph_obligation;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionResultEnvelope,
    WorthQueryGraphObligationSelection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationMaterializedDispatch {
    selection_digest: String,
    inputs: Vec<WorthQueryGraphObligationExecutionInput>,
    dispatch_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationMaterializedDispatch {
    pub fn from_selection(selection: WorthQueryGraphObligationSelection) -> Self {
        let selection_digest = selection.selection_digest().to_string();
        let inputs = selection
            .matched_registrations()
            .iter()
            .cloned()
            .map(|registration| {
                WorthQueryGraphObligationExecutionInput::from_selected_registration(
                    selection_digest.clone(),
                    registration,
                )
            })
            .collect::<Vec<_>>();
        Self::new(selection_digest, inputs)
    }

    fn new(
        selection_digest: String,
        mut inputs: Vec<WorthQueryGraphObligationExecutionInput>,
    ) -> Self {
        inputs.sort_by(|left, right| left.input_digest().cmp(right.input_digest()));
        let input_digests = inputs
            .iter()
            .map(WorthQueryGraphObligationExecutionInput::input_evidence_digest)
            .collect::<Vec<_>>();
        let dispatch_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationMaterializedDispatch,
        )
        .field_value(WorthQueryEvidenceTag::new("selection"), &selection_digest)
        .field_usize(WorthQueryEvidenceTag::new("inputs"), inputs.len())
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("input"), input_digests)
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

    pub fn inputs(&self) -> &[WorthQueryGraphObligationExecutionInput] {
        &self.inputs
    }

    pub fn dispatch_digest(&self) -> &str {
        self.dispatch_digest.as_str()
    }

    pub fn selected_result_envelope(&self) -> WorthQueryGraphObligationExecutionResultEnvelope {
        WorthQueryGraphObligationExecutionResultEnvelope::new(
            self.inputs
                .iter()
                .cloned()
                .map(execute_selected_graph_obligation)
                .collect(),
        )
    }
}
