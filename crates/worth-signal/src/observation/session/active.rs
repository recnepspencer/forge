use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::request::SignalObservationRequest;

/// Graph-owned cleanup for a dropped observation token.
///
/// The token deliberately carries only this narrow capability.  It cannot
/// finish a session or mint a receipt, but it can clear the selected capture
/// stores when its owner is abandoned before an explicit terminal transition.
pub(crate) trait SignalObservationDropCleanup: std::fmt::Debug + Send + Sync {
    fn clear(&self, request: SignalObservationRequest);
}

#[derive(Debug)]
pub struct SignalObservationSession {
    pub(crate) graph_instance: u64,
    pub(crate) generation: u64,
    pub(crate) request: SignalObservationRequest,
    pub(crate) liveness: Arc<AtomicU64>,
    pub(crate) drop_cleanup: Arc<dyn SignalObservationDropCleanup>,
}

impl SignalObservationSession {
    pub const fn request(&self) -> SignalObservationRequest {
        self.request
    }

    pub const fn graph_instance(&self) -> u64 {
        self.graph_instance
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }
}

impl Drop for SignalObservationSession {
    fn drop(&mut self) {
        let cleared = self
            .liveness
            .compare_exchange(self.generation, 0, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        if cleared {
            self.drop_cleanup.clear(self.request);
        }
    }
}
