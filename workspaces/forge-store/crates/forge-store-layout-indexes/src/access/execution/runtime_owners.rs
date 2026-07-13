use super::lowering_facade::AccessLoweringFacade;
use super::{DegradedScanReadmission, DegradedScanReady, StaleDegradedExactScan};
use crate::planning::SelectedDegradedExactScan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegradedScanRuntime;
pub const fn degraded_scan_runtime() -> DegradedScanRuntime {
    DegradedScanRuntime
}

impl DegradedScanRuntime {
    pub fn lower(&self, selected: SelectedDegradedExactScan) -> super::LoweredDegradedExactScan {
        AccessLoweringFacade.lower_degraded(selected)
    }
    pub fn admit_ready(
        &self,
        lowered: super::LoweredDegradedExactScan,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> super::DegradedScanReadinessOutcome {
        AccessLoweringFacade.admit_degraded_ready(lowered, frontier)
    }
    pub fn execute_physical(
        &self,
        ready: DegradedScanReady,
        physical: &mut forge_store_physical_format::PlatformPhysicalFacade,
    ) -> Result<super::DegradedScanExecution, super::PhysicalDegradedExecutionDenial> {
        AccessLoweringFacade.execute_physical_degraded_exact_scan(ready, physical)
    }
    pub fn admit_stale_readmission(
        &self,
        stale: &StaleDegradedExactScan,
        current: crate::CurrentLayoutMaterialization,
    ) -> Result<DegradedScanReadmission, super::DegradedScanAdmissionDenied> {
        super::degraded_scan::admit_stale_readmission(stale, current)
    }
    pub fn readmit_stale(
        &self,
        stale: StaleDegradedExactScan,
        admission: DegradedScanReadmission,
    ) -> Result<DegradedScanReady, super::DegradedScanAdmissionDenied> {
        super::degraded_scan::readmit_stale(stale, admission)
    }
}
