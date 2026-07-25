#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalMeasurementBasisSuccessionDenial {
    MissingPredecessor,
    AmbiguousPredecessor,
    EvidenceCategoryMismatch,
    NormalizationAuthorityMismatch,
    EvidenceGenerationDidNotAdvance,
    SuccessorBasisDenied,
}

impl super::UiMeasurementBasis {
    pub(crate) fn succeed_portal_measurement_result(
        &self,
        successor: &crate::evidence::UiMeasurementResult,
    ) -> Result<Self, UiPortalMeasurementBasisSuccessionDenial> {
        if successor.evidence_category()
            != crate::evidence::UiMeasurementEvidenceCategory::PortalAnchorRect
        {
            return Err(UiPortalMeasurementBasisSuccessionDenial::EvidenceCategoryMismatch);
        }
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
                return Err(UiPortalMeasurementBasisSuccessionDenial::AmbiguousPredecessor);
            }
        }
        let Some((ordinal, predecessor)) = matching else {
            return Err(UiPortalMeasurementBasisSuccessionDenial::MissingPredecessor);
        };
        if predecessor.evidence_category() != successor.evidence_category() {
            return Err(UiPortalMeasurementBasisSuccessionDenial::EvidenceCategoryMismatch);
        }
        if !predecessor
            .authority_witness()
            .same_normalization_authority(successor.authority_witness())
        {
            return Err(UiPortalMeasurementBasisSuccessionDenial::NormalizationAuthorityMismatch);
        }
        if successor.evidence_generation() <= predecessor.evidence_generation() {
            return Err(UiPortalMeasurementBasisSuccessionDenial::EvidenceGenerationDidNotAdvance);
        }
        inputs[ordinal] =
            crate::evidence::MeasurementEvidenceInput::host_measurement_result(successor);
        let admitted = super::admit_measurement_basis(
            self.declaration_identity().clone(),
            self.graph_node_identity(),
            self.world_profile().clone(),
            successor.evidence_generation(),
            self.declared_measurement_policy(),
            &inputs,
        );
        admitted
            .is_admitted()
            .then_some(admitted)
            .ok_or(UiPortalMeasurementBasisSuccessionDenial::SuccessorBasisDenied)
    }
}

#[cfg(test)]
mod tests {
    use super::UiPortalMeasurementBasisSuccessionDenial as Denial;
    use crate::evidence::measurement::projection::fact_test_support::{
        capability_report, host_result_portal_anchor_at, synthetic_declaration_identity,
    };
    use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
    use worth_ui_inspection::UiEvidenceAuthorityGeneration;

    #[test]
    fn exact_portal_slot_succeeds_without_replacing_unrelated_inputs() {
        let (basis, report) = portal_basis(980, 17);
        let successor = host_result_portal_anchor_at(
            980,
            45,
            [5.0, 6.0, 7.0, 8.0],
            &report,
            UiEvidenceAuthorityGeneration::new(18),
        );
        let succeeded = basis
            .succeed_portal_measurement_result(&successor)
            .expect("same-slot newer portal evidence should admit");

        assert_eq!(
            succeeded.host_measurement_result(successor.request_identity()),
            Some(&successor)
        );
        assert_eq!(
            succeeded.declaration_support_authority_generation(),
            successor.evidence_generation()
        );
        assert_eq!(succeeded.evidence_inputs().len(), 2);
        assert!(succeeded
            .evidence_inputs()
            .iter()
            .any(|input| input.as_host_capability_report() == Some(&report)));
    }

    #[test]
    fn non_advancing_or_foreign_portal_evidence_is_typed() {
        let (basis, report) = portal_basis(980, 17);
        let stale = host_result_portal_anchor_at(
            980,
            45,
            [5.0, 6.0, 7.0, 8.0],
            &report,
            UiEvidenceAuthorityGeneration::new(17),
        );
        assert_eq!(
            basis.succeed_portal_measurement_result(&stale),
            Err(Denial::EvidenceGenerationDidNotAdvance)
        );
        let foreign = host_result_portal_anchor_at(
            981,
            45,
            [5.0, 6.0, 7.0, 8.0],
            &report,
            UiEvidenceAuthorityGeneration::new(18),
        );
        assert_eq!(
            basis.succeed_portal_measurement_result(&foreign),
            Err(Denial::MissingPredecessor)
        );
    }

    fn portal_basis(
        request_seed: u64,
        generation: u64,
    ) -> (
        super::super::UiMeasurementBasis,
        worth_ui_host_contract::WorthUiHostCapabilityReport,
    ) {
        let report = capability_report(77);
        let generation = UiEvidenceAuthorityGeneration::new(generation);
        let result = host_result_portal_anchor_at(
            request_seed,
            44,
            [1.0, 2.0, 3.0, 4.0],
            &report,
            generation,
        );
        let policy = crate::declaration::UiDeclaredMeasurementPolicyPosture::new(
            Some(crate::declaration::UiDeclaredMeasurementMode::HugHeight),
            Some(crate::declaration::UiDeclaredMeasurementConstraintModifier::Bounded),
            Some(crate::declaration::UiDeclaredMeasurementBasisSource::PortalAnchor),
            Some(
                crate::declaration::UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired,
            ),
            vec![
                crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics,
            ],
        )
        .expect("portal policy admits");
        let basis = admit_measurement_basis(
            synthetic_declaration_identity("portal-basis-successor"),
            crate::graph::UiGraphNodeIdentity::new(91),
            crate::graph::UiGraphWorldProfile::authoritative(),
            generation,
            &policy,
            &[
                MeasurementEvidenceInput::host_capability_report(&report),
                MeasurementEvidenceInput::host_measurement_result(&result),
            ],
        );
        assert!(basis.is_admitted());
        (basis, report)
    }
}
