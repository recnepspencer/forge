use crate::declaration::UiDeclaredMeasurementConstraintModifier;
use crate::evidence::{
    UiConstraintAxisScope, UiConstraintIntrinsicSourcePosture,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationGroup,
    UiConstraintSiblingNegotiationMember, UiConstraintSiblingNegotiationMode,
    UiConstraintSiblingNegotiationResult, UiConstraintSiblingNegotiationSolveOrder,
};

#[test]
fn sibling_negotiation_group_identity_ignores_peer_order() {
    let left = UiConstraintSiblingNegotiationGroup::new(
        91,
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        vec![301, 201, 401],
    );
    let right = UiConstraintSiblingNegotiationGroup::new(
        91,
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        vec![401, 301, 201],
    );

    assert_eq!(left, right);
    assert_eq!(left.identity_digest(), right.identity_digest());
}

#[test]
fn sibling_negotiation_result_identity_preserves_member_requirement_distinction() {
    let group = UiConstraintSiblingNegotiationGroup::new(
        99,
        UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis,
        UiConstraintAxisScope::Primary,
        vec![11, 22],
    );
    let left = UiConstraintSiblingNegotiationResult::new(
        group.clone(),
        UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired,
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds,
        None,
        None,
        None,
        vec![
            UiConstraintSiblingNegotiationMember::new(
                11,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
                Some(1001),
                Some(UiConstraintIntrinsicSourcePosture::HostOnly),
            ),
            UiConstraintSiblingNegotiationMember::new(22, None, Some(1002), None),
        ],
    );
    let right = UiConstraintSiblingNegotiationResult::new(
        group,
        UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired,
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds,
        None,
        None,
        None,
        vec![
            UiConstraintSiblingNegotiationMember::new(
                11,
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
                Some(1001),
                Some(UiConstraintIntrinsicSourcePosture::HostOnly),
            ),
            UiConstraintSiblingNegotiationMember::new(22, None, Some(1003), None),
        ],
    );

    assert_ne!(left, right);
    assert_ne!(left.identity_digest(), right.identity_digest());
}
