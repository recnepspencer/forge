#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiQueryMeasurementBasisSuccessionDenial {
    MissingPredecessor,
    ForeignBindingSlot,
    AmbiguousPredecessor,
    SourceGenerationDidNotAdvance,
    SuccessorBasisDenied,
}

impl super::UiMeasurementBasis {
    pub(crate) fn succeed_settled_query_receipt(
        &self,
        successor: &crate::evidence::UiSettledQueryFactReceipt,
    ) -> Result<Self, UiQueryMeasurementBasisSuccessionDenial> {
        let mut inputs = self.evidence_inputs().to_vec();
        let mut query_receipts = 0usize;
        let mut matching = None;
        for (ordinal, input) in inputs.iter().enumerate() {
            let Some(predecessor) = input.as_settled_query_fact() else {
                continue;
            };
            query_receipts += 1;
            if predecessor.same_binding_slot(successor)
                && matching
                    .replace((ordinal, predecessor.source_generation()))
                    .is_some()
            {
                return Err(UiQueryMeasurementBasisSuccessionDenial::AmbiguousPredecessor);
            }
        }
        let Some((ordinal, predecessor_generation)) = matching else {
            return Err(if query_receipts == 0 {
                UiQueryMeasurementBasisSuccessionDenial::MissingPredecessor
            } else {
                UiQueryMeasurementBasisSuccessionDenial::ForeignBindingSlot
            });
        };
        if successor.source_generation() <= predecessor_generation {
            return Err(UiQueryMeasurementBasisSuccessionDenial::SourceGenerationDidNotAdvance);
        }
        inputs[ordinal] = crate::evidence::MeasurementEvidenceInput::settled_query_fact(successor);
        let admitted = super::admit_measurement_basis(
            self.declaration_identity().clone(),
            self.graph_node_identity(),
            self.world_profile().clone(),
            self.declaration_support_authority_generation(),
            self.declared_measurement_policy(),
            &inputs,
        );
        admitted
            .is_admitted()
            .then_some(admitted)
            .ok_or(UiQueryMeasurementBasisSuccessionDenial::SuccessorBasisDenied)
    }
}
