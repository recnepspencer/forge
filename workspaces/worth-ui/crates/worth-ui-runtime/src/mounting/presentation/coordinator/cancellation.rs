use worth_ui_host_contract::{UiHostSurfaceCancellationOutcome, UiHostSurfaceStopReason};

use super::super::state::UiPendingMountedSurface;
use crate::facade::UiHostEffectPort;

pub(super) struct UiStoppedMountedSurface {
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    outcome: UiHostSurfaceCancellationOutcome,
    semantic_receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
}

pub(super) fn cancel_all(
    pending: Vec<UiPendingMountedSurface>,
    host: UiHostEffectPort<'_>,
) -> Vec<UiStoppedMountedSurface> {
    stop_all(pending, host, UiHostSurfaceStopReason::Cancelled)
}

pub(super) fn stop_all(
    pending: Vec<UiPendingMountedSurface>,
    host: UiHostEffectPort<'_>,
    reason: UiHostSurfaceStopReason,
) -> Vec<UiStoppedMountedSurface> {
    let mut stopped = Vec::with_capacity(pending.len());
    for pending_surface in pending {
        let outcome =
            host.adapter()
                .cancel_mounted_surface(host.authority(), pending_surface.token, reason);
        stopped.push(UiStoppedMountedSurface {
            binding: pending_surface.binding,
            outcome,
            semantic_receipts: pending_surface.semantic_receipts,
        });
    }
    stopped
}

impl UiStoppedMountedSurface {
    pub(super) fn into_parts(
        self,
    ) -> (
        worth_ui_host_contract::UiSurfaceBindingGeneration,
        UiHostSurfaceCancellationOutcome,
        Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
    ) {
        (self.binding, self.outcome, self.semantic_receipts)
    }
}
