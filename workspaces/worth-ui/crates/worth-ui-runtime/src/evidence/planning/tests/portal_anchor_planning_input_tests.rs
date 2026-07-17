use crate::evidence::{
    UiConstraintPortalAnchorPlanningInput, UiConstraintPortalAnchorPlanningInputResult,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
    UiPortalAnchorPlanningInputPosture, UiPortalAnchorPlanningInputSolveOrder,
};

fn portal_input(
    neighborhood_identity_digest: u64,
    source_evidence_identity_digest: Option<u64>,
    posture: UiPortalAnchorPlanningInputPosture,
) -> UiConstraintPortalAnchorPlanningInput {
    UiConstraintPortalAnchorPlanningInput {
        neighborhood_identity_digest,
        solve_order: UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies,
        posture,
        source_evidence_identity_digest,
        source_generation_digest: source_evidence_identity_digest.map(|_| 201),
        unit_posture: source_evidence_identity_digest.map(|_| UiMeasurementUnitPosture::LogicalPx),
        coordinate_space: source_evidence_identity_digest
            .map(|_| UiMeasurementCoordinateSpace::PortalLayer),
        rounding_posture: source_evidence_identity_digest
            .map(|_| UiMeasurementRoundingPosture::ExactFloat),
        planning_time_only: true,
    }
}

#[test]
fn portal_anchor_planning_input_identity_depends_on_source_evidence() {
    let left = UiConstraintPortalAnchorPlanningInputResult::new(portal_input(
        17,
        Some(101),
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));
    let right = UiConstraintPortalAnchorPlanningInputResult::new(portal_input(
        17,
        Some(102),
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));

    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn portal_anchor_planning_input_identity_depends_on_posture() {
    let admitted = UiConstraintPortalAnchorPlanningInputResult::new(portal_input(
        19,
        Some(101),
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
    ));
    let missing = UiConstraintPortalAnchorPlanningInputResult::new(portal_input(
        19,
        None,
        UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence,
    ));

    assert_ne!(admitted.identity_digest(), missing.identity_digest());
}
