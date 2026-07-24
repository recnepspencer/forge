use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use worth_query::facade::domain;

#[derive(Clone, Default)]
pub struct ArtifactProbe {
    inner: Arc<ArtifactProbeInner>,
}

#[derive(Default)]
struct ArtifactProbeInner {
    allocations: AtomicUsize,
    projection_calls: AtomicUsize,
    borrow_observations: AtomicUsize,
    disposals: AtomicUsize,
    replacements: AtomicUsize,
    cancellations: AtomicUsize,
    native_row_counts: AtomicUsize,
    native_row_batches: AtomicUsize,
    native_field_slices: AtomicUsize,
    native_projections: AtomicUsize,
    native_scalars: AtomicUsize,
    denials: Mutex<Vec<domain::WorthQueryArtifactDenialKind>>,
    lifecycle_snapshots: Mutex<Vec<domain::WorthQueryArtifactOwnerSnapshot>>,
    consumer_mode: Mutex<Option<String>>,
    retained_admission: Mutex<Option<domain::WorthQueryArtifactProductionAdmission>>,
    escaped_handle: Mutex<Option<domain::WorthQueryTransferredArtifactHandle>>,
    escaped_lease: Mutex<Option<domain::WorthQueryRetainedArtifactLease>>,
}

impl ArtifactProbe {
    pub fn allocations(&self) -> usize {
        self.inner.allocations.load(Ordering::SeqCst)
    }

    pub fn projection_calls(&self) -> usize {
        self.inner.projection_calls.load(Ordering::SeqCst)
    }

    pub fn borrow_observations(&self) -> usize {
        self.inner.borrow_observations.load(Ordering::SeqCst)
    }

    pub fn disposals(&self) -> usize {
        self.inner.disposals.load(Ordering::SeqCst)
    }

    pub fn replacements(&self) -> usize {
        self.inner.replacements.load(Ordering::SeqCst)
    }

    pub fn cancellations(&self) -> usize {
        self.inner.cancellations.load(Ordering::SeqCst)
    }

    pub fn native_row_counts(&self) -> usize {
        self.inner.native_row_counts.load(Ordering::SeqCst)
    }

    pub fn native_row_batches(&self) -> usize {
        self.inner.native_row_batches.load(Ordering::SeqCst)
    }

    pub fn native_field_slices(&self) -> usize {
        self.inner.native_field_slices.load(Ordering::SeqCst)
    }

    pub fn native_projections(&self) -> usize {
        self.inner.native_projections.load(Ordering::SeqCst)
    }

    pub fn native_scalars(&self) -> usize {
        self.inner.native_scalars.load(Ordering::SeqCst)
    }

    pub fn denials(&self) -> Vec<domain::WorthQueryArtifactDenialKind> {
        self.inner
            .denials
            .lock()
            .expect("artifact denial probe lock remains available")
            .clone()
    }

    pub fn lifecycle_snapshots(&self) -> Vec<domain::WorthQueryArtifactOwnerSnapshot> {
        self.inner
            .lifecycle_snapshots
            .lock()
            .expect("artifact lifecycle snapshot probe lock remains available")
            .clone()
    }

    pub fn take_escaped_handle(&self) -> Option<domain::WorthQueryTransferredArtifactHandle> {
        self.inner
            .escaped_handle
            .lock()
            .expect("escaped artifact handle probe lock remains available")
            .take()
    }

    pub fn take_escaped_lease(&self) -> Option<domain::WorthQueryRetainedArtifactLease> {
        self.inner
            .escaped_lease
            .lock()
            .expect("escaped artifact lease probe lock remains available")
            .take()
    }

    pub(super) fn candidate(&self, projection: &[u8]) -> CandidateArtifactResource {
        self.inner.allocations.fetch_add(1, Ordering::SeqCst);
        CandidateArtifactResource {
            probe: self.clone(),
            projection: projection.to_vec(),
            retained_bytes: projection.len() * 16,
        }
    }

    pub(super) fn foreign(&self, projection: &[u8]) -> ForeignArtifactResource {
        self.inner.allocations.fetch_add(1, Ordering::SeqCst);
        ForeignArtifactResource {
            probe: self.clone(),
            projection: projection.to_vec(),
        }
    }

