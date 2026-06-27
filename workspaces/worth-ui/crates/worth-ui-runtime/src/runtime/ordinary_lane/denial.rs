use crate::runtime::WorthUiOrdinaryLaneCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryLanePlanDenialReason {
    LaneAdmissionMissingOrdinarySupport,
    LaneAdmissionMissingCommandSurfaceSupport,
    LaneAdmissionMissingStyleTokenSupport,
    LaneAdmissionMissingEguiBoundarySupport,
    NoOrdinaryRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryLaneFrameDenialReason {
    TargetNotInOrdinaryPlan,
    TargetGenerationMismatch,
    NonOrdinaryLaneClaim,
    FramePathSourceParse,
    FramePathRegistryLookup,
    FramePathArtifactScan,
    FullPlanScanCertificationFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLanePlanDenial {
    reason: WorthUiOrdinaryLanePlanDenialReason,
    counters: WorthUiOrdinaryLaneCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneFrameDenial {
    reason: WorthUiOrdinaryLaneFrameDenialReason,
    plan_index: Option<u32>,
    counters: WorthUiOrdinaryLaneCounters,
}

impl WorthUiOrdinaryLanePlanDenial {
    pub(crate) fn new(
        reason: WorthUiOrdinaryLanePlanDenialReason,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiOrdinaryLanePlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }
}

impl WorthUiOrdinaryLaneFrameDenial {
    pub(crate) fn new(
        reason: WorthUiOrdinaryLaneFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiOrdinaryLaneFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }
}
