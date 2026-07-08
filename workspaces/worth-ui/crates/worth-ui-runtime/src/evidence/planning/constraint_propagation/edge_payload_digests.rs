use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintEqualSharePosture, UiConstraintEqualShareSolveOrder,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationSolveOrder,
    UiPortalAnchorPlanningInputPosture, UiPortalAnchorPlanningInputSolveOrder,
    UiScrollOwnerPlanningInputPosture, UiScrollOwnerPlanningInputSolveOrder,
    UiViewportPlanningInputPosture, UiViewportPlanningInputSolveOrder,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConstraintAxisScope {
    Primary,
    Cross,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConstraintEqualShareDistributionPolicy {
    ExactFractional,
    DeterministicRemainderLeftToRightByStablePeerIdentity,
    DeterministicRemainderCenterOutByStablePeerIdentity,
    DenyIfNonIntegralRequired,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConstraintResizeInputPosture {
    DurableAuthorityRequired,
}

pub(super) fn viewport_solve_order_digest(order: UiViewportPlanningInputSolveOrder) -> u64 {
    match order {
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-viewport.before-derived-families")
        }
    }
}

pub(super) fn viewport_posture_digest(posture: UiViewportPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-viewport.posture.admitted-planning-time-only"
        }
        UiViewportPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-viewport.posture.missing-required-evidence"
        }
        UiViewportPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-viewport.posture.incompatible-measurement-posture"
        }
    })
}

pub(super) fn scroll_solve_order_digest(order: UiScrollOwnerPlanningInputSolveOrder) -> u64 {
    match order {
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-scroll-owner.before-derived-families")
        }
    }
}

pub(super) fn scroll_posture_digest(posture: UiScrollOwnerPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-scroll-owner.posture.admitted-planning-time-only"
        }
        UiScrollOwnerPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-scroll-owner.posture.missing-required-evidence"
        }
        UiScrollOwnerPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-scroll-owner.posture.incompatible-measurement-posture"
        }
    })
}

pub(super) fn portal_solve_order_digest(order: UiPortalAnchorPlanningInputSolveOrder) -> u64 {
    match order {
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-portal-anchor.before-derived-families")
        }
    }
}

pub(super) fn portal_posture_digest(posture: UiPortalAnchorPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-portal-anchor.posture.admitted-planning-time-only"
        }
        UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-portal-anchor.posture.missing-required-evidence"
        }
        UiPortalAnchorPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-portal-anchor.posture.incompatible-measurement-posture"
        }
    })
}

pub(super) fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.constraint-edge.payload.true"
    } else {
        "worth-ui.constraint-edge.payload.false"
    })
}

pub(super) fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

pub(super) fn equal_share_policy_digest(policy: UiConstraintEqualShareDistributionPolicy) -> u64 {
    match policy {
        UiConstraintEqualShareDistributionPolicy::ExactFractional => {
            stable_text_digest("worth-ui.constraint-equal-share.exact-fractional")
        }
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity => {
            stable_text_digest("worth-ui.constraint-equal-share.deterministic-peer-remainder-left-to-right")
        }
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderCenterOutByStablePeerIdentity => {
            stable_text_digest("worth-ui.constraint-equal-share.deterministic-peer-remainder-center-out")
        }
        UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired => {
            stable_text_digest("worth-ui.constraint-equal-share.deny-if-non-integral-required")
        }
    }
}

pub(super) fn resize_input_posture_digest(posture: UiConstraintResizeInputPosture) -> u64 {
    match posture {
        UiConstraintResizeInputPosture::DurableAuthorityRequired => {
            stable_text_digest("worth-ui.constraint-resize-posture.durable-authority-required")
        }
    }
}

pub(super) fn equal_share_posture_digest(posture: UiConstraintEqualSharePosture) -> u64 {
    stable_text_digest(match posture {
        UiConstraintEqualSharePosture::ExactFractional => {
            "worth-ui.constraint-edge.equal-share-posture.exact-fractional"
        }
        UiConstraintEqualSharePosture::DeterministicRemainderApplied => {
            "worth-ui.constraint-edge.equal-share-posture.deterministic-remainder"
        }
        UiConstraintEqualSharePosture::ZeroShare => {
            "worth-ui.constraint-edge.equal-share-posture.zero-share"
        }
        UiConstraintEqualSharePosture::ZeroAvailableSpace => {
            "worth-ui.constraint-edge.equal-share-posture.zero-available-space"
        }
        UiConstraintEqualSharePosture::NoAdmittedAvailableSpace => {
            "worth-ui.constraint-edge.equal-share-posture.no-admitted-available-space"
        }
        UiConstraintEqualSharePosture::SingleSurvivingPeer => {
            "worth-ui.constraint-edge.equal-share-posture.single-surviving-peer"
        }
    })
}

pub(super) fn equal_share_solve_order_digest(order: UiConstraintEqualShareSolveOrder) -> u64 {
    match order {
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds => {
            stable_text_digest("worth-ui.constraint-edge.equal-share.after-sibling-before-bounds")
        }
    }
}

pub(super) fn sibling_fixed_point_policy_digest(
    policy: UiConstraintSiblingNegotiationFixedPointPolicy,
) -> u64 {
    match policy {
        UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired => {
            stable_text_digest("worth-ui.constraint-sibling.negotiation.not-required")
        }
        UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual => {
            stable_text_digest("worth-ui.constraint-sibling.negotiation.admitted-fixed-point")
        }
    }
}

pub(super) fn sibling_solve_order_digest(order: UiConstraintSiblingNegotiationSolveOrder) -> u64 {
    match order {
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds => {
            stable_text_digest("worth-ui.constraint-sibling.negotiation.before-equal-share-bounds")
        }
    }
}

pub(super) fn bound_reconciliation_solve_order_digest(order: UiBoundReconciliationSolveOrder) -> u64 {
    match order {
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout => {
            stable_text_digest("worth-ui.constraint-bound.solve-order.after-equal-share-before-closeout")
        }
    }
}

pub(super) fn bound_reconciliation_posture_digest(posture: UiBoundReconciliationPosture) -> u64 {
    stable_text_digest(match posture {
        UiBoundReconciliationPosture::SatisfiedWithoutClamp => {
            "worth-ui.constraint-bound.posture.satisfied-without-clamp"
        }
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp => {
            "worth-ui.constraint-bound.posture.satisfied-with-declared-clamp"
        }
        UiBoundReconciliationPosture::Underconstrained => {
            "worth-ui.constraint-bound.posture.underconstrained"
        }
        UiBoundReconciliationPosture::Overconstrained => {
            "worth-ui.constraint-bound.posture.overconstrained"
        }
        UiBoundReconciliationPosture::ContradictoryMinMax => {
            "worth-ui.constraint-bound.posture.contradictory-min-max"
        }
        UiBoundReconciliationPosture::UnsupportedUnitMix => {
            "worth-ui.constraint-bound.posture.unsupported-unit-mix"
        }
        UiBoundReconciliationPosture::UnsupportedRoundingMix => {
            "worth-ui.constraint-bound.posture.unsupported-rounding-mix"
        }
        UiBoundReconciliationPosture::Cyclic => "worth-ui.constraint-bound.posture.cyclic",
        UiBoundReconciliationPosture::StaleInput => "worth-ui.constraint-bound.posture.stale-input",
        UiBoundReconciliationPosture::UnsupportedSpecialInput => {
            "worth-ui.constraint-bound.posture.unsupported-special-input"
        }
    })
}