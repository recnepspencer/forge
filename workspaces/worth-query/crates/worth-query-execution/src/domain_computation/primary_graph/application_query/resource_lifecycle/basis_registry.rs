use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use worth_relational::facade::{
    runtime::{
        RelationalExecutionBasisIdentity, RelationalExecutionBasisLease,
        RelationalExecutionBasisReleaseReceipt,
    },
    snapshots::SnapshotHandle,
};
use worth_runtime_bridge::facade::BridgePreviewSessionLivenessObserver;

use super::lifecycle_count::{acquire, record_one_saturating, release};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationBasisObservation {
    active: usize,
    peak_active: usize,
    acquisitions: usize,
}

pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationBasisLease
{
    lease: Option<RelationalExecutionBasisLease>,
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
        lease: RelationalExecutionBasisLease,
    ) -> WorthQueryApplicationBasisLease {
        let active = acquire(&self.state.active, 1)
            .expect("live application-query basis count cannot overflow");
        record_one_saturating(&self.state.acquisitions);
        self.state.peak_active.fetch_max(active, Ordering::AcqRel);
        WorthQueryApplicationBasisLease {
            lease: Some(lease),
            preview_session_liveness: None,
            state: Arc::clone(&self.state),
        }
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

    pub fn identity(&self) -> &RelationalExecutionBasisIdentity {
        self.lease().identity()
    }

    pub fn version_id(&self) -> worth_relational::facade::identity::VersionId {
        self.lease().version_id()
    }

    pub fn snapshot_handle(&self) -> &SnapshotHandle {
        self.lease().snapshot_handle()
    }

    pub fn is_live(&self) -> bool {
        self.lease().is_live()
    }

    pub fn release(mut self) -> RelationalExecutionBasisReleaseReceipt {
        let receipt = self
            .lease
            .take()
            .expect("an application-query basis releases once")
            .release();
        self.release_observation();
        receipt
    }

    fn lease(&self) -> &RelationalExecutionBasisLease {
        self.lease
            .as_ref()
            .expect("an active application-query basis retains its Relational lease")
    }

    fn release_observation(&self) {
        release(&self.state.active, 1)
            .expect("live application-query basis count cannot underflow");
    }
}

impl Drop for WorthQueryApplicationBasisLease {
    fn drop(&mut self) {
        if self.lease.take().is_some() {
            self.release_observation();
        }
    }
}
