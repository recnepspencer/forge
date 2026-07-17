use crate::declaration::UiDeclaredMeasurementConstraintModifier;

use crate::evidence::{
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope, UiConstraintBoundReconciliationInput,
    UiConstraintBoundReconciliationMember, UiConstraintBoundReconciliationResult,
    UiConstraintBoundedMinMaxRequirement, UiConstraintSpecialInputPosture,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

fn bound_input(
    neighborhood_identity_digest: u64,
    axis_scope: UiConstraintAxisScope,
    requirement: UiConstraintBoundedMinMaxRequirement,
    posture: UiBoundReconciliationPosture,
    members: Vec<UiConstraintBoundReconciliationMember>,
    configure: impl FnOnce(&mut UiConstraintBoundReconciliationInput),
) -> UiConstraintBoundReconciliationInput {
    let mut input = UiConstraintBoundReconciliationInput {
        neighborhood_identity_digest,
        axis_scope,
        requirement,
        solve_order: UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        posture,
        incoming_available_space_posture: Some(
            UiConstraintAvailableSpacePosture::AdmittedPositiveExtent,
        ),
        viewport_requirement: UiConstraintSpecialInputPosture::NotRequired,
        scroll_owner_requirement: UiConstraintSpecialInputPosture::NotRequired,
        portal_anchor_requirement: UiConstraintSpecialInputPosture::NotRequired,
        unit_posture: Some(UiMeasurementUnitPosture::LogicalPx),
        coordinate_space: Some(UiMeasurementCoordinateSpace::Viewport),
        rounding_posture: Some(UiMeasurementRoundingPosture::ExactFloat),
        members,
    };
    configure(&mut input);
    input
}

#[test]
fn bound_reconciliation_identity_ignores_member_order() {
    let left = UiConstraintBoundReconciliationResult::new(bound_input(
        71,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        vec![
            UiConstraintBoundReconciliationMember::new(
                900,
                UiConstraintBoundedMinMaxRequirement::BothAxes,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            ),
            UiConstraintBoundReconciliationMember::new(
                800,
                UiConstraintBoundedMinMaxRequirement::BothAxes,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            ),
        ],
        |_| {},
    ));
    let right = UiConstraintBoundReconciliationResult::new(bound_input(
        71,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        vec![
            UiConstraintBoundReconciliationMember::new(
                800,
                UiConstraintBoundedMinMaxRequirement::BothAxes,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            ),
            UiConstraintBoundReconciliationMember::new(
                900,
                UiConstraintBoundedMinMaxRequirement::BothAxes,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            ),
        ],
        |_| {},
    ));

    assert_eq!(left.identity_digest(), right.identity_digest());
}

#[test]
fn bound_reconciliation_identity_preserves_posture_distinctions() {
    let clamped = UiConstraintBoundReconciliationResult::new(bound_input(
        72,
        UiConstraintAxisScope::Primary,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        vec![UiConstraintBoundReconciliationMember::new(
            810,
            UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
        |_| {},
    ));
    let contradictory = UiConstraintBoundReconciliationResult::new(bound_input(
        72,
        UiConstraintAxisScope::Primary,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        UiBoundReconciliationPosture::ContradictoryMinMax,
        vec![UiConstraintBoundReconciliationMember::new(
            810,
            UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
        |input| {
            input.incoming_available_space_posture =
                Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown);
        },
    ));

    assert_ne!(clamped.identity_digest(), contradictory.identity_digest());
}

#[test]
fn bound_reconciliation_identity_preserves_normalization_and_special_input_state() {
    let viewport_bound = UiConstraintBoundReconciliationResult::new(bound_input(
        73,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationPosture::UnsupportedSpecialInput,
        vec![UiConstraintBoundReconciliationMember::new(
            820,
            UiConstraintBoundedMinMaxRequirement::BothAxes,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
        |input| {
            input.incoming_available_space_posture = None;
            input.viewport_requirement = UiConstraintSpecialInputPosture::Required;
        },
    ));
    let rounded = UiConstraintBoundReconciliationResult::new(bound_input(
        73,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationPosture::UnsupportedRoundingMix,
        vec![UiConstraintBoundReconciliationMember::new(
            820,
            UiConstraintBoundedMinMaxRequirement::BothAxes,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
        |input| input.rounding_posture = Some(UiMeasurementRoundingPosture::HostRounded),
    ));

    assert_ne!(viewport_bound.identity_digest(), rounded.identity_digest());
}
