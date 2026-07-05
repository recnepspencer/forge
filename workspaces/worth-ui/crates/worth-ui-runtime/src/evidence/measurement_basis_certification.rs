use super::{
    MeasurementEvidenceInput, UiMeasurementBasis, UiMeasurementDependencyLineageKind,
    UiMeasurementEvidenceCategory, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMeasurementBasisDeterminismPosture {
    Equivalent,
    Divergent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementBasisCertificationReport {
    determinism_posture: UiMeasurementBasisDeterminismPosture,
    first_compatibility: UiMeasurementGenerationCompatibility,
    second_compatibility: UiMeasurementGenerationCompatibility,
    basis_postures_match: bool,
    evidence_inputs_match: bool,
    dependency_maps_match: bool,
    first_lineage_is_narrow: bool,
    second_lineage_is_narrow: bool,
    first_neighborhood_is_narrow: bool,
    second_neighborhood_is_narrow: bool,
}

pub fn certify_measurement_basis_determinism(
    first: &UiMeasurementBasis,
    second: &UiMeasurementBasis,
) -> UiMeasurementBasisCertificationReport {
    UiMeasurementBasisCertificationReport {
        determinism_posture: determinism_posture(first, second),
        first_compatibility: first.generation_compatibility().clone(),
        second_compatibility: second.generation_compatibility().clone(),
        basis_postures_match: first.basis_posture() == second.basis_posture(),
        evidence_inputs_match: canonical_evidence_input_digests(first)
            == canonical_evidence_input_digests(second),
        dependency_maps_match: first.dependency_map() == second.dependency_map(),
        first_lineage_is_narrow: lineage_is_narrow(first),
        second_lineage_is_narrow: lineage_is_narrow(second),
        first_neighborhood_is_narrow: neighborhood_is_narrow(first.neighborhood_class_hint()),
        second_neighborhood_is_narrow: neighborhood_is_narrow(second.neighborhood_class_hint()),
    }
}

impl UiMeasurementBasisCertificationReport {
    pub fn determinism_posture(&self) -> UiMeasurementBasisDeterminismPosture {
        self.determinism_posture
    }

    pub fn first_compatibility(&self) -> &UiMeasurementGenerationCompatibility {
        &self.first_compatibility
    }

    pub fn second_compatibility(&self) -> &UiMeasurementGenerationCompatibility {
        &self.second_compatibility
    }

    pub fn basis_postures_match(&self) -> bool {
        self.basis_postures_match
    }

    pub fn evidence_inputs_match(&self) -> bool {
        self.evidence_inputs_match
    }

    pub fn dependency_maps_match(&self) -> bool {
        self.dependency_maps_match
    }

    pub fn lineage_is_narrow(&self) -> bool {
        self.first_lineage_is_narrow && self.second_lineage_is_narrow
    }

    pub fn first_lineage_is_narrow(&self) -> bool {
        self.first_lineage_is_narrow
    }

    pub fn second_lineage_is_narrow(&self) -> bool {
        self.second_lineage_is_narrow
    }

    pub fn neighborhoods_are_narrow(&self) -> bool {
        self.first_neighborhood_is_narrow && self.second_neighborhood_is_narrow
    }
}

fn determinism_posture(
    first: &UiMeasurementBasis,
    second: &UiMeasurementBasis,
) -> UiMeasurementBasisDeterminismPosture {
    if first.identity_digest() == second.identity_digest()
        && first.generation() == second.generation()
        && first.basis_posture() == second.basis_posture()
        && first.generation_compatibility() == second.generation_compatibility()
        && first.denial_posture() == second.denial_posture()
        && canonical_evidence_input_digests(first) == canonical_evidence_input_digests(second)
        && first.dependency_lineage() == second.dependency_lineage()
        && first.dependency_map() == second.dependency_map()
        && first.neighborhood_class_hint() == second.neighborhood_class_hint()
    {
        UiMeasurementBasisDeterminismPosture::Equivalent
    } else {
        UiMeasurementBasisDeterminismPosture::Divergent
    }
}

fn canonical_evidence_input_digests(basis: &UiMeasurementBasis) -> Vec<u64> {
    let mut digests = basis
        .evidence_inputs()
        .iter()
        .map(MeasurementEvidenceInput::identity_digest)
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

fn lineage_is_narrow(basis: &UiMeasurementBasis) -> bool {
    let inputs = basis.evidence_inputs();
    basis.dependency_lineage().entries().len() <= inputs.len()
        && basis
            .dependency_lineage()
            .entries()
            .iter()
            .all(|entry| lineage_entry_has_supporting_input(entry.kind(), inputs))
}

fn lineage_entry_has_supporting_input(
    kind: UiMeasurementDependencyLineageKind,
    inputs: &[MeasurementEvidenceInput],
) -> bool {
    match kind {
        UiMeasurementDependencyLineageKind::QueryScrollContentExtent => inputs
            .iter()
            .any(|input| matches!(input, MeasurementEvidenceInput::QueryProjectionFact(_))),
        UiMeasurementDependencyLineageKind::HostFontMetrics => {
            has_host_result(inputs, UiMeasurementEvidenceCategory::FontMetrics)
        }
        UiMeasurementDependencyLineageKind::HostViewportExtent => {
            has_host_result(inputs, UiMeasurementEvidenceCategory::ViewportExtent)
        }
        UiMeasurementDependencyLineageKind::HostPortalAnchorRect => {
            has_host_result(inputs, UiMeasurementEvidenceCategory::PortalAnchorRect)
        }
        UiMeasurementDependencyLineageKind::HostScrollContainerViewport => has_host_result(
            inputs,
            UiMeasurementEvidenceCategory::ScrollContainerViewport,
        ),
    }
}

fn has_host_result(
    inputs: &[MeasurementEvidenceInput],
    category: UiMeasurementEvidenceCategory,
) -> bool {
    inputs.iter().any(|input| {
        matches!(
            input,
            MeasurementEvidenceInput::HostMeasurementResult(result)
                if result.evidence_category() == category
        )
    })
}

fn neighborhood_is_narrow(hint: UiMeasurementNeighborhoodClassHint) -> bool {
    matches!(
        hint,
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
            | UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
            | UiMeasurementNeighborhoodClassHint::ViewportDependency
            | UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
            | UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
    )
}
