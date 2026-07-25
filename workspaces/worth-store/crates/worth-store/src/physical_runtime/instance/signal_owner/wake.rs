use std::sync::{
    atomic::{AtomicBool, Ordering},
    Condvar, Mutex,
};

pub(super) struct PhysicalSignalWorkerWake {
    signalled: Mutex<bool>,
    changed: Condvar,
}

impl PhysicalSignalWorkerWake {
    pub(super) fn new() -> Self {
        Self {
            signalled: Mutex::new(false),
            changed: Condvar::new(),
        }
    }

    pub(super) fn signal(&self) {
        *self
            .signalled
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = true;
        self.changed.notify_one();
    }

    pub(super) fn wait(&self, stopping: &AtomicBool) {
        let mut signalled = self
            .signalled
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while !*signalled && !stopping.load(Ordering::Acquire) {
            signalled = self
                .changed
                .wait(signalled)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        *signalled = false;
    }
}

impl crate::physical_runtime::work::PhysicalWorkAbandonmentWake for PhysicalSignalWorkerWake {
    fn wake(&self) {
        self.signal();
    }
}
