use crate::evidence::UiConstraintPropagationEdgeFamily;

use super::planning_semantics::{
    UiLayoutOperatorDenialPolicy, UiLayoutOperatorIntrinsicReturnPolicy,
    UiLayoutOperatorOverflowPolicy, UiLayoutOperatorSiblingGroupingRule,
    UiLayoutOperatorSizingMode, UiLayoutOperatorSpecialInputRequirement,
};

pub(super) fn family_name(family: UiConstraintPropagationEdgeFamily) -> &'static str {
    match family {
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace => "parent-available-space",
        UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution => "child-intrinsic",
        UiConstraintPropagationEdgeFamily::SiblingNegotiation => "sibling-negotiation",
        UiConstraintPropagationEdgeFamily::EqualShareDistribution => "equal-share",
        UiConstraintPropagationEdgeFamily::BoundedReconciliation => "bounded-reconciliation",
        UiConstraintPropagationEdgeFamily::ViewportInput => "viewport-input",
        UiConstraintPropagationEdgeFamily::ScrollViewportInput => "scroll-viewport-input",
        UiConstraintPropagationEdgeFamily::PortalAnchorInput => "portal-anchor-input",
        UiConstraintPropagationEdgeFamily::DurableResizeInput => "durable-resize-input",
    }
}

pub(super) fn sizing_mode_name(mode: UiLayoutOperatorSizingMode) -> &'static str {
    match mode {
        UiLayoutOperatorSizingMode::AvailableSpace => "available-space",
        UiLayoutOperatorSizingMode::IntrinsicContent => "intrinsic-content",
        UiLayoutOperatorSizingMode::BoundedContent => "bounded-content",
        UiLayoutOperatorSizingMode::ViewportBound => "viewport-bound",
        UiLayoutOperatorSizingMode::PortalAnchorBound => "portal-anchor-bound",
    }
}

pub(super) fn intrinsic_return_policy_name(
    policy: UiLayoutOperatorIntrinsicReturnPolicy,
) -> &'static str {
    match policy {
        UiLayoutOperatorIntrinsicReturnPolicy::None => "none",
        UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent => "children-to-parent",
    }
}

pub(super) fn sibling_grouping_rule_name(
    rule: UiLayoutOperatorSiblingGroupingRule,
) -> &'static str {
    match rule {
        UiLayoutOperatorSiblingGroupingRule::None => "none",
        UiLayoutOperatorSiblingGroupingRule::StablePeerGroup => "stable-peer-group",
    }
}

pub(super) fn overflow_policy_name(policy: UiLayoutOperatorOverflowPolicy) -> &'static str {
    match policy {
        UiLayoutOperatorOverflowPolicy::None => "none",
        UiLayoutOperatorOverflowPolicy::Clip => "clip",
        UiLayoutOperatorOverflowPolicy::ScrollViewport => "scroll-viewport",
        UiLayoutOperatorOverflowPolicy::PortalAnchorBounded => "portal-anchor-bounded",
    }
}

pub(super) fn special_input_requirement_name(
    requirement: UiLayoutOperatorSpecialInputRequirement,
) -> &'static str {
    match requirement {
        UiLayoutOperatorSpecialInputRequirement::ViewportExtent => "viewport-extent",
        UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent => "scroll-viewport-extent",
        UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect => "portal-anchor-rect",
    }
}

pub(super) fn denial_policy_name(policy: UiLayoutOperatorDenialPolicy) -> &'static str {
    match policy {
        UiLayoutOperatorDenialPolicy::RejectUnsupportedPropagationAuthority => {
            "reject-unsupported-propagation-authority"
        }
    }
}
