use crate::evidence::{
    UiConstraintPortalAnchorPlanningInputResult, UiMeasurementCoordinateSpace,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture, UiPortalAnchorPlanningInputPosture,
    UiPortalAnchorPlanningInputSolveOrder,
};

#[test]
fn portal_anchor_planning_input_identity_depends_on_source_evidence() {
    let left = UiConstraintPortalAnchorPlanningInputResult::new(
        17,
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::PortalLayer),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let right = UiConstraintPortalAnchorPlanningInputResult::new(
        17,
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(102),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::PortalLayer),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );

    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn portal_anchor_planning_input_identity_depends_on_posture() {
    let admitted = UiConstraintPortalAnchorPlanningInputResult::new(
        19,
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
        Some(101),
        Some(201),
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::PortalLayer),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        true,
    );
    let missing = UiConstraintPortalAnchorPlanningInputResult::new(
        19,
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence,
        None,
        None,
        None,
        None,
        None,
        true,
    );

    assert_ne!(admitted.identity_digest(), missing.identity_digest());
}
