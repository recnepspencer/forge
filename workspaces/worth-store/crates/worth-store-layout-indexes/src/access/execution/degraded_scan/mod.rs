mod authority;
mod counter_receipt;
mod executed;
mod lowering;
mod operation;
mod readiness;
mod rebind;
#[cfg(test)]
mod staged_runtime;

pub use counter_receipt::DegradedScanCounterReceipt;
pub(in crate::access::execution::degraded_scan) use executed::execute_ready;
pub use executed::DegradedScanExecution;
pub(in crate::access::execution::degraded_scan) use lowering::lower;
pub use lowering::{DegradedScanLoweringBasis, LoweredDegradedExactScan, StaleDegradedExactScan};
pub use operation::{
    layout_degraded_scan_runtime, DegradedExactScanExecutionDenied,
    DegradedExactScanExecutionRequest, LayoutDegradedScanRuntime,
};
pub(in crate::access::execution::degraded_scan) use readiness::admit_rebound_ready;
pub(in crate::access::execution::degraded_scan) use readiness::classify_readiness;
pub use readiness::{
    degraded_scan_readiness_cases, DegradedScanReadinessCaseId, DegradedScanReadinessOutcome,
    DegradedScanReadinessView, DegradedScanReady,
};
pub(in crate::access::execution::degraded_scan) use rebind::{admit as admit_rebind, rebind};
pub use rebind::{DegradedScanRebindAdmission, DegradedScanRebindTrace};
#[cfg(test)]
pub(crate) use staged_runtime::degraded_scan_runtime;
