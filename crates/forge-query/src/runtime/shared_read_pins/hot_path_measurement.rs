use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct ForgeQuerySharedReadHotPathMeasurement {
    committed_read_hot_path_lock_count: Arc<AtomicUsize>,
}

impl ForgeQuerySharedReadHotPathMeasurement {
    #[allow(dead_code)]
    pub(in crate::runtime) fn record_committed_read_hot_path_lock(&self) {
        self.committed_read_hot_path_lock_count
            .fetch_add(1, Ordering::SeqCst);
    }

    pub(in crate::runtime) fn committed_read_hot_path_lock_count(&self) -> usize {
        self.committed_read_hot_path_lock_count
            .load(Ordering::SeqCst)
    }
}
