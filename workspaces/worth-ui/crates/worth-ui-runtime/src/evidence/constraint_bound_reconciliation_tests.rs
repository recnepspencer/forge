use crate::declaration::UiDeclaredMeasurementConstraintModifier;

use super::{
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope,
    UiConstraintBoundReconciliationMember, UiConstraintBoundReconciliationResult,
    UiConstraintBoundedMinMaxRequirement, UiConstraintSpecialInputPosture,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

#[test]
fn bound_reconciliation_identity_ignores_member_order() {
    let left = UiConstraintBoundReconciliationResult::new(
        71,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent),
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
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
    );
    let right = UiConstraintBoundReconciliationResult::new(
        71,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent),
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
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
    );

    assert_eq!(left.identity_digest(), right.identity_digest());
}

#[test]
fn bound_reconciliation_identity_preserves_posture_distinctions() {
    let clamped = UiConstraintBoundReconciliationResult::new(
        72,
        UiConstraintAxisScope::Primary,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent),
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        vec![UiConstraintBoundReconciliationMember::new(
            810,
            UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
    );
    let contradictory = UiConstraintBoundReconciliationResult::new(
        72,
        UiConstraintAxisScope::Primary,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::ContradictoryMinMax,
        Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown),
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        vec![UiConstraintBoundReconciliationMember::new(
            810,
            UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
    );

    assert_ne!(clamped.identity_digest(), contradictory.identity_digest());
}

#[test]
fn bound_reconciliation_identity_preserves_normalization_and_special_input_state() {
    let viewport_bound = UiConstraintBoundReconciliationResult::new(
        73,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::UnsupportedSpecialInput,
        None,
        UiConstraintSpecialInputPosture::Required,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::ExactFloat),
        vec![UiConstraintBoundReconciliationMember::new(
            820,
            UiConstraintBoundedMinMaxRequirement::BothAxes,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
    );
    let rounded = UiConstraintBoundReconciliationResult::new(
        73,
        UiConstraintAxisScope::Both,
        UiConstraintBoundedMinMaxRequirement::BothAxes,
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        UiBoundReconciliationPosture::UnsupportedRoundingMix,
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent),
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        Some(UiMeasurementUnitPosture::LogicalPx),
        Some(UiMeasurementCoordinateSpace::Viewport),
        Some(UiMeasurementRoundingPosture::HostRounded),
        vec![UiConstraintBoundReconciliationMember::new(
            820,
            UiConstraintBoundedMinMaxRequirement::BothAxes,
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        )],
    );

    assert_ne!(viewport_bound.identity_digest(), rounded.identity_digest());
}
