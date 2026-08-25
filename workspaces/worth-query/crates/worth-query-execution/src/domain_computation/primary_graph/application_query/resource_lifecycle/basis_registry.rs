use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use worth_relational::facade::{
    branch::{
        AdmittedRelationalBranchBasis, RelationalBranchBasisDescriptor,
        RelationalComponentBasisRetentionLease,
    },
    history::BranchId,
    snapshots::{SnapshotHandle, SnapshotId},
};
use worth_runtime_bridge::facade::BridgePreviewSessionLivenessObserver;

use super::lifecycle_count::{acquire, record_one_saturating, release};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphIntegrationHandle;

#[derive(Default)]
struct WorthQueryApplicationBasisRegistryState {
    active: AtomicUsize,
    peak_active: AtomicUsize,
    acquisitions: AtomicUsize,
}

#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryApplicationBasisRegistry {
    state: Arc<WorthQueryApplicationBasisRegistryState>,
}

#[derive(Clone)]
pub struct WorthQueryApplicationBasisObserver {
    state: Arc<WorthQueryApplicationBasisRegistryState>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationBasisIdentity {
    runtime_instance_id: u64,
    branch_id: BranchId,
    snapshot_id: SnapshotId,
    descriptor: RelationalBranchBasisDescriptor,
}

impl WorthQueryApplicationBasisIdentity {
    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub const fn snapshot_id(&self) -> SnapshotId {
        self.snapshot_id
    }

    pub fn descriptor(&self) -> &RelationalBranchBasisDescriptor {
        &self.descriptor
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationBasisReleaseReceipt {
    identity: WorthQueryApplicationBasisIdentity,
    released: bool,
}

impl WorthQueryApplicationBasisReleaseReceipt {
    pub fn identity(&self) -> &WorthQueryApplicationBasisIdentity {
        &self.identity
    }

    pub const fn released(&self) -> bool {
        self.released
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationBasisObservation {
    active: usize,
    peak_active: usize,
    acquisitions: usize,
}

pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationBasisLease
{
    identity: WorthQueryApplicationBasisIdentity,
    basis: Option<AdmittedRelationalBranchBasis>,
    retention: Option<RelationalComponentBasisRetentionLease>,
    snapshot: Option<SnapshotHandle>,
    graph: WorthQueryPrimaryGraphIntegrationHandle,
    preview_session_liveness: Option<BridgePreviewSessionLivenessObserver>,
    state: Arc<WorthQueryApplicationBasisRegistryState>,
}

impl WorthQueryApplicationBasisRegistry {
    pub(in crate::domain_computation::primary_graph) fn observer(
        &self,
    ) -> WorthQueryApplicationBasisObserver {
        WorthQueryApplicationBasisObserver {
            state: Arc::clone(&self.state),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn register(
        &self,
        basis: AdmittedRelationalBranchBasis,
        graph: WorthQueryPrimaryGraphIntegrationHandle,
    ) -> Result<
        WorthQueryApplicationBasisLease,
        worth_relational::facade::branch::RelationalBranchBasisDenial,
    > {
        let observation = basis.observation();
        let snapshot = graph.with_runtime_mut(|runtime| {
            runtime.snapshots().snapshot_for_observation(&observation)
        })?;
        let retention = graph.with_runtime(|runtime| runtime.retain_component_basis(&basis));
        let retention = match retention {
            Ok(retention) => retention,
            Err(denial) => {
                graph.with_runtime_mut(|runtime| {
                    runtime.snapshots().release_snapshot(&snapshot);
                });
                return Err(denial);
            }
        };
        let active = acquire(&self.state.active, 1)
            .expect("live application-query basis count cannot overflow");
        record_one_saturating(&self.state.acquisitions);
        self.state.peak_active.fetch_max(active, Ordering::AcqRel);
        Ok(WorthQueryApplicationBasisLease {
            identity: WorthQueryApplicationBasisIdentity {
                runtime_instance_id: basis.identity().runtime_instance_id(),
                branch_id: basis.identity().branch_id().clone(),
                snapshot_id: snapshot.snapshot_id(),
                descriptor: basis.descriptor().clone(),
            },
            basis: Some(basis),
            retention: Some(retention),
            snapshot: Some(snapshot),
            graph,
            preview_session_liveness: None,
            state: Arc::clone(&self.state),
        })
    }
}

impl WorthQueryApplicationBasisObserver {
    pub fn observe(&self) -> WorthQueryApplicationBasisObservation {
        WorthQueryApplicationBasisObservation {
            active: self.state.active.load(Ordering::Acquire),
            peak_active: self.state.peak_active.load(Ordering::Acquire),
            acquisitions: self.state.acquisitions.load(Ordering::Acquire),
        }
    }
}

impl WorthQueryApplicationBasisObservation {
    pub const fn active(self) -> usize {
        self.active
    }

    pub const fn peak_active(self) -> usize {
        self.peak_active
    }

    pub const fn acquisitions(self) -> usize {
        self.acquisitions
    }
}

impl WorthQueryApplicationBasisLease {
    pub(in crate::domain_computation::primary_graph::application_query) fn bind_preview_session(
        mut self,
        liveness: BridgePreviewSessionLivenessObserver,
    ) -> Self {
        self.preview_session_liveness = Some(liveness);
        self
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn preview_session_liveness(
        &self,
    ) -> Option<&BridgePreviewSessionLivenessObserver> {
        self.preview_session_liveness.as_ref()
    }

    pub fn identity(&self) -> &WorthQueryApplicationBasisIdentity {
        &self.identity
    }

    pub fn version_id(&self) -> worth_relational::facade::identity::VersionId {
        self.basis().observation().version_id()
    }

    pub fn snapshot_handle(&self) -> &SnapshotHandle {
        self.snapshot
            .as_ref()
            .expect("an active application-query basis retains its snapshot")
    }

    pub fn is_live(&self) -> bool {
        self.basis.is_some()
            && self.snapshot.as_ref().is_some_and(|snapshot| {
                self.graph.with_runtime(|runtime| {
                    runtime.read_truth().project_snapshot(snapshot).is_some()
                })
            })
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn retain_for_continuation(
        &self,
    ) -> Result<
        RelationalComponentBasisRetentionLease,
        worth_relational::facade::branch::RelationalBranchBasisDenial,
    > {
        self.graph
            .with_runtime(|runtime| runtime.retain_component_basis(self.basis()))
    }

    pub fn release(mut self) -> WorthQueryApplicationBasisReleaseReceipt {
        let released = self.release_snapshot();
        let retained_released = self
            .retention
            .take()
            .is_some_and(|lease| lease.release().descriptor() == self.identity.descriptor());
        self.basis.take();
        self.release_observation();
        WorthQueryApplicationBasisReleaseReceipt {
            identity: self.identity.clone(),
            released: released && retained_released,
        }
    }

    fn basis(&self) -> &AdmittedRelationalBranchBasis {
        self.basis
            .as_ref()
            .expect("an active application-query basis retains its Relational observation")
    }

    fn release_snapshot(&mut self) -> bool {
        self.snapshot.take().is_some_and(|snapshot| {
            self.graph
                .with_runtime_mut(|runtime| runtime.snapshots().release_snapshot(&snapshot))
        })
    }

    fn release_observation(&self) {
        release(&self.state.active, 1)
            .expect("live application-query basis count cannot underflow");
    }
}

impl Drop for WorthQueryApplicationBasisLease {
    fn drop(&mut self) {
        if self.basis.take().is_some() {
            let _ = self.release_snapshot();
            self.retention.take();
            self.release_observation();
        }
    }
}
