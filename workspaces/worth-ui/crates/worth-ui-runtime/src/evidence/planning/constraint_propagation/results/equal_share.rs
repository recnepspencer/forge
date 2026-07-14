use crate::declaration::{stable_text_digest, UiDeclaredMeasurementConstraintModifier};

use crate::evidence::{
    UiConstraintAxisScope, UiConstraintEqualShareDistributionPolicy, UiConstraintEqualShareGroup,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintEqualShareSolveOrder {
    AfterSiblingNegotiationBeforeBounds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintEqualSharePosture {
    ExactFractional,
    DeterministicRemainderApplied,
    ZeroShare,
    ZeroAvailableSpace,
    NoAdmittedAvailableSpace,
    SingleSurvivingPeer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiConstraintEqualShareMember {
    member_identity_digest: u64,
    remainder_rank: Option<u16>,
    constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintEqualShareDistributionResult {
    neighborhood_identity_digest: u64,
    group: UiConstraintEqualShareGroup,
    axis_scope: UiConstraintAxisScope,
    policy: UiConstraintEqualShareDistributionPolicy,
    solve_order: UiConstraintEqualShareSolveOrder,
    posture: UiConstraintEqualSharePosture,
    members: Box<[UiConstraintEqualShareMember]>,
    group_identity_digest: u64,
    identity_digest: u64,
}

impl UiConstraintEqualShareMember {
    pub(crate) fn new(
        member_identity_digest: u64,
        remainder_rank: Option<u16>,
        constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    ) -> Self {
        Self {
            member_identity_digest,
            remainder_rank,
            constraint_modifier,
        }
    }

    pub fn member_identity_digest(&self) -> u64 {
        self.member_identity_digest
    }

    pub fn remainder_rank(&self) -> Option<u16> {
        self.remainder_rank
    }

    pub fn constraint_modifier(&self) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.constraint_modifier
    }
}

impl UiConstraintEqualShareDistributionResult {
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        group: UiConstraintEqualShareGroup,
        axis_scope: UiConstraintAxisScope,
        policy: UiConstraintEqualShareDistributionPolicy,
        solve_order: UiConstraintEqualShareSolveOrder,
        posture: UiConstraintEqualSharePosture,
        mut members: Vec<UiConstraintEqualShareMember>,
    ) -> Self {
        members.sort_unstable_by_key(UiConstraintEqualShareMember::member_identity_digest);
        let group_identity_digest = members.iter().fold(
            stable_text_digest("worth-ui.constraint-equal-share-group")
                ^ neighborhood_identity_digest.rotate_left(7)
                ^ equal_share_group_digest(group).rotate_left(13)
                ^ axis_scope_digest(axis_scope).rotate_left(19)
                ^ equal_share_policy_digest(policy).rotate_left(23),
            |digest, member| digest.rotate_left(11) ^ member.member_identity_digest,
        );
        let identity_digest = members.iter().fold(
            stable_text_digest("worth-ui.constraint-equal-share-result")
                ^ group_identity_digest.rotate_left(7)
                ^ equal_share_posture_digest(posture).rotate_left(13)
                ^ equal_share_solve_order_digest(solve_order).rotate_left(19),
            |digest, member| digest.rotate_left(11) ^ member_digest(*member),
        );
        Self {
            neighborhood_identity_digest,
            group,
            axis_scope,
            policy,
            solve_order,
            posture,
            members: members.into_boxed_slice(),
            group_identity_digest,
            identity_digest,
        }
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn group(&self) -> UiConstraintEqualShareGroup {
        self.group
    }

    pub fn axis_scope(&self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub fn policy(&self) -> UiConstraintEqualShareDistributionPolicy {
        self.policy
    }

    pub fn solve_order(&self) -> UiConstraintEqualShareSolveOrder {
        self.solve_order
    }

    pub fn posture(&self) -> UiConstraintEqualSharePosture {
        self.posture
    }

    pub fn members(&self) -> &[UiConstraintEqualShareMember] {
        &self.members
    }

    pub fn group_identity_digest(&self) -> u64 {
        self.group_identity_digest
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn member_digest(member: UiConstraintEqualShareMember) -> u64 {
    stable_text_digest("worth-ui.constraint-equal-share-member")
        ^ member.member_identity_digest.rotate_left(7)
        ^ member
            .remainder_rank
            .map_or_else(
                || stable_text_digest("worth-ui.constraint-equal-share-member.no-remainder"),
                u64::from,
            )
            .rotate_left(13)
        ^ constraint_modifier_digest(member.constraint_modifier).rotate_left(19)
}

fn constraint_modifier_digest(modifier: Option<UiDeclaredMeasurementConstraintModifier>) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("worth-ui.constraint-equal-share.constraint.bounded")
        }
        None => stable_text_digest("worth-ui.constraint-equal-share.constraint.none"),
    }
}

fn equal_share_group_digest(group: UiConstraintEqualShareGroup) -> u64 {
    match group {
        UiConstraintEqualShareGroup::None => {
            stable_text_digest("worth-ui.constraint-equal-share.none")
        }
        UiConstraintEqualShareGroup::StablePeerPrimaryAxis => {
            stable_text_digest("worth-ui.constraint-equal-share.primary-axis")
        }
        UiConstraintEqualShareGroup::StablePeerTwoDimensional => {
            stable_text_digest("worth-ui.constraint-equal-share.two-dimensional")
        }
    }
}

fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

fn equal_share_policy_digest(policy: UiConstraintEqualShareDistributionPolicy) -> u64 {
    match policy {
        UiConstraintEqualShareDistributionPolicy::ExactFractional => {
            stable_text_digest("worth-ui.constraint-equal-share-policy.exact-fractional")
        }
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity => {
            stable_text_digest("worth-ui.constraint-equal-share-policy.deterministic-peer-remainder-left-to-right")
        }
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderCenterOutByStablePeerIdentity => {
            stable_text_digest("worth-ui.constraint-equal-share-policy.deterministic-peer-remainder-center-out")
        }
        UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired => {
            stable_text_digest("worth-ui.constraint-equal-share-policy.deny-if-non-integral-required")
        }
    }
}

fn equal_share_posture_digest(posture: UiConstraintEqualSharePosture) -> u64 {
    stable_text_digest(match posture {
        UiConstraintEqualSharePosture::ExactFractional => {
            "worth-ui.constraint-equal-share-posture.exact-fractional"
        }
        UiConstraintEqualSharePosture::DeterministicRemainderApplied => {
            "worth-ui.constraint-equal-share-posture.deterministic-remainder"
        }
        UiConstraintEqualSharePosture::ZeroShare => {
            "worth-ui.constraint-equal-share-posture.zero-share"
        }
        UiConstraintEqualSharePosture::ZeroAvailableSpace => {
            "worth-ui.constraint-equal-share-posture.zero-available-space"
        }
        UiConstraintEqualSharePosture::NoAdmittedAvailableSpace => {
            "worth-ui.constraint-equal-share-posture.no-admitted-available-space"
        }
        UiConstraintEqualSharePosture::SingleSurvivingPeer => {
            "worth-ui.constraint-equal-share-posture.single-surviving-peer"
        }
    })
}

fn equal_share_solve_order_digest(order: UiConstraintEqualShareSolveOrder) -> u64 {
    match order {
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds => {
            stable_text_digest("worth-ui.constraint-equal-share.after-sibling-before-bounds")
        }
    }
}
