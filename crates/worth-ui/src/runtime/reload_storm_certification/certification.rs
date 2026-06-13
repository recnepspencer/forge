use crate::runtime::{
    WorthUiReloadCertificationBundle, WorthUiReloadLatencyCounters, WorthUiReloadStormOrderedTruth,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormCertification {
    scenario_name: String,
    scenario_digest: u64,
    ordered_truth: WorthUiReloadStormOrderedTruth,
    bundle: WorthUiReloadCertificationBundle,
}

impl WorthUiReloadStormCertification {
    pub(crate) fn new(
        scenario_name: impl Into<String>,
        scenario_digest: u64,
        ordered_truth: WorthUiReloadStormOrderedTruth,
        bundle: WorthUiReloadCertificationBundle,
    ) -> Self {
        Self {
            scenario_name: scenario_name.into(),
            scenario_digest,
            ordered_truth,
            bundle,
        }
    }

    pub fn scenario_name(&self) -> &str {
        &self.scenario_name
    }

    pub fn scenario_digest(&self) -> u64 {
        self.scenario_digest
    }

    pub fn ordered_truth(&self) -> &WorthUiReloadStormOrderedTruth {
        &self.ordered_truth
    }

    pub fn bundle(&self) -> &WorthUiReloadCertificationBundle {
        &self.bundle
    }

    pub fn counters(&self) -> WorthUiReloadLatencyCounters {
        self.bundle.counters()
    }
}
