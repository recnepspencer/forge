use std::sync::{
    atomic::{AtomicU8, Ordering},
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
    state: Arc<AtomicU8>,
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
            state: Arc::new(AtomicU8::new(DISARMED)),
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn arm(&self) -> Result<(), MediaFaultActivationDenial> {
        self.state
            .compare_exchange(DISARMED, ARMED, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|state| match state {
                ARMED => MediaFaultActivationDenial::AlreadyArmed,
                _ => MediaFaultActivationDenial::AlreadyConsumed,
            })
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn is_consumed(&self) -> bool {
        self.state.load(Ordering::Acquire) == CONSUMED
    }

    pub(super) fn consume_if_armed(&self) -> bool {
        self.state
            .compare_exchange(ARMED, CONSUMED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(super) fn same_activation(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }
}
