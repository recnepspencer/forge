use std::num::NonZeroU64;
use std::sync::{
    atomic::{AtomicU64, AtomicU8, Ordering},
    Arc,
};

#[cfg(any(test, feature = "certification-test-authority"))]
const DISARMED: u8 = 0;
const ARMED: u8 = 1;
const CONSUMED: u8 = 2;

/// One-shot certification authority for a fault whose operation identity is
/// allocated only after media admission.
#[derive(Debug, Clone)]
pub struct CertificationMediaFaultActivation {
    state: Arc<CertificationMediaFaultActivationState>,
}

#[derive(Debug)]
struct CertificationMediaFaultActivationState {
    disposition: AtomicU8,
    matching_operations: AtomicU64,
}

#[cfg(any(test, feature = "certification-test-authority"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFaultActivationDenial {
    AlreadyArmed,
    AlreadyConsumed,
}

impl CertificationMediaFaultActivation {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(super) fn for_certification() -> Self {
        Self {
            state: Arc::new(CertificationMediaFaultActivationState {
                disposition: AtomicU8::new(DISARMED),
                matching_operations: AtomicU64::new(0),
            }),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn arm(&self) -> Result<(), MediaFaultActivationDenial> {
        self.state
            .disposition
            .compare_exchange(DISARMED, ARMED, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|state| match state {
                ARMED => MediaFaultActivationDenial::AlreadyArmed,
                _ => MediaFaultActivationDenial::AlreadyConsumed,
            })
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn is_consumed(&self) -> bool {
        self.state.disposition.load(Ordering::Acquire) == CONSUMED
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn matching_operation_count(&self) -> u64 {
        self.state.matching_operations.load(Ordering::Acquire)
    }

    pub(super) fn consume_if_armed_on_match(&self, selected_match: NonZeroU64) -> bool {
        if self.state.disposition.load(Ordering::Acquire) != ARMED {
            return false;
        }
        let prior = self
            .state
            .matching_operations
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |observed| {
                Some(observed.saturating_add(1))
            })
            .expect("the matching-operation counter always accepts saturation");
        if prior.saturating_add(1) != selected_match.get() {
            return false;
        }
        self.state
            .disposition
            .compare_exchange(ARMED, CONSUMED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(super) fn same_activation(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }
}
