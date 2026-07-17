use crate::declaration::{stable_text_digest, UiDeclaredMeasurementConstraintModifier};

use crate::evidence::{
    UiConstraintIntrinsicSourcePosture, UiConstraintSiblingNegotiationGroup,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintSiblingNegotiationFixedPointPolicy {
    NotRequired,
    AdmittedStablePeerMutual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintSiblingNegotiationSolveOrder {
    BeforeEqualShareAndBounds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiConstraintSiblingNegotiationMember {
    member_identity_digest: u64,
    constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    intrinsic_contribution_identity_digest: Option<u64>,
    intrinsic_source_posture: Option<UiConstraintIntrinsicSourcePosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintSiblingNegotiationResult {
    group: UiConstraintSiblingNegotiationGroup,
    fixed_point_policy: UiConstraintSiblingNegotiationFixedPointPolicy,
    solve_order: UiConstraintSiblingNegotiationSolveOrder,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
    members: Box<[UiConstraintSiblingNegotiationMember]>,
    identity_digest: u64,
}

impl UiConstraintSiblingNegotiationMember {
    pub(crate) fn new(
        member_identity_digest: u64,
        constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
        intrinsic_contribution_identity_digest: Option<u64>,
        intrinsic_source_posture: Option<UiConstraintIntrinsicSourcePosture>,
    ) -> Self {
        Self {
            member_identity_digest,
            constraint_modifier,
            intrinsic_contribution_identity_digest,
            intrinsic_source_posture,
        }
    }

    pub fn member_identity_digest(&self) -> u64 {
        self.member_identity_digest
    }

    pub fn constraint_modifier(&self) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.constraint_modifier
    }

    pub fn intrinsic_contribution_identity_digest(&self) -> Option<u64> {
        self.intrinsic_contribution_identity_digest
    }

    pub fn intrinsic_source_posture(&self) -> Option<UiConstraintIntrinsicSourcePosture> {
        self.intrinsic_source_posture
    }
}

impl UiConstraintSiblingNegotiationResult {
    pub(crate) fn new(
        group: UiConstraintSiblingNegotiationGroup,
        fixed_point_policy: UiConstraintSiblingNegotiationFixedPointPolicy,
        solve_order: UiConstraintSiblingNegotiationSolveOrder,
        unit_posture: Option<UiMeasurementUnitPosture>,
        coordinate_space: Option<UiMeasurementCoordinateSpace>,
        rounding_posture: Option<UiMeasurementRoundingPosture>,
        mut members: Vec<UiConstraintSiblingNegotiationMember>,
    ) -> Self {
        members.sort_unstable_by_key(UiConstraintSiblingNegotiationMember::member_identity_digest);
        let identity_digest = members.iter().fold(
            stable_text_digest("worth-ui.constraint-sibling-negotiation-result")
                ^ group.identity_digest().rotate_left(7)
                ^ fixed_point_policy_digest(fixed_point_policy).rotate_left(13)
                ^ solve_order_digest(solve_order).rotate_left(19)
                ^ unit_posture_digest(unit_posture).rotate_left(23)
                ^ coordinate_space_digest(coordinate_space).rotate_left(29)
                ^ rounding_posture_digest(rounding_posture).rotate_left(31),
            |digest, member| digest.rotate_left(11) ^ member_digest(*member),
        );
        Self {
            group,
            fixed_point_policy,
            solve_order,
            unit_posture,
            coordinate_space,
            rounding_posture,
            members: members.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn group(&self) -> &UiConstraintSiblingNegotiationGroup {
        &self.group
    }

    pub fn fixed_point_policy(&self) -> UiConstraintSiblingNegotiationFixedPointPolicy {
        self.fixed_point_policy
    }

    pub fn solve_order(&self) -> UiConstraintSiblingNegotiationSolveOrder {
        self.solve_order
    }

    pub fn unit_posture(&self) -> Option<UiMeasurementUnitPosture> {
        self.unit_posture
    }

    pub fn coordinate_space(&self) -> Option<UiMeasurementCoordinateSpace> {
        self.coordinate_space
    }

    pub fn rounding_posture(&self) -> Option<UiMeasurementRoundingPosture> {
        self.rounding_posture
    }

    pub fn members(&self) -> &[UiConstraintSiblingNegotiationMember] {
        &self.members
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn member_digest(member: UiConstraintSiblingNegotiationMember) -> u64 {
    stable_text_digest("worth-ui.constraint-sibling-negotiation-member")
        ^ member.member_identity_digest.rotate_left(7)
        ^ constraint_modifier_digest(member.constraint_modifier).rotate_left(13)
        ^ member
            .intrinsic_contribution_identity_digest
            .unwrap_or_else(|| stable_text_digest("worth-ui.constraint-sibling-negotiation.none"))
            .rotate_left(19)
        ^ intrinsic_source_posture_digest(member.intrinsic_source_posture).rotate_left(23)
}

fn constraint_modifier_digest(modifier: Option<UiDeclaredMeasurementConstraintModifier>) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.bounded")
        }
        None => stable_text_digest("worth-ui.constraint-sibling-negotiation.unbounded"),
    }
}

fn intrinsic_source_posture_digest(posture: Option<UiConstraintIntrinsicSourcePosture>) -> u64 {
    match posture {
        Some(UiConstraintIntrinsicSourcePosture::QueryOnly) => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.query-only")
        }
        Some(UiConstraintIntrinsicSourcePosture::HostOnly) => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.host-only")
        }
        Some(UiConstraintIntrinsicSourcePosture::QueryAndHost) => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.query-and-host")
        }
        None => stable_text_digest("worth-ui.constraint-sibling-negotiation.no-intrinsic"),
    }
}

fn fixed_point_policy_digest(policy: UiConstraintSiblingNegotiationFixedPointPolicy) -> u64 {
    match policy {
        UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.not-required")
        }
        UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.admitted-fixed-point")
        }
    }
}

