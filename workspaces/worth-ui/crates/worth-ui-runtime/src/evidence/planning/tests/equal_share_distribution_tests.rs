use crate::declaration::UiDeclaredMeasurementConstraintModifier;

use crate::evidence::{
    UiConstraintAxisScope, UiConstraintEqualShareDistributionPolicy,
    UiConstraintEqualShareDistributionResult, UiConstraintEqualShareGroup,
    UiConstraintEqualShareMember, UiConstraintEqualSharePosture, UiConstraintEqualShareSolveOrder,
};

#[test]
fn equal_share_result_identity_ignores_peer_order() {
    let left = UiConstraintEqualShareDistributionResult::new(
        71,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::DeterministicRemainderApplied,
        vec![
            UiConstraintEqualShareMember::new(900, Some(1), None),
            UiConstraintEqualShareMember::new(800, Some(0), None),
        ],
    );
    let right = UiConstraintEqualShareDistributionResult::new(
        71,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::DeterministicRemainderApplied,
        vec![
            UiConstraintEqualShareMember::new(800, Some(0), None),
            UiConstraintEqualShareMember::new(900, Some(1), None),
        ],
    );

    assert_eq!(left.group_identity_digest(), right.group_identity_digest());
    assert_eq!(left.identity_digest(), right.identity_digest());
}

#[test]
fn equal_share_result_identity_preserves_posture_and_remainder_contract() {
    let deterministic = UiConstraintEqualShareDistributionResult::new(
        72,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::DeterministicRemainderApplied,
        vec![
            UiConstraintEqualShareMember::new(
                810,
                Some(0),
                Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            ),
            UiConstraintEqualShareMember::new(910, Some(1), None),
        ],
    );
    let single_survivor = UiConstraintEqualShareDistributionResult::new(
        72,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::SingleSurvivingPeer,
        vec![UiConstraintEqualShareMember::new(810, None, None)],
    );

    assert_ne!(
        deterministic.identity_digest(),
        single_survivor.identity_digest()
    );
}

#[test]
fn equal_share_result_identity_preserves_policy_distinctions() {
    let left_to_right = UiConstraintEqualShareDistributionResult::new(
        73,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::DeterministicRemainderApplied,
        vec![
            UiConstraintEqualShareMember::new(820, Some(0), None),
            UiConstraintEqualShareMember::new(920, Some(1), None),
        ],
    );
    let center_out = UiConstraintEqualShareDistributionResult::new(
        73,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional,
        UiConstraintAxisScope::Both,
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderCenterOutByStablePeerIdentity,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        UiConstraintEqualSharePosture::DeterministicRemainderApplied,
        vec![
            UiConstraintEqualShareMember::new(820, Some(1), None),
            UiConstraintEqualShareMember::new(920, Some(0), None),
        ],
    );

    assert_ne!(
        left_to_right.group_identity_digest(),
        center_out.group_identity_digest()
    );
    assert_ne!(
        left_to_right.identity_digest(),
        center_out.identity_digest()
    );
}
