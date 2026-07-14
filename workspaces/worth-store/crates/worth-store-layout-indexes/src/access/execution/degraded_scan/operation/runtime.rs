use worth_store_physical_format::PhysicalStoreRuntime;

use super::{denial::DegradedExactScanExecutionDenied, request::DegradedExactScanExecutionRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutDegradedScanRuntime;

pub const fn layout_degraded_scan_runtime() -> LayoutDegradedScanRuntime {
    LayoutDegradedScanRuntime
}

impl LayoutDegradedScanRuntime {
    pub fn prepare(
        self,
        request: DegradedExactScanExecutionRequest<'_>,
    ) -> Result<crate::DegradedScanReadinessOutcome, DegradedExactScanExecutionDenied> {
        super::execution::prepare(request)
    }

    pub fn rebind(
        self,
        stale: crate::StaleDegradedExactScan,
        replacement_request: DegradedExactScanExecutionRequest<'_>,
    ) -> Result<crate::DegradedScanReady, DegradedExactScanExecutionDenied> {
        super::execution::rebind(stale, replacement_request)
    }

    pub fn execute_ready(
        self,
        ready: crate::DegradedScanReady,
        physical: &mut PhysicalStoreRuntime,
    ) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
        super::execution::execute_ready(ready, physical)
    }

    pub fn execute(
        self,
        request: DegradedExactScanExecutionRequest<'_>,
        physical: &mut PhysicalStoreRuntime,
    ) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
        super::execution::execute(request, physical)
    }
}
