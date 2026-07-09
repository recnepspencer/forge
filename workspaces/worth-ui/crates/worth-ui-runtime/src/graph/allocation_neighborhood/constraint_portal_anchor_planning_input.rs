use crate::evidence::{
    MeasurementEvidenceInput, UiAllocationNeighborhood,
    UiConstraintPortalAnchorPlanningInputResult, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdgeFamily, UiMeasurementBasis,
    UiMeasurementCoordinateSpace, UiMeasurementDependencyLineageKind, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture, UiMeasurementValue, UiPortalAnchorPlanningInputPosture,
    UiPortalAnchorPlanningInputSolveOrder,
};

pub(super) fn admit_portal_anchor_planning_input(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    portal_anchor_required: bool,
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<Option<UiConstraintPortalAnchorPlanningInputResult>, UiConstraintPropagationDenial> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::PortalAnchorInput) {
        return Ok(None);
    }
    if !portal_anchor_required {
        return Ok(None);
    }

    let neighborhood_identity_digest = neighborhood.identity().identity_digest();
    let contract_identity_digest = neighborhood
        .layout_operator_planning_contract()
        .identity()
        .identity_digest();
    let result = match portal_anchor_source(measurement_basis) {
        Some((source_identity_digest, source_generation_digest, unit, coordinate, rounding)) => {
            let posture = if measurement_basis.generation_compatibility().is_compatible() {
                UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly
            } else {
                UiPortalAnchorPlanningInputPosture::IncompatibleMeasurementPosture
            };
            UiConstraintPortalAnchorPlanningInputResult::new(
                neighborhood_identity_digest,
                UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
                posture,
                Some(source_identity_digest),
                Some(source_generation_digest),
                Some(unit),
                Some(coordinate),
                Some(rounding),
                true,
            )
        }
        None => UiConstraintPortalAnchorPlanningInputResult::new(
            neighborhood_identity_digest,
            UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
            UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence,
            None,
            None,
            None,
            None,
            None,
            true,
        ),
    };

    match result.posture() {
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly => Ok(Some(result)),
        UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::MissingRequiredPortalAnchorPlanningInput,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::PortalAnchorInput),
                result.identity_digest(),
            ))
        }
        UiPortalAnchorPlanningInputPosture::IncompatibleMeasurementPosture => {
            Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(UiConstraintPropagationEdgeFamily::PortalAnchorInput),
                result.identity_digest(),
            ))
        }
    }
}

fn portal_anchor_source(
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
        .find(|entry| entry.kind() == UiMeasurementDependencyLineageKind::HostPortalAnchorRect)?;
    let result = measurement_basis
        .evidence_inputs()
        .iter()
        .find_map(|input| match input {
            MeasurementEvidenceInput::HostMeasurementResult(result)
                if matches!(result.value(), UiMeasurementValue::PortalAnchorRect(_)) =>
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
