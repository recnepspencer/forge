use worth_store::physical_runtime::ServingPhysicalRuntime;

use super::configuration::BoundedResidencyConfiguration;

mod boundary_trace;
mod event_reconciliation;
mod scope_isolation;

pub(super) struct AllocationDimensionEvidence {
    pub(super) name: &'static str,
    pub(super) attempts: u64,
    pub(super) admissions: u64,
    pub(super) releases: u64,
    pub(super) denials: u64,
    pub(super) allocator_failures: u64,
    pub(super) admitted_units: u64,
    pub(super) released_units: u64,
    pub(super) denied_units: u64,
    pub(super) active_units: u64,
    pub(super) current_units: u64,
    pub(super) peak_units: u64,
    pub(super) limit_units: u64,
}

pub(super) struct ScopeIsolationEvidence {
    pub(super) admitted_scopes: u32,
    pub(super) exact_scope_denials: u32,
    pub(super) global_envelope_denied: bool,
    pub(super) global_denial_requested: u64,
    pub(super) global_denial_current: u64,
    pub(super) global_denial_limit: u64,
    pub(super) peak_operation_bytes: u64,
    pub(super) terminal_operation_bytes: u64,
    pub(super) all_effect_free: bool,
}

pub(super) struct AllocationBoundaryTraceEvidence {
    pub(super) store: [u8; 16],
    pub(super) pool_incarnation: u64,
    pub(super) event_count: u64,
    pub(super) process: u32,
    pub(super) attributed_actualizations: u64,
    pub(super) events: Vec<AllocationBoundaryEventEvidence>,
}

pub(super) struct AllocationBoundaryEventEvidence {
    pub(super) sequence: u64,
    pub(super) kind: &'static str,
    pub(super) dimension: &'static str,
    pub(super) scope: Option<&'static str>,
    pub(super) requested_units: u64,
    pub(super) actual_units: u64,
    pub(super) process: u32,
    pub(super) physical_operation: Option<u64>,
}

pub(super) struct AllocationPressureEvidence {
    pub(super) scopes: ScopeIsolationEvidence,
    pub(super) dimensions: [AllocationDimensionEvidence; 19],
    pub(super) trace: AllocationBoundaryTraceEvidence,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
) -> Result<AllocationPressureEvidence, String> {
    let scopes = scope_isolation::prove(serving, configuration)?;
    let dimensions = event_reconciliation::reconcile(serving)?;
    let trace = boundary_trace::reconcile(serving)?;
    Ok(AllocationPressureEvidence {
        scopes,
        dimensions,
        trace,
    })
}
