use worth_signal::facade::{ResourceCancellationReason, ResourceRequestHandle};

use crate::physical_runtime::PhysicalWorkIdentity;

use super::PhysicalSignalGraph;

impl PhysicalSignalGraph {
    pub(super) fn cancel_unbound_request(
        &mut self,
        identity: PhysicalWorkIdentity,
        request: ResourceRequestHandle,
    ) {
        let cancelled = self
            .runtime
            .cancel_resource_request(request, ResourceCancellationReason::HostRequested)
            .is_ok_and(|report| report.cancelled_request().is_some());
        self.healthy &= cancelled;
        self.release_identity(identity);
    }
}
