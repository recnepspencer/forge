#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiHostMeasurementBasisSuccessionDenial {
    MissingPredecessor,
    AmbiguousPredecessor,
    EvidenceCategoryMismatch,
    NormalizationAuthorityMismatch,
    SourcePositionDidNotAdvance,
    SuccessorBasisDenied,
}

impl super::UiMeasurementBasis {
    pub(crate) fn succeed_host_measurement_result(
        &self,
        successor: &crate::evidence::UiMeasurementResult,
    ) -> Result<Self, UiHostMeasurementBasisSuccessionDenial> {
        let mut inputs = self.evidence_inputs().to_vec();
        let mut matching = None;
        for (ordinal, input) in inputs.iter().enumerate() {
            let Some(predecessor) = input.as_host_measurement_result() else {
                continue;
            };
            if predecessor.request_identity() != successor.request_identity() {
                continue;
            }
            if matching.replace((ordinal, predecessor)).is_some() {
                return Err(UiHostMeasurementBasisSuccessionDenial::AmbiguousPredecessor);
            }
        }
        let Some((ordinal, predecessor)) = matching else {
            return Err(UiHostMeasurementBasisSuccessionDenial::MissingPredecessor);
        };
        if predecessor.evidence_category() != successor.evidence_category() {
            return Err(UiHostMeasurementBasisSuccessionDenial::EvidenceCategoryMismatch);
        }
        if !predecessor
            .authority_witness()
            .same_normalization_authority(successor.authority_witness())
        {
            return Err(UiHostMeasurementBasisSuccessionDenial::NormalizationAuthorityMismatch);
        }
        let predecessor_position = predecessor.host_source_position();
        let successor_position = successor.host_source_position();
        if predecessor_position.1 != successor_position.1
            || successor_position.2 <= predecessor_position.2
        {
            return Err(UiHostMeasurementBasisSuccessionDenial::SourcePositionDidNotAdvance);
        }
        inputs[ordinal] =
            crate::evidence::MeasurementEvidenceInput::host_measurement_result(successor);
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
            .ok_or(UiHostMeasurementBasisSuccessionDenial::SuccessorBasisDenied)
    }
}
