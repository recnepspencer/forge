use worth_relational::facade::snapshots::SnapshotHandle;

use super::super::WorthQueryPrimaryGraphIntegrationHandle;

pub(in crate::domain_computation) struct WorthQueryApplicationSnapshotLease {
    handle: WorthQueryPrimaryGraphIntegrationHandle,
    snapshot: Option<SnapshotHandle>,
    pub(super) layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
}

impl WorthQueryApplicationSnapshotLease {
    pub(in crate::domain_computation) fn acquire(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Option<Self> {
        let snapshot = handle.with_runtime_mut(|runtime| {
            runtime.snapshots().historical_snapshot_for_branch(branch)
        })?;
        Some(Self {
            handle,
            snapshot: Some(snapshot),
            layout,
        })
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

    pub(in crate::domain_computation) fn snapshot(&self) -> &SnapshotHandle {
        self.snapshot
            .as_ref()
            .expect("application snapshot lease remains live until consumed")
    }

    pub(in crate::domain_computation) fn handle(&self) -> &WorthQueryPrimaryGraphIntegrationHandle {
        &self.handle
    }

    pub(in crate::domain_computation) fn release(mut self) -> bool {
        self.snapshot.take().is_some_and(|snapshot| {
            self.handle
                .with_runtime_mut(|runtime| runtime.snapshots().release_snapshot(&snapshot))
        })
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
