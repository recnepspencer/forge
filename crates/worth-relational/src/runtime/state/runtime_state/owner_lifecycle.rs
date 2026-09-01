use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

/// Drop-governed lifecycle authority shared by every independently borrowable
/// service issued by one Relational runtime.
#[derive(Debug)]
pub(in crate::runtime) struct RelationalRuntimeOwner {
    binding: RelationalRuntimeOwnerBinding,
}

/// Cloneable lifecycle binding carried by narrow runtime-owned services.
#[derive(Debug, Clone)]
pub(crate) struct RelationalRuntimeOwnerBinding {
    lifecycle: Arc<RelationalRuntimeLifecycle>,
}

#[derive(Debug)]
struct RelationalRuntimeLifecycle {
    accepting_operations: AtomicBool,
    in_flight: AtomicUsize,
    close_wait: Mutex<()>,
    close_ready: Condvar,
}

#[derive(Debug)]
pub(crate) struct AdmittedRelationalRuntimeOperation {
    lifecycle: Arc<RelationalRuntimeLifecycle>,
}

impl RelationalRuntimeOwner {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            binding: RelationalRuntimeOwnerBinding {
                lifecycle: Arc::new(RelationalRuntimeLifecycle {
                    accepting_operations: AtomicBool::new(true),
                    in_flight: AtomicUsize::new(0),
                    close_wait: Mutex::new(()),
                    close_ready: Condvar::new(),
                }),
            },
        }
    }

    pub(super) fn binding(&self) -> RelationalRuntimeOwnerBinding {
        self.binding.clone()
    }
}

impl RelationalRuntimeOwnerBinding {
    /// Stop admitting operations and wait for the admitted ones to return.
    ///
    /// This is owner authority, so it stays reachable only inside the runtime
    /// module tree: a narrow service carries this binding to admit work and can
    /// never use it to close the runtime it borrows.
    pub(in crate::runtime) fn close(&self) {
        self.lifecycle
            .accepting_operations
            .store(false, Ordering::Release);
        let mut wait = self
            .lifecycle
            .close_wait
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while self.lifecycle.in_flight.load(Ordering::Acquire) != 0 {
            wait = self
                .lifecycle
                .close_ready
                .wait(wait)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
    }

    pub(crate) fn admit(&self) -> Option<AdmittedRelationalRuntimeOperation> {
        if !self.lifecycle.accepting_operations.load(Ordering::Acquire) {
            return None;
        }
        self.lifecycle.in_flight.fetch_add(1, Ordering::AcqRel);
        if !self.lifecycle.accepting_operations.load(Ordering::Acquire) {
            release_operation(&self.lifecycle);
            return None;
        }
        Some(AdmittedRelationalRuntimeOperation {
            lifecycle: Arc::clone(&self.lifecycle),
        })
    }

    /// Observe whether this owner still accepts work without admitting any.
    ///
    /// This is descriptive state only. It carries no close authority and does
    /// not increment or otherwise participate in the in-flight drain.
    pub(crate) fn accepts_operations(&self) -> bool {
        self.lifecycle.accepting_operations.load(Ordering::Acquire)
    }
}

impl Drop for AdmittedRelationalRuntimeOperation {
    fn drop(&mut self) {
        release_operation(&self.lifecycle);
    }
}

fn release_operation(lifecycle: &RelationalRuntimeLifecycle) {
    let previous = lifecycle.in_flight.fetch_sub(1, Ordering::AcqRel);
    debug_assert!(previous > 0, "runtime operation admission underflow");
    if previous == 1 && !lifecycle.accepting_operations.load(Ordering::Acquire) {
        let _wait = lifecycle
            .close_wait
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lifecycle.close_ready.notify_all();
    }
}