    pub(super) fn panic_during_projection(&self) -> PanicDuringProjectionResource {
        self.inner.allocations.fetch_add(1, Ordering::SeqCst);
        PanicDuringProjectionResource {
            probe: self.clone(),
        }
    }

    pub(super) fn observe_borrow(&self) {
        self.inner
            .borrow_observations
            .fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_replacement(&self) {
        self.inner.replacements.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_cancellation(&self) {
        self.inner.cancellations.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_native_row_count(&self) {
        self.inner.native_row_counts.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_native_row_batch(&self) {
        self.inner.native_row_batches.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_native_field_slice(&self) {
        self.inner
            .native_field_slices
            .fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_native_projection(&self) {
        self.inner.native_projections.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_native_scalar(&self) {
        self.inner.native_scalars.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn observe_denial(&self, kind: domain::WorthQueryArtifactDenialKind) {
        self.inner
            .denials
            .lock()
            .expect("artifact denial probe lock remains available")
            .push(kind);
    }

    pub(super) fn observe_lifecycle(&self, snapshot: domain::WorthQueryArtifactOwnerSnapshot) {
        self.inner
            .lifecycle_snapshots
            .lock()
            .expect("artifact lifecycle snapshot probe lock remains available")
            .push(snapshot);
    }

    pub(super) fn arm_consumer(&self, mode: String) {
        *self
            .inner
            .consumer_mode
            .lock()
            .expect("artifact consumer mode probe lock remains available") = Some(mode);
    }

    pub(super) fn take_consumer_mode(&self) -> Option<String> {
        self.inner
            .consumer_mode
            .lock()
            .expect("artifact consumer mode probe lock remains available")
            .take()
    }

    pub(super) fn retain_admission(
        &self,
        admission: domain::WorthQueryArtifactProductionAdmission,
    ) {
        *self
            .inner
            .retained_admission
            .lock()
            .expect("retained artifact admission probe lock remains available") = Some(admission);
    }

    pub(super) fn take_retained_admission(
        &self,
    ) -> Option<domain::WorthQueryArtifactProductionAdmission> {
        self.inner
            .retained_admission
            .lock()
            .expect("retained artifact admission probe lock remains available")
            .take()
    }

    pub(super) fn escape_handle(&self, handle: domain::WorthQueryTransferredArtifactHandle) {
        *self
            .inner
            .escaped_handle
            .lock()
            .expect("escaped artifact handle probe lock remains available") = Some(handle);
    }

    pub(super) fn escape_lease(&self, lease: domain::WorthQueryRetainedArtifactLease) {
        *self
            .inner
            .escaped_lease
            .lock()
            .expect("escaped artifact lease probe lock remains available") = Some(lease);
    }
}

pub(super) struct CandidateArtifactResource {
    probe: ArtifactProbe,
    projection: Vec<u8>,
    retained_bytes: usize,
}

impl domain::WorthQueryArtifactProviderResource for CandidateArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.artifact-workflow.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        self.probe
            .inner
            .projection_calls
            .fetch_add(1, Ordering::SeqCst);
        self.projection.clone()
    }

    fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    fn dispose(self) {
        self.probe.inner.disposals.fetch_add(1, Ordering::SeqCst);
    }
}

pub(super) struct ForeignArtifactResource {
    probe: ArtifactProbe,
    projection: Vec<u8>,
}

pub(super) struct PanicDuringProjectionResource {
    probe: ArtifactProbe,
}

impl domain::WorthQueryArtifactProviderResource for ForeignArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.artifact-workflow.foreign-provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        self.probe
            .inner
            .projection_calls
            .fetch_add(1, Ordering::SeqCst);
        self.projection.clone()
    }

    fn retained_bytes(&self) -> usize {
        self.projection.len()
    }

    fn dispose(self) {
        self.probe.inner.disposals.fetch_add(1, Ordering::SeqCst);
    }
}

impl domain::WorthQueryArtifactProviderResource for PanicDuringProjectionResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.artifact-workflow.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        self.probe
            .inner
            .projection_calls
            .fetch_add(1, Ordering::SeqCst);
        panic!("provider projection panic");
    }

    fn retained_bytes(&self) -> usize {
        1
    }

    fn dispose(self) {
        self.probe.inner.disposals.fetch_add(1, Ordering::SeqCst);
    }
}
