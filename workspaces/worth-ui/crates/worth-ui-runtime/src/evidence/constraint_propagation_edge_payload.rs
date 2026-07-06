use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintChildIntrinsicContribution, UiConstraintParentAvailableSpace,
    UiConstraintEqualSharePosture, UiConstraintEqualShareSolveOrder,
    UiPortalAnchorPlanningInputPosture, UiPortalAnchorPlanningInputSolveOrder,
    UiConstraintPropagationEdgeFamily, UiConstraintSiblingNegotiationFixedPointPolicy,
    UiConstraintSiblingNegotiationSolveOrder, UiScrollOwnerPlanningInputPosture,
    UiScrollOwnerPlanningInputSolveOrder, UiViewportPlanningInputPosture,
    UiViewportPlanningInputSolveOrder,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintPropagationEdgePayload {
    ParentAvailableSpace(UiConstraintParentAvailableSpace),
    ChildIntrinsicContribution(UiConstraintChildIntrinsicContribution),
    SiblingNegotiation {
        axis_scope: UiConstraintAxisScope,
        group_identity_digest: u64,
        negotiation_identity_digest: u64,
        fixed_point_policy: UiConstraintSiblingNegotiationFixedPointPolicy,
        solve_order: UiConstraintSiblingNegotiationSolveOrder,
    },
    EqualShareDistribution {
        axis_scope: UiConstraintAxisScope,
        policy: UiConstraintEqualShareDistributionPolicy,
        group_identity_digest: u64,
        distribution_identity_digest: u64,
        solve_order: UiConstraintEqualShareSolveOrder,
        posture: UiConstraintEqualSharePosture,
    },
    BoundedReconciliation {
        axis_scope: UiConstraintAxisScope,
        reconciliation_identity_digest: u64,
        solve_order: UiBoundReconciliationSolveOrder,
        posture: UiBoundReconciliationPosture,
    },
    ViewportInput {
        viewport_identity_digest: u64,
        solve_order: UiViewportPlanningInputSolveOrder,
        posture: UiViewportPlanningInputPosture,
        planning_time_only: bool,
    },
    ScrollViewportInput {
        scroll_identity_digest: u64,
        solve_order: UiScrollOwnerPlanningInputSolveOrder,
        posture: UiScrollOwnerPlanningInputPosture,
        planning_time_only: bool,
    },
    PortalAnchorInput {
        portal_identity_digest: u64,
        solve_order: UiPortalAnchorPlanningInputSolveOrder,
        posture: UiPortalAnchorPlanningInputPosture,
        planning_time_only: bool,
    },
    DurableResizeInput {
        durable_identity_digest: u64,
        axis_scope: UiConstraintAxisScope,
        posture: UiConstraintResizeInputPosture,
        planning_time_only: bool,
    },
}

impl UiConstraintPropagationEdgePayload {
    pub(crate) const fn family(self) -> UiConstraintPropagationEdgeFamily {
        match self {
            Self::ParentAvailableSpace(..) => {
                UiConstraintPropagationEdgeFamily::ParentAvailableSpace
            }
            Self::ChildIntrinsicContribution(..) => {
                UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution
            }
            Self::SiblingNegotiation { .. } => {
                UiConstraintPropagationEdgeFamily::SiblingNegotiation
            }
            Self::EqualShareDistribution { .. } => {
                UiConstraintPropagationEdgeFamily::EqualShareDistribution
            }
            Self::BoundedReconciliation { .. } => {
                UiConstraintPropagationEdgeFamily::BoundedReconciliation
            }
            Self::ViewportInput { .. } => UiConstraintPropagationEdgeFamily::ViewportInput,
            Self::ScrollViewportInput { .. } => UiConstraintPropagationEdgeFamily::ScrollViewportInput,
            Self::PortalAnchorInput { .. } => UiConstraintPropagationEdgeFamily::PortalAnchorInput,
            Self::DurableResizeInput { .. } => {
                UiConstraintPropagationEdgeFamily::DurableResizeInput
            }
        }
    }

