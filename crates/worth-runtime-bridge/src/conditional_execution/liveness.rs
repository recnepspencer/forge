use std::sync::atomic::{AtomicBool, Ordering};

pub(super) struct BridgeConditionalLoweringLease {
    live: AtomicBool,
}

impl BridgeConditionalLoweringLease {
    pub(super) fn issue() -> Self {
        Self {
            live: AtomicBool::new(true),
        }
    }

    pub(super) fn is_live(&self) -> bool {
        self.live.load(Ordering::Acquire)
    }

    pub(super) fn revoke_liveness(&self) {
        self.live.store(false, Ordering::Release);
    }
}