fn solve_order_digest(order: UiConstraintSiblingNegotiationSolveOrder) -> u64 {
    match order {
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds => {
            stable_text_digest("worth-ui.constraint-sibling-negotiation.before-equal-share-bounds")
        }
    }
}

fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => "worth-ui.constraint-unit.logical-px",
        Some(UiMeasurementUnitPosture::PhysicalPx) => "worth-ui.constraint-unit.physical-px",
        Some(UiMeasurementUnitPosture::UnitlessScale) => "worth-ui.constraint-unit.unitless-scale",
        None => "worth-ui.constraint-unit.none",
    })
}

fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
    stable_text_digest(match space {
        Some(UiMeasurementCoordinateSpace::Viewport) => "worth-ui.constraint-coordinate.viewport",
        Some(UiMeasurementCoordinateSpace::Window) => "worth-ui.constraint-coordinate.window",
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal) => {
            "worth-ui.constraint-coordinate.graph-node-local"
        }
        Some(UiMeasurementCoordinateSpace::HostSurface) => {
            "worth-ui.constraint-coordinate.host-surface"
        }
        Some(UiMeasurementCoordinateSpace::PortalLayer) => {
            "worth-ui.constraint-coordinate.portal-layer"
        }
        None => "worth-ui.constraint-coordinate.none",
    })
}

fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementRoundingPosture::ExactFloat) => {
            "worth-ui.constraint-rounding.exact-float"
        }
        Some(UiMeasurementRoundingPosture::HostRounded) => {
            "worth-ui.constraint-rounding.host-rounded"
        }
        Some(UiMeasurementRoundingPosture::RuntimeRounded) => {
            "worth-ui.constraint-rounding.runtime-rounded"
        }
        Some(UiMeasurementRoundingPosture::DeferredToAllocation) => {
            "worth-ui.constraint-rounding.deferred-to-allocation"
        }
        None => "worth-ui.constraint-rounding.none",
    })
}
