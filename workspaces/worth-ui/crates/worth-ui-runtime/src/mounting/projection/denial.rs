#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedProjectionDenial {
    Identity(super::super::UiMountedIdentityDenial),
    UnknownGraphNode,
    MissingSurfaceBinding,
    ForeignPlan,
    ForeignGraphWorld,
    ForeignMountIncarnation,
    ForeignAllocation,
    PreviewInstanceMismatch,
    CoordinateBasisMismatch,
    NonFiniteGeometry,
    NegativeExtent,
    TableCapacityExceeded,
    AmbiguousStaticPaintToken,
    MissingStaticPaintToken,
    ForeignStaticPaintToken,
    MissingStaticPaintColor,
    InvalidStaticPaintColor,
    MissingStaticPaintAllocation(crate::graph::UiGraphNodeIdentity),
    UnsupportedStaticPaintAllocation(crate::graph::UiGraphNodeIdentity),
    StaticPaintParticipationWithheld(crate::graph::UiGraphNodeIdentity),
    StaticPaintNodeReceiptMismatch,
    StaticPaintCapacityExceeded,
    StaticPaintCompletion(worth_ui_host_contract::UiMountedFilledRectCompletionDenial),
    MissingHitTestOrder(crate::graph::UiGraphNodeIdentity),
    DuplicateHitTestOrder {
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        order: worth_ui_host_contract::UiMountedHitTestOrder,
    },
    MissingHitTestAllocation(crate::graph::UiGraphNodeIdentity),
    UnsupportedHitTestAllocation(crate::graph::UiGraphNodeIdentity),
    HitTestParticipationWithheld(crate::graph::UiGraphNodeIdentity),
    HitTestNodeReceiptMismatch,
    HitTestCapacityExceeded,
    HitTestCompletion(worth_ui_host_contract::UiMountedHitTestCompletionDenial),
    VisualOverlayTargetMissing,
    VisualOverlaySurfaceMismatch,
    DuplicateLaneContribution,
    CostCounterOverflow,
}

impl From<super::super::UiMountedIdentityDenial> for UiMountedProjectionDenial {
    fn from(denial: super::super::UiMountedIdentityDenial) -> Self {
        Self::Identity(denial)
    }
}
