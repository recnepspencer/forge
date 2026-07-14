use crate::evidence::{
    UiConstraintViewportPlanningInputResult, UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture, UiViewportPlanningInputPosture,
    UiViewportPlanningInputSolveOrder,
};

#[test]
fn viewport_planning_input_identity_depends_on_source_evidence() {
    let left = UiConstraintViewportPlanningInputResult::new(
        17,
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let right = UiConstraintViewportPlanningInputResult::new(
        17,
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(102),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );

    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn viewport_planning_input_identity_depends_on_posture() {
    let admitted = UiConstraintViewportPlanningInputResult::new(
        19,
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let missing = UiConstraintViewportPlanningInputResult::new(
        19,
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiViewportPlanningInputPosture::MissingRequiredEvidence,
        None,
        None,
        None,
        None,
        None,
        true,
    );

    assert_ne!(admitted.identity_digest(), missing.identity_digest());
}
