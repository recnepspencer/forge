use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct PlatformPulseExecutorGate {
    state: Arc<PlatformPulseExecutorGateState>,
}

struct PlatformPulseExecutorGateState {
    revision: AtomicU64,
    held: AtomicBool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseExecutorGateRevisionDenial {
    active: u64,
    submitted: u64,
}

impl PlatformPulseExecutorGate {
    pub fn held(revision: u64) -> Self {
        Self::at(revision, true)
    }

    pub fn at(revision: u64, held: bool) -> Self {
        Self {
            state: Arc::new(PlatformPulseExecutorGateState {
                revision: AtomicU64::new(revision),
                held: AtomicBool::new(held),
            }),
        }
    }

    pub fn apply(
        &self,
        revision: u64,
        held: bool,
    ) -> Result<(), PlatformPulseExecutorGateRevisionDenial> {
        let active = self.state.revision.load(Ordering::Acquire);
        if revision <= active {
            return Err(PlatformPulseExecutorGateRevisionDenial {
                active,
                submitted: revision,
            });
        }
        self.state.held.store(held, Ordering::Release);
        self.state.revision.store(revision, Ordering::Release);
        Ok(())
    }

    pub fn is_held(&self) -> bool {
        self.state.held.load(Ordering::Acquire)
    }

    pub fn revision(&self) -> u64 {
        self.state.revision.load(Ordering::Acquire)
    }
}

impl PlatformPulseExecutorGateRevisionDenial {
    pub const fn active(self) -> u64 {
        self.active
    }

    pub const fn submitted(self) -> u64 {
        self.submitted
    }
}
