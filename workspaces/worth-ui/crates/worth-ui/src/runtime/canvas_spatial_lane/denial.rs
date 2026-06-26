use crate::runtime::WorthUiCanvasSpatialCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialPlanDenialReason {
    LaneAdmissionMissingCanvasSpatialSupport,
    LaneAdmissionPlanMismatch,
    HandleAllocationPlanMismatch,
    MissingCanvasSpatialHook,
    UnsupportedCanvasSpatialHook,
    NoCanvasSpatialRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialFrameDenialReason {
    TargetNotInCanvasSpatialPlan,
    TargetGenerationMismatch,
    DomainGeometryTruthOwnership,
    RendererInternalOwnership,
    DomainGeometryTruthRead,
    NonCanvasSpatialClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialPlanDenial {
    reason: WorthUiCanvasSpatialPlanDenialReason,
    counters: WorthUiCanvasSpatialCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialFrameDenial {
    reason: WorthUiCanvasSpatialFrameDenialReason,
    plan_index: Option<u32>,
    counters: WorthUiCanvasSpatialCounters,
}

impl WorthUiCanvasSpatialPlanDenial {
    pub(crate) fn new(
        reason: WorthUiCanvasSpatialPlanDenialReason,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Self {
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiCanvasSpatialPlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        self.counters
    }
}

impl WorthUiCanvasSpatialFrameDenial {
    pub(crate) fn new(
        reason: WorthUiCanvasSpatialFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiCanvasSpatialFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        self.counters
    }
}
