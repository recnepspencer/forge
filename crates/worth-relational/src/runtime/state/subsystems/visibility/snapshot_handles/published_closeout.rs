use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

use crate::snapshots::data::SnapshotId;

use super::published_capacity::PublishedSnapshotCapacityOwner;
use super::published_registry::PublishedSnapshotHandleRegistry;

/// One published snapshot handle's release obligation.
///
/// Closing takes only the published registry's lock, through that registry's own
/// removal authority; it never observes the active registry.
#[derive(Clone, Debug)]
pub(crate) struct PublishedSnapshotCloseout {
    inner: Arc<PublishedSnapshotCloseoutInner>,
}

#[derive(Debug)]
struct PublishedSnapshotCloseoutInner {
    registry: Weak<PublishedSnapshotHandleRegistry>,
    capacity: Arc<PublishedSnapshotCapacityOwner>,
    snapshot_id: SnapshotId,
    /// Whether dropping this closeout still owes the handle a release. It is
    /// cleared when the obligation moves to a commit-result holder, because
    /// exactly one party may release one published handle.
    armed: AtomicBool,
}

impl PublishedSnapshotCloseout {
    pub(super) fn new(
        registry: &Arc<PublishedSnapshotHandleRegistry>,
        capacity: Arc<PublishedSnapshotCapacityOwner>,
        snapshot_id: SnapshotId,
    ) -> Self {
        Self {
            inner: Arc::new(PublishedSnapshotCloseoutInner {
                registry: Arc::downgrade(registry),
                capacity,
                snapshot_id,
                armed: AtomicBool::new(true),
            }),
        }
    }

    /// Hand the release obligation to the holder of the commit result that
    /// names this snapshot. Every clone of this closeout stops closing on
    /// drop, so the handle is released once by its new owner and not here.
    pub(crate) fn transfer_release_obligation(&self) {
        self.inner.armed.store(false, Ordering::Release);
    }
}

impl PartialEq for PublishedSnapshotCloseout {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for PublishedSnapshotCloseout {}

impl PublishedSnapshotCloseoutInner {
    fn close(&self) {
        let Some(registry) = self.registry.upgrade() else {
            return;
        };
        if registry.remove(self.snapshot_id).is_some() {
            self.capacity.release();
        }
    }
}

impl Drop for PublishedSnapshotCloseoutInner {
    fn drop(&mut self) {
        if self.armed.load(Ordering::Acquire) {
            self.close();
        }
    }
}
