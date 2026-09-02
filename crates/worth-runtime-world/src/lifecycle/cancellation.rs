use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Caller-owned cancellation source for one Runtime World publication.
#[derive(Debug, Clone, Default)]
pub struct RuntimeWorldCancellationSource {
    requested: Arc<AtomicBool>,
}

/// Read-only cancellation token borrowed by a single owner transition.
#[derive(Debug, Clone)]
pub struct RuntimeWorldCancellationToken {
    requested: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeWorldCancellationBoundary {
    BeforeReservation,
    BeforeFirstOwnerEffect,
    BetweenOwnerEffects,
    BeforeProductMovement,
    AfterProductMovement,
}

impl RuntimeWorldCancellationSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(&self) -> RuntimeWorldCancellationToken {
        RuntimeWorldCancellationToken {
            requested: Arc::clone(&self.requested),
        }
    }

    pub fn cancel(&self) {
        self.requested.store(true, Ordering::Release);
    }
}

impl RuntimeWorldCancellationToken {
    pub fn is_cancelled(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }

    pub(crate) fn check(&self, _boundary: RuntimeWorldCancellationBoundary) -> Result<(), ()> {
        if self.is_cancelled() {
            Err(())
        } else {
            Ok(())
        }
    }
}
