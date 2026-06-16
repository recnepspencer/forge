use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_RUNTIME_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRuntimeInstanceId(u64);

impl WorthUiRuntimeInstanceId {
    pub(crate) fn next() -> Self {
        Self(NEXT_RUNTIME_INSTANCE_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn raw(self) -> u64 {
        self.0
    }
}
