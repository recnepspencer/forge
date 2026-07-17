use crate::evidence::{
    UiConstraintViewportPlanningInput, UiConstraintViewportPlanningInputResult,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
    UiViewportPlanningInputPosture, UiViewportPlanningInputSolveOrder,
};

fn viewport_input(
    neighborhood_identity_digest: u64,
    source_evidence_identity_digest: Option<u64>,
    posture: UiViewportPlanningInputPosture,
) -> UiConstraintViewportPlanningInput {
    UiConstraintViewportPlanningInput {
        neighborhood_identity_digest,
        solve_order: UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        posture,
        source_evidence_identity_digest,
        source_generation_digest: source_evidence_identity_digest.map(|_| 201),
        unit_posture: source_evidence_identity_digest.map(|_| UiMeasurementUnitPosture::LogicalPx),
        coordinate_space: source_evidence_identity_digest
            .map(|_| UiMeasurementCoordinateSpace::Viewport),
        rounding_posture: source_evidence_identity_digest
            .map(|_| UiMeasurementRoundingPosture::ExactFloat),
        planning_time_only: true,
    }
}

#[test]
fn viewport_planning_input_identity_depends_on_source_evidence() {
    let left = UiConstraintViewportPlanningInputResult::new(viewport_input(
        17,
        Some(101),
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));
    let right = UiConstraintViewportPlanningInputResult::new(viewport_input(
        17,
        Some(102),
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));

    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn viewport_planning_input_identity_depends_on_posture() {
    let admitted = UiConstraintViewportPlanningInputResult::new(viewport_input(
        19,
        Some(101),
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));
    let missing = UiConstraintViewportPlanningInputResult::new(viewport_input(
        19,
        None,
        UiViewportPlanningInputPosture::MissingRequiredEvidence,
    ));

    assert_ne!(admitted.identity_digest(), missing.identity_digest());
}
