use crate::runtime::WorthUiSteadyFrameCounterDenialReason;

use super::counters::WorthUiLaneFrameCostCertificationCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneFrameCostCertificationDenial {
    reason: WorthUiLaneFrameCostCertificationDenialReason,
    counters: WorthUiLaneFrameCostCertificationCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLaneFrameCostCertificationDenialReason {
    EmptyScenario,
    MissingSteadyFrameReceipt,
    ActivePlanDigestMismatch {
        active_plan_digest: u64,
        receipt_plan_digest: u64,
    },
    MissingLaneEvidence,
    DuplicateLaneEvidence,
    MissingLaneCertificationEvidence,
    UncertifiedFrameReceipt(WorthUiSteadyFrameCounterDenialReason),
    ForbiddenSourceOrRegistryWork,
    BroadScanRegression,
    FullCollectionDataScan,
    RealtimeOrdinaryTraversal,
    MissingScaleVariation,
    MissingCrossLaneParity,
    CrossLaneParityPlanDigestMismatch {
        active_plan_digest: u64,
        parity_active_plan_digest: u64,
    },
    FoundationalReadinessWithoutWorthUiEvidence,
    FoundationalReadinessNotRequested,
    FoundationalReadinessDenied,
    FoundationalCertificationDenied,
}

impl WorthUiLaneFrameCostCertificationDenial {
    pub(crate) fn new(
        reason: WorthUiLaneFrameCostCertificationDenialReason,
        mut counters: WorthUiLaneFrameCostCertificationCounters,
    ) -> Self {
        counters.record_denial();
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiLaneFrameCostCertificationDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiLaneFrameCostCertificationCounters {
        self.counters
    }
}
