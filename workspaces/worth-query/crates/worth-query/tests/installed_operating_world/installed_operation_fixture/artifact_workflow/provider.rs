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
    denials: Mutex<Vec<domain::WorthQueryArtifactDenialKind>>,
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

    pub fn denials(&self) -> Vec<domain::WorthQueryArtifactDenialKind> {
        self.inner
            .denials
            .lock()
            .expect("artifact denial probe lock remains available")
            .clone()
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

    pub(super) fn observe_denial(&self, kind: domain::WorthQueryArtifactDenialKind) {
        self.inner
            .denials
            .lock()
            .expect("artifact denial probe lock remains available")
            .push(kind);
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
