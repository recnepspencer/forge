use crate::evidence::{
    MeasurementEvidenceInput, UiAllocationNeighborhood, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdgeFamily,
    UiConstraintViewportPlanningInputResult, UiMeasurementBasis, UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture, UiViewportPlanningInputPosture,
    UiViewportPlanningInputSolveOrder,
};

pub(super) fn admit_viewport_planning_input(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    viewport_required: bool,
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<Option<UiConstraintViewportPlanningInputResult>, UiConstraintPropagationDenial> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::ViewportInput) {
        return Ok(None);
    }
    if !viewport_required {
        return Ok(None);
    }

    let neighborhood_identity_digest = neighborhood.identity().identity_digest();
    let contract_identity_digest = neighborhood
        .layout_operator_planning_contract()
        .identity()
        .identity_digest();
    let result = match viewport_source(measurement_basis) {
        Some((source_identity_digest, source_generation_digest, unit, coordinate, rounding)) => {
            let posture = if measurement_basis.generation_compatibility().is_compatible() {
                UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly
            } else {
                UiViewportPlanningInputPosture::IncompatibleMeasurementPosture
            };
            UiConstraintViewportPlanningInputResult::new(
                neighborhood_identity_digest,
                UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
                posture,
                Some(source_identity_digest),
                Some(source_generation_digest),
                Some(unit),
                Some(coordinate),
                Some(rounding),
                true,
            )
        }
        None => UiConstraintViewportPlanningInputResult::new(
            neighborhood_identity_digest,
            UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
            UiViewportPlanningInputPosture::MissingRequiredEvidence,
            None,
            None,
            None,
            None,
            None,
            true,
        ),
    };

    match result.posture() {
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly => Ok(Some(result)),
        UiViewportPlanningInputPosture::MissingRequiredEvidence => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::MissingRequiredViewportPlanningInput,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::ViewportInput),
                result.identity_digest(),
            ))
        }
        UiViewportPlanningInputPosture::IncompatibleMeasurementPosture => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::ViewportInput),
                result.identity_digest(),
            ))
        }
    }
}

fn viewport_source(
    measurement_basis: &UiMeasurementBasis,
) -> Option<(
    u64,
    u64,
    UiMeasurementUnitPosture,
    UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture,
)> {
    let source = measurement_basis
        .dependency_lineage()
        .entries()
        .iter()
        .find(|entry| {
            entry.kind() == crate::evidence::UiMeasurementDependencyLineageKind::HostViewportExtent
        })?;
    let result = measurement_basis
        .evidence_inputs()
        .iter()
        .find_map(|input| match input {
            MeasurementEvidenceInput::HostMeasurementResult(result)
                if matches!(
                    result.value(),
                    crate::evidence::UiMeasurementValue::ViewportExtent(_)
                ) =>
            {
                Some(result)
            }
            _ => None,
        })?;
    Some((
        source.identity_digest(),
        source.generation_digest(),
        result.unit_posture(),
        result.coordinate_space(),
        result.rounding_posture(),
    ))
}
