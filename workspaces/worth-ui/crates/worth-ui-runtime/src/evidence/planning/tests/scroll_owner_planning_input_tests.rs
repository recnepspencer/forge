use crate::evidence::{
    UiConstraintScrollOwnerPlanningInputResult, UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture, UiScrollOwnerPlanningInputPosture,
    UiScrollOwnerPlanningInputSolveOrder,
};

#[test]
fn scroll_owner_planning_input_identity_depends_on_source_evidence() {
    let left = UiConstraintScrollOwnerPlanningInputResult::new(
        17,
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let right = UiConstraintScrollOwnerPlanningInputResult::new(
        17,
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(102),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );

    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn scroll_owner_planning_input_identity_depends_on_posture() {
    let admitted = UiConstraintScrollOwnerPlanningInputResult::new(
        19,
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let missing = UiConstraintScrollOwnerPlanningInputResult::new(
        19,
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence,
        None,
        None,
        None,
        None,
        None,
        true,
    );

    assert_ne!(admitted.identity_digest(), missing.identity_digest());
}
