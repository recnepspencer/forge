use super::posture_for_equal_share;
use crate::evidence::{
    UiAllocationConstraintSummary, UiAllocationConstraintSummaryInput,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope, UiConstraintBoundedMinMaxRequirement,
    UiConstraintEqualShareGroup, UiConstraintEqualSharePosture,
    UiConstraintResizePermissionPosture, UiConstraintSiblingNegotiationMode,
    UiConstraintSpecialInputPosture,
};

#[test]
fn admitted_zero_available_space_maps_to_typed_equal_share_posture() {
    let summary = UiAllocationConstraintSummary::new(UiAllocationConstraintSummaryInput {
        incoming_available_space: Some(UiConstraintAxisScope::Both),
        incoming_available_space_posture: Some(
            UiConstraintAvailableSpacePosture::AdmittedZeroExtent,
        ),
        intrinsic_contribution_requirements: Some(UiConstraintAxisScope::Both),
        sibling_negotiation_mode: UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional,
        equal_share_group: UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        bounded_min_max_requirements: UiConstraintBoundedMinMaxRequirement::BothAxes,
        viewport_requirement: UiConstraintSpecialInputPosture::NotRequired,
        scroll_owner_requirement: UiConstraintSpecialInputPosture::NotRequired,
        portal_anchor_requirement: UiConstraintSpecialInputPosture::NotRequired,
        resize_permission_posture: UiConstraintResizePermissionPosture::None,
        unit_posture: None,
        coordinate_space: None,
        rounding_posture: None,
    });

    assert_eq!(
        posture_for_equal_share(summary, UiConstraintAxisScope::Both, 2),
        UiConstraintEqualSharePosture::ZeroAvailableSpace
    );
}
