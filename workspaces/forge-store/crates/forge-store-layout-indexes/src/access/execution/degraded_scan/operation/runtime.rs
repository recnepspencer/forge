use forge_store_physical_format::PlatformPhysicalFacade;

use super::{denial::DegradedExactScanExecutionDenied, request::DegradedExactScanExecutionRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutDegradedScanRuntime;

pub const fn layout_degraded_scan_runtime() -> LayoutDegradedScanRuntime {
    LayoutDegradedScanRuntime
}

impl LayoutDegradedScanRuntime {
    pub fn execute(
        self,
        request: DegradedExactScanExecutionRequest<'_>,
        physical: &mut PlatformPhysicalFacade,
    ) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
        super::execution::execute(request, physical)
    }
}
