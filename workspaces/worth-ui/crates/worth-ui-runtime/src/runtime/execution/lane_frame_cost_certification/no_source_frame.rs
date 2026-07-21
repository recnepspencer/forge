use crate::runtime::WorthUiSteadyFrameCounters;

use super::denial::WorthUiLaneFrameCostCertificationDenialReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiNoSourceFrameProof {
    forbidden_work_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiBroadScanRegressionDenial {
    broad_scan_count: u64,
}

impl WorthUiNoSourceFrameProof {
    pub(crate) fn certify(
        counters: WorthUiSteadyFrameCounters,
    ) -> Result<Self, WorthUiLaneFrameCostCertificationDenialReason> {
        let forbidden_work_count = counters.total_forbidden_source_or_registry_work()
            + counters.ordinary().component_string_resolution_count() as u64
            + counters.ordinary().command_string_resolution_count() as u64;
        if forbidden_work_count > 0 {
            return Err(
                WorthUiLaneFrameCostCertificationDenialReason::ForbiddenSourceOrRegistryWork,
            );
        }
        Ok(Self {
            forbidden_work_count,
        })
    }

    pub fn forbidden_work_count(self) -> u64 {
        self.forbidden_work_count
    }
}

impl WorthUiBroadScanRegressionDenial {
    pub(crate) fn certify_absent(
        counters: WorthUiSteadyFrameCounters,
    ) -> Result<Self, WorthUiLaneFrameCostCertificationDenialReason> {
        let broad_scan_count = counters.ordinary().artifact_tree_scan_count() as u64
            + counters.ordinary().full_plan_scan_count() as u64
            + counters.virtualized_data().full_collection_scan_count() as u64;
        if broad_scan_count > 0 {
            return Err(WorthUiLaneFrameCostCertificationDenialReason::BroadScanRegression);
        }
        Ok(Self { broad_scan_count })
    }

    pub fn broad_scan_count(self) -> u64 {
        self.broad_scan_count
    }
}
