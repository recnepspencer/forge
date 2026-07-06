use crate::declaration::{stable_text_digest, UiDeclaredMeasurementConstraintModifier};

use super::{
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope, UiConstraintBoundedMinMaxRequirement,
    UiConstraintSpecialInputPosture, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiBoundReconciliationSolveOrder {
    AfterEqualShareBeforePlanCloseout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiBoundReconciliationPosture {
    SatisfiedWithoutClamp,
    SatisfiedWithDeclaredClamp,
    Underconstrained,
    Overconstrained,
    ContradictoryMinMax,
    UnsupportedUnitMix,
    UnsupportedRoundingMix,
    Cyclic,
    StaleInput,
    UnsupportedSpecialInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiConstraintBoundReconciliationMember {
    member_identity_digest: u64,
    bounded_requirement: UiConstraintBoundedMinMaxRequirement,
    constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintBoundReconciliationResult {
    neighborhood_identity_digest: u64,
    axis_scope: UiConstraintAxisScope,
    requirement: UiConstraintBoundedMinMaxRequirement,
    solve_order: UiBoundReconciliationSolveOrder,
    posture: UiBoundReconciliationPosture,
    incoming_available_space_posture: Option<UiConstraintAvailableSpacePosture>,
    viewport_requirement: UiConstraintSpecialInputPosture,
    scroll_owner_requirement: UiConstraintSpecialInputPosture,
    portal_anchor_requirement: UiConstraintSpecialInputPosture,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
    members: Box<[UiConstraintBoundReconciliationMember]>,
    identity_digest: u64,
}

impl UiConstraintBoundReconciliationMember {
    pub(crate) fn new(
        member_identity_digest: u64,
        bounded_requirement: UiConstraintBoundedMinMaxRequirement,
        constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    ) -> Self {
        Self {
            member_identity_digest,
            bounded_requirement,
            constraint_modifier,
        }
    }

    pub fn member_identity_digest(&self) -> u64 {
        self.member_identity_digest
    }

    pub fn bounded_requirement(&self) -> UiConstraintBoundedMinMaxRequirement {
        self.bounded_requirement
    }

    pub fn constraint_modifier(&self) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.constraint_modifier
    }
}

impl UiConstraintBoundReconciliationResult {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        axis_scope: UiConstraintAxisScope,
        requirement: UiConstraintBoundedMinMaxRequirement,
        solve_order: UiBoundReconciliationSolveOrder,
        posture: UiBoundReconciliationPosture,
        incoming_available_space_posture: Option<UiConstraintAvailableSpacePosture>,
        viewport_requirement: UiConstraintSpecialInputPosture,
        scroll_owner_requirement: UiConstraintSpecialInputPosture,
        portal_anchor_requirement: UiConstraintSpecialInputPosture,
        unit_posture: Option<UiMeasurementUnitPosture>,
        coordinate_space: Option<UiMeasurementCoordinateSpace>,
        rounding_posture: Option<UiMeasurementRoundingPosture>,
        mut members: Vec<UiConstraintBoundReconciliationMember>,
    ) -> Self {
        members.sort_unstable_by_key(UiConstraintBoundReconciliationMember::member_identity_digest);
        let identity_digest = members.iter().fold(
            stable_text_digest("worth-ui.constraint-bound-reconciliation-result")
                ^ neighborhood_identity_digest.rotate_left(7)
                ^ axis_scope_digest(axis_scope).rotate_left(13)
                ^ bounded_requirement_digest(requirement).rotate_left(19)
                ^ posture_digest(posture).rotate_left(23)
                ^ solve_order_digest(solve_order).rotate_left(29)
                ^ available_space_posture_digest(incoming_available_space_posture).rotate_left(31)
                ^ special_input_posture_digest(viewport_requirement).rotate_left(37)
                ^ special_input_posture_digest(scroll_owner_requirement).rotate_left(41)
                ^ special_input_posture_digest(portal_anchor_requirement).rotate_left(43)
                ^ unit_posture_digest(unit_posture).rotate_left(47)
                ^ coordinate_space_digest(coordinate_space).rotate_left(53)
                ^ rounding_posture_digest(rounding_posture).rotate_left(59),
            |digest, member| digest.rotate_left(11) ^ member_digest(*member),
        );
        Self {
            neighborhood_identity_digest,
            axis_scope,
            requirement,
            solve_order,
            posture,
            incoming_available_space_posture,
            viewport_requirement,
            scroll_owner_requirement,
            portal_anchor_requirement,
            unit_posture,
            coordinate_space,
            rounding_posture,
            members: members.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn axis_scope(&self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub fn requirement(&self) -> UiConstraintBoundedMinMaxRequirement {
        self.requirement
    }

    pub fn solve_order(&self) -> UiBoundReconciliationSolveOrder {
        self.solve_order
    }

    pub fn posture(&self) -> UiBoundReconciliationPosture {
        self.posture
    }

    pub fn incoming_available_space_posture(&self) -> Option<UiConstraintAvailableSpacePosture> {
        self.incoming_available_space_posture
    }

    pub fn viewport_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.viewport_requirement
    }

    pub fn scroll_owner_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.scroll_owner_requirement
    }

    pub fn portal_anchor_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.portal_anchor_requirement
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

    pub fn members(&self) -> &[UiConstraintBoundReconciliationMember] {
        &self.members
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn member_digest(member: UiConstraintBoundReconciliationMember) -> u64 {
    stable_text_digest("worth-ui.constraint-bound-reconciliation-member")
        ^ member.member_identity_digest.rotate_left(7)
        ^ bounded_requirement_digest(member.bounded_requirement).rotate_left(13)
        ^ constraint_modifier_digest(member.constraint_modifier).rotate_left(19)
}

fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

fn bounded_requirement_digest(requirement: UiConstraintBoundedMinMaxRequirement) -> u64 {
    match requirement {
        UiConstraintBoundedMinMaxRequirement::None => stable_text_digest("worth-ui.bound.none"),
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis => {
            stable_text_digest("worth-ui.bound.primary-axis")
        }
        UiConstraintBoundedMinMaxRequirement::BothAxes => {
            stable_text_digest("worth-ui.bound.both-axes")
        }
    }
}

fn posture_digest(posture: UiBoundReconciliationPosture) -> u64 {
    stable_text_digest(match posture {
        UiBoundReconciliationPosture::SatisfiedWithoutClamp => {
            "worth-ui.bound.posture.satisfied-without-clamp"
        }
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp => {
            "worth-ui.bound.posture.satisfied-with-declared-clamp"
        }
        UiBoundReconciliationPosture::Underconstrained => {
            "worth-ui.bound.posture.underconstrained"
        }
        UiBoundReconciliationPosture::Overconstrained => {
            "worth-ui.bound.posture.overconstrained"
        }
        UiBoundReconciliationPosture::ContradictoryMinMax => {
            "worth-ui.bound.posture.contradictory-min-max"
        }
        UiBoundReconciliationPosture::UnsupportedUnitMix => {
            "worth-ui.bound.posture.unsupported-unit-mix"
        }
        UiBoundReconciliationPosture::UnsupportedRoundingMix => {
            "worth-ui.bound.posture.unsupported-rounding-mix"
        }
        UiBoundReconciliationPosture::Cyclic => "worth-ui.bound.posture.cyclic",
        UiBoundReconciliationPosture::StaleInput => "worth-ui.bound.posture.stale-input",
        UiBoundReconciliationPosture::UnsupportedSpecialInput => {
            "worth-ui.bound.posture.unsupported-special-input"
        }
    })
}

fn solve_order_digest(order: UiBoundReconciliationSolveOrder) -> u64 {
    match order {
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout => {
            stable_text_digest("worth-ui.bound.solve-order.after-equal-share-before-closeout")
        }
    }
}

fn available_space_posture_digest(posture: Option<UiConstraintAvailableSpacePosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown) => {
            "worth-ui.bound.available-space.unknown"
        }
        Some(UiConstraintAvailableSpacePosture::AdmittedZeroExtent) => {
            "worth-ui.bound.available-space.zero"
        }
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent) => {
            "worth-ui.bound.available-space.positive"
        }
        None => "worth-ui.bound.available-space.none",
    })
}

fn special_input_posture_digest(posture: UiConstraintSpecialInputPosture) -> u64 {
    match posture {
        UiConstraintSpecialInputPosture::NotRequired => {
            stable_text_digest("worth-ui.bound.special-input.not-required")
        }
        UiConstraintSpecialInputPosture::Required => {
            stable_text_digest("worth-ui.bound.special-input.required")
        }
    }
}

fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => "worth-ui.bound.unit.logical-px",
        Some(UiMeasurementUnitPosture::PhysicalPx) => "worth-ui.bound.unit.physical-px",
        Some(UiMeasurementUnitPosture::UnitlessScale) => "worth-ui.bound.unit.unitless-scale",
        None => "worth-ui.bound.unit.none",
    })
}

fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
    stable_text_digest(match space {
        Some(UiMeasurementCoordinateSpace::Viewport) => "worth-ui.bound.coordinate.viewport",
        Some(UiMeasurementCoordinateSpace::Window) => "worth-ui.bound.coordinate.window",
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal) => {
            "worth-ui.bound.coordinate.graph-node-local"
        }
        Some(UiMeasurementCoordinateSpace::HostSurface) => {
            "worth-ui.bound.coordinate.host-surface"
        }
        Some(UiMeasurementCoordinateSpace::PortalLayer) => "worth-ui.bound.coordinate.portal-layer",
        None => "worth-ui.bound.coordinate.none",
    })
}

fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementRoundingPosture::ExactFloat) => "worth-ui.bound.rounding.exact-float",
        Some(UiMeasurementRoundingPosture::HostRounded) => "worth-ui.bound.rounding.host-rounded",
        Some(UiMeasurementRoundingPosture::RuntimeRounded) => {
            "worth-ui.bound.rounding.runtime-rounded"
        }
        Some(UiMeasurementRoundingPosture::DeferredToAllocation) => {
            "worth-ui.bound.rounding.deferred"
        }
        None => "worth-ui.bound.rounding.none",
    })
}

fn constraint_modifier_digest(modifier: Option<UiDeclaredMeasurementConstraintModifier>) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("worth-ui.bound.constraint.bounded")
        }
        None => stable_text_digest("worth-ui.bound.constraint.none"),
    }
}
