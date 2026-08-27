use worth_relational::facade::snapshots::SnapshotHandle;

use super::super::WorthQueryPrimaryGraphIntegrationHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryApplicationSnapshotLeaseDenial {
    BranchIdentityUnavailable,
    BranchObservationUnavailable,
    ForeignRuntime,
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    SnapshotIdentityExhausted,
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
}

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
    ) -> Result<Self, WorthQueryApplicationSnapshotLeaseDenial> {
        let (basis, snapshot) = handle.with_runtime_mut(|runtime| {
            let identity = runtime.branch_identity(branch).map_err(|_| {
                WorthQueryApplicationSnapshotLeaseDenial::BranchIdentityUnavailable
            })?;
            let (_, basis) = runtime.observe_branch(&identity).map_err(|denial| match denial {
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
                    WorthQueryApplicationSnapshotLeaseDenial::RetentionCapacityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
                    WorthQueryApplicationSnapshotLeaseDenial::RetentionIdentityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
                    WorthQueryApplicationSnapshotLeaseDenial::SnapshotIdentityExhausted
                }
                _ => WorthQueryApplicationSnapshotLeaseDenial::BranchObservationUnavailable,
            })?;
            let snapshot = runtime
                .snapshots()
                .snapshot_for_observation(&basis.observation())
                .map_err(|denial| match denial {
                    worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::ForeignRuntime { .. } => {
                        WorthQueryApplicationSnapshotLeaseDenial::ForeignRuntime
                    }
                    worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::ActiveSnapshotCapacityExhausted {
                        maximum_active_snapshots,
                    } => WorthQueryApplicationSnapshotLeaseDenial::ActiveSnapshotCapacityExhausted {
                        maximum_active_snapshots,
                    },
                    worth_relational::facade::snapshots::RelationalSnapshotAdmissionDenial::SnapshotIdentityExhausted => {
                        WorthQueryApplicationSnapshotLeaseDenial::SnapshotIdentityExhausted
                    }
                })?;
            Ok((basis, snapshot))
        })?;
        Ok(Self {
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
        let Some(snapshot) = self.snapshot.take() else {
            return false;
        };
        self.handle.with_runtime_mut(|runtime| {
            crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
        });
        true
    }
}

impl Drop for WorthQueryApplicationSnapshotLease {
    fn drop(&mut self) {
        if let Some(snapshot) = self.snapshot.take() {
            self.handle.with_runtime_mut(|runtime| {
                crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
            });
        }
    }
}
