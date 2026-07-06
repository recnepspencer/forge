use super::gate_receipt::WorthUiActivationGateReceiptParts;
use crate::runtime::{
    WorthUiActivationGateCounters, WorthUiActivationGateDenial, WorthUiActivationGateDenialReason,
    WorthUiActivationGateReceipt, WorthUiActiveRuntimeObservation, WorthUiFrameBoundary,
    WorthUiReadyActivation, WorthUiRuntimeFrameEpoch,
};

pub(crate) struct WorthUiFrameActivationGate;

impl WorthUiFrameActivationGate {
    pub(crate) fn activate_at_boundary(
        active: WorthUiActiveRuntimeObservation,
        ready: &WorthUiReadyActivation,
        boundary: WorthUiFrameBoundary,
        runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Result<WorthUiActivationGateReceipt, WorthUiActivationGateDenial> {
        let mut counters = ready.counters();
        reject_unsafe_boundary(ready, boundary, &mut counters)?;
        reject_stale_or_future_boundary(ready, boundary, &mut counters)?;
        reject_boundary_runtime_mismatch(ready, boundary, runtime_frame_epoch, &mut counters)?;
        Ok(WorthUiActivationGateReceipt::new(
            WorthUiActivationGateReceiptParts {
                active_artifact_digest: active.artifact_digest(),
                active_plan_digest: active.active_plan_digest(),
                active_snapshot_digest: active.snapshot_digest(),
                candidate_artifact_digest: ready.candidate_artifact_digest(),
                candidate_execution_plan_digest: ready.candidate_execution_plan_digest(),
                handle_allocation_basis_digest: ready.handle_allocation_basis_digest(),
                node_classification_count: ready.node_classification_count(),
                lane_changed_node_count: ready.lane_changed_node_count(),
                reconciliation_basis_digest: ready.reconciliation_basis_digest(),
                reconciliation_receipt_count: ready.reconciliation_receipt_count(),
                query_rebind_basis_digest: ready.query_rebind_basis_digest(),
                query_rebind_entry_count: ready.query_rebind_entry_count(),
                query_rebind_denied_count: ready.query_rebind_denied_count(),
                lane_parity_semantic_reference_digest: ready
                    .lane_parity_semantic_reference_digest(),
                readiness_frame_epoch: ready.readiness_frame_epoch(),
                boundary_frame_epoch: boundary.frame_epoch(),
                counters,
            },
        ))
    }
}

fn reject_unsafe_boundary(
    ready: &WorthUiReadyActivation,
    boundary: WorthUiFrameBoundary,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_boundary_check();
    if boundary.is_safe_to_activate() {
        Ok(())
    } else {
        Err(denial(
            ready,
            boundary,
            WorthUiActivationGateDenialReason::UnsafeFrameBoundary,
            *counters,
        ))
    }
}

fn reject_boundary_runtime_mismatch(
    ready: &WorthUiReadyActivation,
    boundary: WorthUiFrameBoundary,
    runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_boundary_check();
    if boundary.frame_epoch() == runtime_frame_epoch {
        Ok(())
    } else {
        Err(denial(
            ready,
            boundary,
            WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch,
            *counters,
        ))
    }
}

fn reject_stale_or_future_boundary(
    ready: &WorthUiReadyActivation,
    boundary: WorthUiFrameBoundary,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_boundary_check();
    if boundary.frame_epoch() < ready.readiness_frame_epoch() {
        return Err(denial(
            ready,
            boundary,
            WorthUiActivationGateDenialReason::StaleFrameEpoch,
            *counters,
        ));
    }
    if boundary.frame_epoch() > ready.readiness_frame_epoch() {
        return Err(denial(
            ready,
            boundary,
            WorthUiActivationGateDenialReason::FutureFrameEpochMismatch,
            *counters,
        ));
    }
    Ok(())
}

fn denial(
    ready: &WorthUiReadyActivation,
    boundary: WorthUiFrameBoundary,
    reason: WorthUiActivationGateDenialReason,
    mut counters: WorthUiActivationGateCounters,
) -> WorthUiActivationGateDenial {
    counters.record_denial();
    WorthUiActivationGateDenial::new(
        ready.active_artifact_digest(),
        ready.candidate_artifact_digest(),
        ready.readiness_frame_epoch(),
        boundary.frame_epoch(),
        reason,
        counters,
    )
}
