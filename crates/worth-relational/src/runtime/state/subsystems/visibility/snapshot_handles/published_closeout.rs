use std::sync::{Arc, Weak};

use crate::snapshots::data::SnapshotId;

use super::published_capacity::PublishedSnapshotCapacityOwner;
use super::published_registry::PublishedSnapshotHandleRegistry;

/// One published snapshot handle's release obligation.
///
/// The obligation is move-only. Exactly one party holds it: the runtime's own
/// pending settlement record, until a commit-result claimant takes it. It is
/// deliberately not `Clone`, because a second holder would extend the
/// obligation past its owner's lifetime and turn a runtime release into
/// whichever holder happened to drop last.
///
/// Closing takes only the published registry's lock, through that registry's own
/// removal authority; it never observes the active registry.
#[derive(Debug)]
pub(crate) struct PublishedSnapshotCloseout {
    registry: Weak<PublishedSnapshotHandleRegistry>,
    capacity: Arc<PublishedSnapshotCapacityOwner>,
    snapshot_id: SnapshotId,
    /// Whether dropping this closeout still owes the handle a release. It is
    /// cleared when the obligation moves to a commit-result holder, because
    /// exactly one party may release one published handle.
    armed: bool,
}

impl PublishedSnapshotCloseout {
    pub(super) fn new(
        registry: &Arc<PublishedSnapshotHandleRegistry>,
        capacity: Arc<PublishedSnapshotCapacityOwner>,
        snapshot_id: SnapshotId,
    ) -> Self {
        Self {
            registry: Arc::downgrade(registry),
            capacity,
            snapshot_id,
            armed: true,
        }
    }

    /// Hand the release obligation to the holder of the commit result that
    /// names this snapshot. Consuming the obligation is the transfer: this
    /// closeout stops closing, so the handle is released once by its new owner
    /// and not here.
    pub(crate) fn transfer_release_obligation(mut self) {
        self.armed = false;
    }

    fn close(&self) {
        let Some(registry) = self.registry.upgrade() else {
            return;
        };
        if registry.remove(self.snapshot_id).is_some() {
            self.capacity.release();
        }
    }
}

impl Drop for PublishedSnapshotCloseout {
    fn drop(&mut self) {
        if self.armed {
            self.close();
        }
    }
}
