use std::sync::{Arc, Mutex};

use crate::ProductionStorageBoundarySeam;

use super::{StorageBoundaryFault, StorageBoundaryTrace};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageBoundaryExecutionIdentity(u64);

pub trait ProductionStorageBoundaryControl {
    fn fault_at(&self, seam: ProductionStorageBoundarySeam) -> Option<StorageBoundaryFault>;

    fn record_reached(&self, _seam: ProductionStorageBoundarySeam) {}

    fn record_injected(&self, _seam: ProductionStorageBoundarySeam, _fault: StorageBoundaryFault) {}

    fn execution_identity(&self) -> Option<StorageBoundaryExecutionIdentity> {
        None
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UninterruptedStorageBoundaryControl;

#[cfg(feature = "certification-test-authority")]
#[derive(Debug, Clone, Copy)]
pub struct ProcessCrashStorageBoundaryControl {
    seam: ProductionStorageBoundarySeam,
}

#[derive(Debug, Clone)]
pub struct ScriptedStorageBoundaryControl {
    execution_identity: StorageBoundaryExecutionIdentity,
    seam: ProductionStorageBoundarySeam,
    fault: Option<StorageBoundaryFault>,
    trace: Arc<Mutex<StorageBoundaryTrace>>,
}

impl ProductionStorageBoundaryControl for UninterruptedStorageBoundaryControl {
    fn fault_at(&self, _seam: ProductionStorageBoundarySeam) -> Option<StorageBoundaryFault> {
        None
    }
}

#[cfg(feature = "certification-test-authority")]
impl ProcessCrashStorageBoundaryControl {
    pub const fn at(seam: ProductionStorageBoundarySeam) -> Self {
        Self { seam }
    }
}

#[cfg(feature = "certification-test-authority")]
impl ProductionStorageBoundaryControl for ProcessCrashStorageBoundaryControl {
    fn fault_at(&self, _seam: ProductionStorageBoundarySeam) -> Option<StorageBoundaryFault> {
        None
    }

    fn record_reached(&self, seam: ProductionStorageBoundarySeam) {
        if seam == self.seam {
            std::process::abort();
        }
    }
}

impl ScriptedStorageBoundaryControl {
    pub fn observe(seam: ProductionStorageBoundarySeam) -> Self {
        Self::new(seam, None)
    }

    pub fn inject(seam: ProductionStorageBoundarySeam, fault: StorageBoundaryFault) -> Self {
        Self::new(seam, Some(fault))
    }

    fn new(seam: ProductionStorageBoundarySeam, fault: Option<StorageBoundaryFault>) -> Self {
        let execution_identity = StorageBoundaryExecutionIdentity::next();
        Self {
            execution_identity,
            seam,
            fault,
            trace: Arc::new(Mutex::new(StorageBoundaryTrace::for_execution(
                execution_identity,
            ))),
        }
    }

    pub fn trace(&self) -> StorageBoundaryTrace {
        self.trace
            .lock()
            .expect("storage boundary trace lock must not be poisoned")
            .clone()
    }
}

impl ProductionStorageBoundaryControl for ScriptedStorageBoundaryControl {
    fn fault_at(&self, seam: ProductionStorageBoundarySeam) -> Option<StorageBoundaryFault> {
        (seam == self.seam).then_some(self.fault).flatten()
    }

    fn record_reached(&self, seam: ProductionStorageBoundarySeam) {
        self.trace
            .lock()
            .expect("storage boundary trace lock must not be poisoned")
            .record_reached(seam);
    }

    fn record_injected(&self, seam: ProductionStorageBoundarySeam, fault: StorageBoundaryFault) {
        self.trace
            .lock()
            .expect("storage boundary trace lock must not be poisoned")
            .record_injected(seam, fault);
    }

    fn execution_identity(&self) -> Option<StorageBoundaryExecutionIdentity> {
        Some(self.execution_identity)
    }
}

impl StorageBoundaryExecutionIdentity {
    fn next() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_IDENTITY: AtomicU64 = AtomicU64::new(1);
        Self(NEXT_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }
}