    pub(crate) fn identity_digest(self) -> u64 {
        match self {
            Self::ParentAvailableSpace(parent_available_space) => {
                stable_text_digest("worth-ui.constraint-edge.payload.parent-available-space")
                    ^ parent_available_space.identity_digest().rotate_left(7)
            }
            Self::ChildIntrinsicContribution(contribution) => {
                stable_text_digest("worth-ui.constraint-edge.payload.child-intrinsic")
                    ^ contribution.identity_digest().rotate_left(7)
            }
            Self::SiblingNegotiation {
                axis_scope,
                group_identity_digest,
                negotiation_identity_digest,
                fixed_point_policy,
                solve_order,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.sibling-negotiation")
                    ^ axis_scope_digest(axis_scope).rotate_left(7)
                    ^ group_identity_digest.rotate_left(13)
                    ^ negotiation_identity_digest.rotate_left(19)
                    ^ sibling_fixed_point_policy_digest(fixed_point_policy).rotate_left(23)
                    ^ sibling_solve_order_digest(solve_order).rotate_left(29)
            }
            Self::EqualShareDistribution {
                axis_scope,
                policy,
                group_identity_digest,
                distribution_identity_digest,
                solve_order,
                posture,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.equal-share")
                    ^ axis_scope_digest(axis_scope).rotate_left(7)
                    ^ equal_share_policy_digest(policy).rotate_left(13)
                    ^ group_identity_digest.rotate_left(19)
                    ^ distribution_identity_digest.rotate_left(23)
                    ^ equal_share_solve_order_digest(solve_order).rotate_left(29)
                    ^ equal_share_posture_digest(posture).rotate_left(31)
            }
            Self::BoundedReconciliation {
                axis_scope,
                reconciliation_identity_digest,
                solve_order,
                posture,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.bounded-reconciliation")
                    ^ axis_scope_digest(axis_scope).rotate_left(7)
                    ^ reconciliation_identity_digest.rotate_left(13)
                    ^ bound_reconciliation_solve_order_digest(solve_order).rotate_left(19)
                    ^ bound_reconciliation_posture_digest(posture).rotate_left(23)
            }
            Self::ViewportInput {
                viewport_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.viewport")
                    ^ viewport_identity_digest.rotate_left(7)
                    ^ viewport_solve_order_digest(solve_order).rotate_left(13)
                    ^ viewport_posture_digest(posture).rotate_left(19)
                    ^ bool_digest(planning_time_only).rotate_left(23)
            }
            Self::ScrollViewportInput {
                scroll_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.scroll-viewport")
                    ^ scroll_identity_digest.rotate_left(7)
                    ^ scroll_solve_order_digest(solve_order).rotate_left(13)
                    ^ scroll_posture_digest(posture).rotate_left(19)
                    ^ bool_digest(planning_time_only).rotate_left(23)
            }
            Self::PortalAnchorInput {
                portal_identity_digest,
                solve_order,
                posture,
                planning_time_only,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.portal-anchor")
                    ^ portal_identity_digest.rotate_left(7)
                    ^ portal_solve_order_digest(solve_order).rotate_left(13)
                    ^ portal_posture_digest(posture).rotate_left(19)
                    ^ bool_digest(planning_time_only).rotate_left(23)
            }
            Self::DurableResizeInput {
                durable_identity_digest,
                axis_scope,
                posture,
                planning_time_only,
            } => {
                stable_text_digest("worth-ui.constraint-edge.payload.durable-resize")
                    ^ durable_identity_digest.rotate_left(5)
                    ^ axis_scope_digest(axis_scope).rotate_left(7)
                    ^ resize_input_posture_digest(posture).rotate_left(13)
                    ^ bool_digest(planning_time_only).rotate_left(17)
            }
        }
    }

    pub const fn parent_available_space(self) -> Option<UiConstraintParentAvailableSpace> {
        match self {
            Self::ParentAvailableSpace(parent_available_space) => Some(parent_available_space),
            _ => None,
        }
    }

    pub const fn child_intrinsic_contribution(
        self,
    ) -> Option<UiConstraintChildIntrinsicContribution> {
        match self {
            Self::ChildIntrinsicContribution(contribution) => Some(contribution),
            _ => None,
        }
    }
}

fn viewport_solve_order_digest(order: UiViewportPlanningInputSolveOrder) -> u64 {
    match order {
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-viewport.before-derived-families")
        }
    }
}

fn viewport_posture_digest(posture: UiViewportPlanningInputPosture) -> u64 {
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

fn scroll_solve_order_digest(order: UiScrollOwnerPlanningInputSolveOrder) -> u64 {
    match order {
        UiScrollOwnerPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-scroll-owner.before-derived-families")
        }
    }
}

fn scroll_posture_digest(posture: UiScrollOwnerPlanningInputPosture) -> u64 {
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

fn portal_solve_order_digest(order: UiPortalAnchorPlanningInputSolveOrder) -> u64 {
    match order {
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-portal-anchor.before-derived-families")
        }
    }
}

fn portal_posture_digest(posture: UiPortalAnchorPlanningInputPosture) -> u64 {
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

fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.constraint-edge.payload.true"
    } else {
        "worth-ui.constraint-edge.payload.false"
    })
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

fn resize_input_posture_digest(posture: UiConstraintResizeInputPosture) -> u64 {
    match posture {
        UiConstraintResizeInputPosture::DurableAuthorityRequired => {
            stable_text_digest("worth-ui.constraint-resize-posture.durable-authority-required")
        }
    }
}

fn equal_share_posture_digest(posture: UiConstraintEqualSharePosture) -> u64 {
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

fn equal_share_solve_order_digest(order: UiConstraintEqualShareSolveOrder) -> u64 {
    match order {
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds => {
            stable_text_digest("worth-ui.constraint-edge.equal-share.after-sibling-before-bounds")
        }
    }
}

fn sibling_fixed_point_policy_digest(
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

fn sibling_solve_order_digest(order: UiConstraintSiblingNegotiationSolveOrder) -> u64 {
    match order {
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds => {
            stable_text_digest("worth-ui.constraint-sibling.negotiation.before-equal-share-bounds")
        }
    }
}

fn bound_reconciliation_solve_order_digest(order: UiBoundReconciliationSolveOrder) -> u64 {
    match order {
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout => {
            stable_text_digest("worth-ui.constraint-bound.solve-order.after-equal-share-before-closeout")
        }
    }
}

fn bound_reconciliation_posture_digest(posture: UiBoundReconciliationPosture) -> u64 {
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
