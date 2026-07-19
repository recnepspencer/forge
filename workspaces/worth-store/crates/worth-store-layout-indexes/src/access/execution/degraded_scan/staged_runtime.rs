use super::{DegradedScanReady, LoweredDegradedExactScan, StaleDegradedExactScan};
use crate::planning::SelectedDegradedExactScan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DegradedScanRuntime;

pub(crate) const fn degraded_scan_runtime() -> DegradedScanRuntime {
    DegradedScanRuntime
}

impl DegradedScanRuntime {
    pub fn lower(&self, selected: SelectedDegradedExactScan) -> LoweredDegradedExactScan {
        super::lower(selected)
    }

    pub fn admit_ready(
        &self,
        lowered: LoweredDegradedExactScan,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> super::DegradedScanReadinessOutcome {
        super::classify_readiness(lowered, frontier)
    }

    pub fn execute_physical(
        &self,
        ready: DegradedScanReady,
        physical: &mut worth_store_physical_format::InMemoryPhysicalFormatModel,
    ) -> Result<super::DegradedScanExecution, crate::PhysicalDegradedExecutionDenial> {
        super::execute_ready(ready, physical)
    }

    pub fn admit_rebind(
        &self,
        stale: &StaleDegradedExactScan,
        replacement: &SelectedDegradedExactScan,
    ) -> Result<super::DegradedScanRebindAdmission, crate::DegradedScanAdmissionDenied> {
        super::admit_rebind(stale, replacement)
    }

    pub fn rebind(
        &self,
        stale: StaleDegradedExactScan,
        replacement: SelectedDegradedExactScan,
        admission: super::DegradedScanRebindAdmission,
    ) -> Result<DegradedScanReady, crate::DegradedScanAdmissionDenied> {
        super::rebind(stale, replacement, admission)
    }
}
