use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// The bounded admission authority for published snapshot handles.
///
/// Capacity is accounted separately from the registry itself so admission is a
/// nonblocking reservation that never touches a registry lock.
#[derive(Debug)]
pub(crate) struct PublishedSnapshotCapacityOwner {
    maximum_handles: usize,
    occupied_handles: AtomicUsize,
}

impl PublishedSnapshotCapacityOwner {
    pub(crate) fn new(maximum_handles: usize) -> Arc<Self> {
        Arc::new(Self {
            maximum_handles,
            occupied_handles: AtomicUsize::new(0),
        })
    }

    pub(crate) fn reserve(self: &Arc<Self>) -> Result<PublishedSnapshotSlotReservation, usize> {
        let mut occupied = self.occupied_handles.load(Ordering::Acquire);
        loop {
            if occupied >= self.maximum_handles {
                return Err(self.maximum_handles);
            }
            match self.occupied_handles.compare_exchange_weak(
                occupied,
                occupied + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    return Ok(PublishedSnapshotSlotReservation {
                        owner: Arc::clone(self),
                        releases_on_drop: true,
                    });
                }
                Err(observed) => occupied = observed,
            }
        }
    }

    pub(crate) const fn maximum_handles(&self) -> usize {
        self.maximum_handles
    }

    pub(crate) fn occupied_handles(&self) -> usize {
        self.occupied_handles.load(Ordering::Acquire)
    }

    pub(crate) fn release(&self) {
        let previous = self.occupied_handles.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "published snapshot capacity underflow");
    }
}

#[derive(Debug)]
#[must_use = "published snapshot capacity reservations must be installed or released"]
pub(crate) struct PublishedSnapshotSlotReservation {
    owner: Arc<PublishedSnapshotCapacityOwner>,
    releases_on_drop: bool,
}

impl PublishedSnapshotSlotReservation {
    pub(crate) fn install(mut self) {
        self.releases_on_drop = false;
    }
}

impl Drop for PublishedSnapshotSlotReservation {
    fn drop(&mut self) {
        if self.releases_on_drop {
            self.owner.release();
        }
    }
}
