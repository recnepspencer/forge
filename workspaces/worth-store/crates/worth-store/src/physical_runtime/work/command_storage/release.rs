use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

const RELEASE_ACTIVE: u8 = 0;
const RELEASE_ABANDONMENT_OWNED: u8 = 1;
const RELEASED: u8 = 2;

pub(in crate::physical_runtime::work) struct PhysicalCommandRelease {
    release_state: AtomicU8,
    cancelled: AtomicBool,
    consumer_cancelled: AtomicBool,
}

impl PhysicalCommandRelease {
    pub(in crate::physical_runtime::work) fn new() -> Self {
        Self {
            release_state: AtomicU8::new(RELEASE_ACTIVE),
            cancelled: AtomicBool::new(false),
            consumer_cancelled: AtomicBool::new(false),
        }
    }

    pub(in crate::physical_runtime::work) fn claim_abandonment(&self) -> bool {
        self.release_state
            .compare_exchange(
                RELEASE_ACTIVE,
                RELEASE_ABANDONMENT_OWNED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub(in crate::physical_runtime::work) fn claim_release(&self) -> bool {
        self.release_state
            .compare_exchange(
                RELEASE_ACTIVE,
                RELEASED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub(in crate::physical_runtime::work) fn complete_abandonment(&self) -> bool {
        self.release_state
            .compare_exchange(
                RELEASE_ABANDONMENT_OWNED,
                RELEASED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub(in crate::physical_runtime::work) fn claim_shutdown_release(&self) -> bool {
        self.release_state.swap(RELEASED, Ordering::AcqRel) != RELEASED
    }

    pub(in crate::physical_runtime::work) fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.consumer_cancelled.store(true, Ordering::Release);
    }

    pub(in crate::physical_runtime::work) fn mark_consumer_cancelled(&self) {
        self.consumer_cancelled.store(true, Ordering::Release);
    }

    pub(in crate::physical_runtime::work) fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    pub(in crate::physical_runtime::work) fn consumer_cancelled(&self) -> bool {
        self.consumer_cancelled.load(Ordering::Acquire)
    }
}
