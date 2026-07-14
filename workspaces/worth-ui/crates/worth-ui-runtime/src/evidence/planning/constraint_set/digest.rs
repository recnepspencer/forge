use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope, UiConstraintBoundedMinMaxRequirement,
    UiConstraintEqualShareGroup, UiConstraintResizePermissionPosture,
    UiConstraintSiblingNegotiationMode, UiConstraintSpecialInputPosture,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

pub(super) fn axis_scope_digest(scope: Option<UiConstraintAxisScope>) -> u64 {
    match scope {
        Some(UiConstraintAxisScope::Primary) => {
            stable_text_digest("worth-ui.constraint-axis.primary")
        }
        Some(UiConstraintAxisScope::Cross) => stable_text_digest("worth-ui.constraint-axis.cross"),
        Some(UiConstraintAxisScope::Both) => stable_text_digest("worth-ui.constraint-axis.both"),
        None => stable_text_digest("worth-ui.constraint-axis.none"),
    }
}

pub(super) fn sibling_negotiation_mode_digest(mode: UiConstraintSiblingNegotiationMode) -> u64 {
    match mode {
        UiConstraintSiblingNegotiationMode::None => {
            stable_text_digest("worth-ui.constraint-sibling.none")
        }
        UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis => {
            stable_text_digest("worth-ui.constraint-sibling.primary-axis")
        }
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional => {
            stable_text_digest("worth-ui.constraint-sibling.two-dimensional")
        }
    }
}

pub(super) fn equal_share_group_digest(group: UiConstraintEqualShareGroup) -> u64 {
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

pub(super) fn bounded_requirement_digest(requirement: UiConstraintBoundedMinMaxRequirement) -> u64 {
    match requirement {
        UiConstraintBoundedMinMaxRequirement::None => {
            stable_text_digest("worth-ui.constraint-bounds.none")
        }
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis => {
            stable_text_digest("worth-ui.constraint-bounds.primary-axis")
        }
        UiConstraintBoundedMinMaxRequirement::BothAxes => {
            stable_text_digest("worth-ui.constraint-bounds.both-axes")
        }
    }
}

pub(super) fn special_input_posture_digest(posture: UiConstraintSpecialInputPosture) -> u64 {
    match posture {
        UiConstraintSpecialInputPosture::NotRequired => {
            stable_text_digest("worth-ui.constraint-special-input.not-required")
        }
        UiConstraintSpecialInputPosture::Required => {
            stable_text_digest("worth-ui.constraint-special-input.required")
        }
    }
}

pub(super) fn resize_permission_posture_digest(
    posture: UiConstraintResizePermissionPosture,
) -> u64 {
    match posture {
        UiConstraintResizePermissionPosture::None => {
            stable_text_digest("worth-ui.constraint-resize.none")
        }
        UiConstraintResizePermissionPosture::DurableAuthorityLane => {
            stable_text_digest("worth-ui.constraint-resize.durable-authority-lane")
        }
    }
}

pub(super) fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => "worth-ui.constraint-unit.logical-px",
        Some(UiMeasurementUnitPosture::PhysicalPx) => "worth-ui.constraint-unit.physical-px",
        Some(UiMeasurementUnitPosture::UnitlessScale) => "worth-ui.constraint-unit.unitless-scale",
        None => "worth-ui.constraint-unit.none",
    })
}

pub(super) fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
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

pub(super) fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
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

pub(super) fn available_space_posture_digest(
    posture: Option<UiConstraintAvailableSpacePosture>,
) -> u64 {
    stable_text_digest(match posture {
        Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown) => {
            "worth-ui.constraint-available-space.unknown"
        }
        Some(UiConstraintAvailableSpacePosture::AdmittedZeroExtent) => {
            "worth-ui.constraint-available-space.zero"
        }
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent) => {
            "worth-ui.constraint-available-space.positive"
        }
        None => "worth-ui.constraint-available-space.none",
    })
}
