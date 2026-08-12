use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) enum FixtureYieldRecoveryArtifact {
    Cooperative,
    DoublePanicking,
}

#[derive(Clone, Default)]
pub(crate) struct FixtureYieldRecoveryProbe {
    suspension_attempts: Arc<AtomicUsize>,
    disposal_attempts: Arc<AtomicUsize>,
    destructor_attempts: Arc<AtomicUsize>,
}

impl FixtureYieldRecoveryProbe {
    pub(crate) fn suspension_attempt_count(&self) -> usize {
        self.suspension_attempts.load(Ordering::Acquire)
    }

    pub(crate) fn disposal_attempt_count(&self) -> usize {
        self.disposal_attempts.load(Ordering::Acquire)
    }

    pub(crate) fn destructor_attempt_count(&self) -> usize {
        self.destructor_attempts.load(Ordering::Acquire)
    }

    pub(super) fn attempted_suspension(&self) {
        self.suspension_attempts.fetch_add(1, Ordering::AcqRel);
    }

    pub(super) fn attempted_disposal(&self) {
        self.disposal_attempts.fetch_add(1, Ordering::AcqRel);
    }

    pub(super) fn attempted_destructor(&self) {
        self.destructor_attempts.fetch_add(1, Ordering::AcqRel);
    }
}
