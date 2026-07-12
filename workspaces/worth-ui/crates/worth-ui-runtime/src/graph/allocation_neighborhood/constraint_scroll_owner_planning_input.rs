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
) -> Result<
    (
        Option<UiConstraintScrollOwnerPlanningInputResult>,
        Option<super::UiGraphScrollPlanningAuthority>,
    ),
    UiConstraintPropagationDenial,
> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::ScrollViewportInput) {
        return Ok((None, None));
    }
    if !scroll_owner_required {
        return Ok((None, None));
    }

    let neighborhood_identity_digest = neighborhood.identity().identity_digest();
    let contract_identity_digest = neighborhood
        .layout_operator_planning_contract()
        .identity()
        .identity_digest();
    let (result, authority) = match scroll_owner_source(measurement_basis) {
        Some((source_identity_digest, source_generation_digest, unit, coordinate, rounding)) => {
            let (host_sources, query_sources, source_counters) =
                admitted_sources(measurement_basis);
            let mint_authority = super::UiGraphConstraintMintAuthority::mint();
            let authority = super::UiGraphScrollPlanningAuthority::seal(
                neighborhood.identity().clone(),
                host_sources.clone(),
                query_sources.clone(),
                source_counters,
            );
            let source_evidence = host_sources
                .iter()
                .map(|witness| {
                    crate::evidence::UiScrollOwnerSourceEvidence::seal_graph(
                        crate::evidence::UiScrollOwnerSourceKind::HostContainerViewport,
                        crate::declaration::stable_text_digest("worth-ui.scroll-source.host")
                            ^ witness.identity_digest().rotate_left(11),
                        &mint_authority,
                    )
                })
                .chain(query_sources.iter().map(|mapping| {
                    crate::evidence::UiScrollOwnerSourceEvidence::seal_graph(
                        crate::evidence::UiScrollOwnerSourceKind::QueryContentExtent,
                        crate::declaration::stable_text_digest("worth-ui.scroll-source.query")
                            ^ mapping.identity_digest().rotate_left(17),
                        &mint_authority,
                    )
                }))
                .collect();
            let posture = if measurement_basis.generation_compatibility().is_compatible() {
                UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly
            } else {
                UiScrollOwnerPlanningInputPosture::IncompatibleMeasurementPosture
            };
            (
                UiConstraintScrollOwnerPlanningInputResult::new(
                    &mint_authority,
                    neighborhood_identity_digest,
                    UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
                    posture,
                    Some(source_identity_digest),
                    Some(source_generation_digest),
                    Some(unit),
                    Some(coordinate),
                    Some(rounding),
                    true,
                    source_evidence,
                    source_counters,
                ),
                Some(authority),
            )
        }
        None => (
            UiConstraintScrollOwnerPlanningInputResult::new(
                &super::UiGraphConstraintMintAuthority::mint(),
                neighborhood_identity_digest,
                UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
                UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence,
                None,
                None,
                None,
                None,
                None,
                true,
                Vec::new(),
                crate::evidence::UiScrollOwnerSourceAdmissionCounters::default(),
            ),
            None,
        ),
    };

    match result.posture() {
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly => {
            Ok((Some(result), authority))
        }
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

fn admitted_sources(
    measurement_basis: &UiMeasurementBasis,
) -> (
    Vec<crate::evidence::UiHostMeasurementAuthorityWitness>,
    Vec<crate::evidence::measurement::basis::UiQueryAllocationTargetMapping>,
    crate::evidence::UiScrollOwnerSourceAdmissionCounters,
) {
    let mut visited = 0_u64;
    let mut host_sources = Vec::new();
    let mut query_sources = Vec::new();
    for input in measurement_basis.evidence_inputs() {
        visited += 1;
        if let Some(result) = input.as_host_measurement_result().filter(|result| {
            result.evidence_category()
                == crate::evidence::UiMeasurementEvidenceCategory::ScrollContainerViewport
        }) {
            host_sources.push(result.authority_witness());
        }
    }
    for (_, mapping) in measurement_basis.query_allocation_mappings() {
        visited += 1;
        if mapping.admits(
            crate::evidence::measurement::basis::UiQueryAllocationPurpose::ScrollContentExtent,
        ) {
            query_sources.push(mapping.clone());
        }
    }
    let before_dedup = (host_sources.len() + query_sources.len()) as u64;
    host_sources.sort_unstable();
    host_sources.dedup();
    query_sources.sort_unstable();
    query_sources.dedup();
    let admitted = (host_sources.len() + query_sources.len()) as u64;
    (
        host_sources,
        query_sources,
        crate::evidence::UiScrollOwnerSourceAdmissionCounters::new(
            visited,
            admitted,
            before_dedup - admitted,
        ),
    )
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
