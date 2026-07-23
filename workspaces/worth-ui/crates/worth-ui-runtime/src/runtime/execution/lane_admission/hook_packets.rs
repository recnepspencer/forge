use crate::runtime::{WorthUiLaneAdapterHook, WorthUiLaneAdmissionCounters, WorthUiLaneSupportRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExtensionHookAdmission {
    hook: WorthUiLaneAdapterHook,
    preserved_lane_support: WorthUiLaneSupportRow,
    counters: WorthUiLaneAdmissionCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiUnsupportedHookDenialReason {
    LaneNotAdmitted,
    ActivePlanTruthOverride,
    QueryPostureOverride,
    StateCarryForwardOverride,
    LaneTaxonomyOverride,
    PerformanceCertificationOverride,
    PrivateLaneClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiUnsupportedHookDenial {
    hook: WorthUiLaneAdapterHook,
    reason: WorthUiUnsupportedHookDenialReason,
    counters: Box<WorthUiLaneAdmissionCounters>,
}

impl WorthUiExtensionHookAdmission {
    pub(crate) fn new(
        hook: WorthUiLaneAdapterHook,
        preserved_lane_support: WorthUiLaneSupportRow,
        counters: WorthUiLaneAdmissionCounters,
    ) -> Self {
        Self {
            hook,
            preserved_lane_support,
            counters,
        }
    }

    pub fn hook(&self) -> &WorthUiLaneAdapterHook {
        &self.hook
    }

    pub fn preserved_lane_support(&self) -> &WorthUiLaneSupportRow {
        &self.preserved_lane_support
    }

    pub fn counters(&self) -> WorthUiLaneAdmissionCounters {
        self.counters
    }
}

impl WorthUiUnsupportedHookDenial {
    pub(crate) fn new(
        hook: WorthUiLaneAdapterHook,
        reason: WorthUiUnsupportedHookDenialReason,
        mut counters: WorthUiLaneAdmissionCounters,
    ) -> Self {
        counters.record_denial();
        Self {
            hook,
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn hook(&self) -> &WorthUiLaneAdapterHook {
        &self.hook
    }

    pub fn reason(&self) -> WorthUiUnsupportedHookDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiLaneAdmissionCounters {
        *self.counters
    }
}
