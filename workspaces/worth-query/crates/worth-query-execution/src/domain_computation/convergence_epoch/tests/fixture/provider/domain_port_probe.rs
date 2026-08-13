use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Default)]
pub(crate) struct FixtureDomainPortProbe {
    comparator_entries: Arc<AtomicUsize>,
    progress_entries: Arc<AtomicUsize>,
    repeated_state_entries: Arc<AtomicUsize>,
}

impl FixtureDomainPortProbe {
    pub(super) fn entered_comparator(&self) {
        self.comparator_entries.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn entered_progress(&self) {
        self.progress_entries.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn entered_repeated_state(&self) {
        self.repeated_state_entries.fetch_add(1, Ordering::SeqCst);
    }

    pub(crate) fn entries(&self) -> [usize; 3] {
        [
            self.comparator_entries.load(Ordering::SeqCst),
            self.progress_entries.load(Ordering::SeqCst),
            self.repeated_state_entries.load(Ordering::SeqCst),
        ]
    }
}
