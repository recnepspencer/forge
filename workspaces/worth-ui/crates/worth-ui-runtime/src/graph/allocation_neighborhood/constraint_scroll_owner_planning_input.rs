use crate::evidence::{
    MeasurementEvidenceInput, UiAllocationNeighborhood, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdgeFamily,
    UiConstraintScrollOwnerPlanningInputResult, UiMeasurementBasis, UiMeasurementCoordinateSpace,
    UiMeasurementDependencyLineageKind, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
    UiMeasurementValue, UiScrollOwnerPlanningInputPosture, UiScrollOwnerPlanningInputSolveOrder,
};

pub(super) fn admit_scroll_owner_planning_input(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    scroll_owner_required: bool,
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<Option<UiConstraintScrollOwnerPlanningInputResult>, UiConstraintPropagationDenial> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::ScrollViewportInput) {
        return Ok(None);
    }
    if !scroll_owner_required {
        return Ok(None);
    }

    let neighborhood_identity_digest = neighborhood.identity().identity_digest();
    let contract_identity_digest = neighborhood
        .layout_operator_planning_contract()
        .identity()
        .identity_digest();
    let result = match scroll_owner_source(measurement_basis) {
        Some((source_identity_digest, source_generation_digest, unit, coordinate, rounding)) => {
            let posture = if measurement_basis.generation_compatibility().is_compatible() {
                UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly
            } else {
                UiScrollOwnerPlanningInputPosture::IncompatibleMeasurementPosture
            };
            UiConstraintScrollOwnerPlanningInputResult::new(
                neighborhood_identity_digest,
                UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
                posture,
                Some(source_identity_digest),
                Some(source_generation_digest),
                Some(unit),
                Some(coordinate),
                Some(rounding),
                true,
            )
        }
        None => UiConstraintScrollOwnerPlanningInputResult::new(
            neighborhood_identity_digest,
            UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
            UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence,
            None,
            None,
            None,
            None,
            None,
            true,
        ),
    };

    match result.posture() {
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly => Ok(Some(result)),
        UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::MissingRequiredScrollOwnerPlanningInput,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::ScrollViewportInput),
                result.identity_digest(),
            ))
        }
        UiScrollOwnerPlanningInputPosture::IncompatibleMeasurementPosture => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::ScrollViewportInput),
                result.identity_digest(),
            ))
        }
    }
}

fn scroll_owner_source(
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
            entry.kind() == UiMeasurementDependencyLineageKind::HostScrollContainerViewport
        })?;
    let result = measurement_basis
        .evidence_inputs()
        .iter()
        .find_map(|input| match input {
            MeasurementEvidenceInput::HostMeasurementResult(result)
                if matches!(
                    result.value(),
                    UiMeasurementValue::ScrollContainerViewport(_)
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
