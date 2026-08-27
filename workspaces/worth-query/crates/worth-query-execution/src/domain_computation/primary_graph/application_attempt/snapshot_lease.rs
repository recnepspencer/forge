use worth_relational::facade::snapshots::SnapshotHandle;

use super::super::WorthQueryPrimaryGraphIntegrationHandle;

pub(in crate::domain_computation) struct WorthQueryApplicationSnapshotLease {
    handle: WorthQueryPrimaryGraphIntegrationHandle,
    snapshot: Option<SnapshotHandle>,
    basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    pub(super) layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
}

impl WorthQueryApplicationSnapshotLease {
    pub(in crate::domain_computation) fn acquire(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Option<Self> {
        let (basis, snapshot) = handle.with_runtime_mut(|runtime| {
            let identity = runtime.branch_identity(branch).ok()?;
            let (_, basis) = runtime.observe_branch(&identity).ok()?;
            let snapshot = runtime
                .snapshots()
                .snapshot_for_observation(&basis.observation())
                .ok()?;
            Some((basis, snapshot))
        })?;
        Some(Self {
            handle,
            snapshot: Some(snapshot),
            basis,
            layout,
        })
    }

    pub(in crate::domain_computation::primary_graph) fn from_existing(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
        basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
        snapshot: SnapshotHandle,
    ) -> Self {
        assert_eq!(
            basis.observation().version_id(),
            snapshot.version_id(),
            "existing application snapshot must select its carried owner basis"
        );
        assert_eq!(
            basis.identity().branch_id(),
            snapshot.branch_id(),
            "existing application snapshot and carried basis must share a branch"
        );
        Self {
            handle,
            snapshot: Some(snapshot),
            basis,
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

    pub(in crate::domain_computation) fn basis_descriptor(
        &self,
    ) -> &worth_relational::facade::branch::RelationalBranchBasisDescriptor {
        self.basis.descriptor()
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
