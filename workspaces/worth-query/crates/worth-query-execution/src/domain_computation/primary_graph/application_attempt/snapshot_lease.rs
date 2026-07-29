use worth_relational::facade::snapshots::SnapshotHandle;

use super::super::WorthQueryPrimaryGraphIntegrationHandle;

pub(in crate::domain_computation::primary_graph) struct WorthQueryApplicationSnapshotLease {
    handle: WorthQueryPrimaryGraphIntegrationHandle,
    snapshot: Option<SnapshotHandle>,
    pub(super) layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
}

impl WorthQueryApplicationSnapshotLease {
    pub(super) fn acquire(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
    ) -> Self {
        let snapshot = handle.with_runtime_mut(|runtime| runtime.snapshots().snapshot());
        Self {
            handle,
            snapshot: Some(snapshot),
            layout,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn from_existing(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
        snapshot: SnapshotHandle,
    ) -> Self {
        Self {
            handle,
            snapshot: Some(snapshot),
            layout,
        }
    }

    pub(super) fn snapshot(&self) -> &SnapshotHandle {
        self.snapshot
            .as_ref()
            .expect("application snapshot lease remains live until consumed")
    }

    pub(super) fn handle(&self) -> &WorthQueryPrimaryGraphIntegrationHandle {
        &self.handle
    }
}

impl Drop for WorthQueryApplicationSnapshotLease {
    fn drop(&mut self) {
        if let Some(snapshot) = self.snapshot.take() {
            self.handle.with_runtime_mut(|runtime| {
                runtime.snapshots().release_snapshot(&snapshot);
            });
        }
    }
}
