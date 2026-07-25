use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Read-only liveness fact shared with physical work admission.
///
/// This is derived availability, not Signal mutation authority. The owner
/// revokes it during shutdown, and every worker revokes it on exit, including
/// panic unwinding.
#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalSignalAdmissionStatus {
    available: Arc<AtomicBool>,
}

impl PhysicalSignalAdmissionStatus {
    pub(super) fn available() -> Self {
        Self {
            available: Arc::new(AtomicBool::new(true)),
        }
    }

    pub(in crate::physical_runtime) fn is_available(&self) -> bool {
        self.available.load(Ordering::Acquire)
    }

    pub(in crate::physical_runtime) fn revoke(&self) {
        self.available.store(false, Ordering::Release);
    }
}
