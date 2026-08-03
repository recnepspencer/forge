use std::sync::atomic::{AtomicUsize, Ordering};

use tracking_allocator::{
    AllocationGroupId, AllocationRegistry, AllocationTracker, SetTrackerError,
};

static LARGEST_SUCCESSFUL_REQUEST_BYTES: AtomicUsize = AtomicUsize::new(0);

struct ProcessAllocationTracker;

impl AllocationTracker for ProcessAllocationTracker {
    fn allocated(
        &self,
        _address: usize,
        object_size: usize,
        _wrapped_size: usize,
        _group: AllocationGroupId,
    ) {
        LARGEST_SUCCESSFUL_REQUEST_BYTES.fetch_max(object_size, Ordering::SeqCst);
    }

    fn deallocated(
        &self,
        _address: usize,
        _object_size: usize,
        _wrapped_size: usize,
        _source_group: AllocationGroupId,
        _current_group: AllocationGroupId,
    ) {
    }
}

pub(super) struct ProcessAllocationEpoch {
    finished: bool,
}

impl ProcessAllocationEpoch {
    pub(super) fn begin() -> Result<Self, String> {
        install_tracker().map_err(|error| {
            format!("process allocation tracker installation was not unique: {error}")
        })?;
        LARGEST_SUCCESSFUL_REQUEST_BYTES.store(0, Ordering::SeqCst);
        AllocationRegistry::enable_tracking();
        Ok(Self { finished: false })
    }

    pub(super) fn finish(mut self) -> ProcessAllocationEvidence {
        AllocationRegistry::disable_tracking();
        let evidence = ProcessAllocationEvidence {
            largest_successful_request_bytes: LARGEST_SUCCESSFUL_REQUEST_BYTES
                .load(Ordering::SeqCst),
        };
        self.finished = true;
        evidence
    }
}

impl Drop for ProcessAllocationEpoch {
    fn drop(&mut self) {
        if !self.finished {
            AllocationRegistry::disable_tracking();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ProcessAllocationEvidence {
    largest_successful_request_bytes: usize,
}

impl ProcessAllocationEvidence {
    pub(super) const fn largest_successful_request_bytes(self) -> usize {
        self.largest_successful_request_bytes
    }
}

fn install_tracker() -> Result<(), SetTrackerError> {
    AllocationRegistry::set_global_tracker(ProcessAllocationTracker)
}
