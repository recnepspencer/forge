use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::operation_counters::{increment, MediaCounterCells};

impl MediaCounterCells {
    pub(super) fn file_handle_opened(
        self: &Arc<Self>,
        kind: super::NamespaceFileOpenKind,
    ) -> MediaFileHandleAccounting {
        match kind {
            super::NamespaceFileOpenKind::Existing => increment(&self.file_opens, 1),
            super::NamespaceFileOpenKind::CreatedNew => increment(&self.file_creates, 1),
        }
        acquire_live_handle(&self.live_file_handles, &self.peak_file_handles);
        MediaFileHandleAccounting {
            counters: Arc::clone(self),
        }
    }

    pub(super) fn directory_handle_opened(self: &Arc<Self>) -> MediaDirectoryHandleAccounting {
        increment(&self.directory_opens, 1);
        acquire_live_handle(&self.live_directory_handles, &self.peak_directory_handles);
        MediaDirectoryHandleAccounting {
            counters: Arc::clone(self),
        }
    }
}

#[derive(Debug)]
pub(super) struct MediaFileHandleAccounting {
    counters: Arc<MediaCounterCells>,
}

impl Drop for MediaFileHandleAccounting {
    fn drop(&mut self) {
        increment(&self.counters.file_closes, 1);
        release_live_handle(&self.counters.live_file_handles);
    }
}

#[derive(Debug)]
pub(super) struct MediaDirectoryHandleAccounting {
    counters: Arc<MediaCounterCells>,
}

impl Drop for MediaDirectoryHandleAccounting {
    fn drop(&mut self) {
        increment(&self.counters.directory_closes, 1);
        release_live_handle(&self.counters.live_directory_handles);
    }
}

fn acquire_live_handle(live: &AtomicU64, peak: &AtomicU64) {
    let current = live
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
            Some(value.saturating_add(1))
        })
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let _ = peak.fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
        (current > value).then_some(current)
    });
}

fn release_live_handle(live: &AtomicU64) {
    let _ = live.fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
        value.checked_sub(1)
    });
}
