//! Injected provider terminal-outcome boundary.

use std::sync::Arc;

#[derive(Clone, Copy)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryPrimaryGraphFault {
    LostCommitResponse,
    RejectedSessionPreparation,
    RejectedCommitBeforeTransaction,
    FailedIndexPublication,
    SkippedInvariantOwnerExecution,
    RelationalInvariantViolation,
    #[cfg(test)]
    UndeclaredApplicationTouch,
}

pub(in crate::domain_computation::primary_graph) trait WorthQueryPrimaryGraphFaultPort:
    Send + Sync
{
    fn take(&self, fault: WorthQueryPrimaryGraphFault) -> bool;
}

struct WorthQueryNoPrimaryGraphFaults;

impl WorthQueryPrimaryGraphFaultPort for WorthQueryNoPrimaryGraphFaults {
    fn take(&self, _fault: WorthQueryPrimaryGraphFault) -> bool {
        false
    }
}

pub(in crate::domain_computation::primary_graph) fn production_fault_port(
) -> Arc<dyn WorthQueryPrimaryGraphFaultPort> {
    Arc::new(WorthQueryNoPrimaryGraphFaults)
}
