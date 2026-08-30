use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use super::counters::SignalOwnerServiceCounters;
use super::SignalOwnerLifecycleObservation;

static NEXT_SIGNAL_OWNER_LIFECYCLE_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalOwnerAdmissionDenial {
    ForeignOwner,
    OwnerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalOwnerAdmissionMismatch {
    ForeignOwner,
    ExpiredLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SignalOwnerCloseDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SignalOwnerLifecycleIdentity(u64);

#[derive(Debug)]
struct SignalOwnerLifecycleStatus {
    observation: SignalOwnerLifecycleObservation,
    admitted_operations: usize,
}

#[derive(Debug)]
pub(crate) struct SignalOwnerLifecycleState {
    owner_runtime_instance_id: u64,
    lifecycle_identity: SignalOwnerLifecycleIdentity,
    status: Mutex<SignalOwnerLifecycleStatus>,
    drain: Condvar,
    counters: Arc<SignalOwnerServiceCounters>,
}

impl SignalOwnerLifecycleState {
    pub(crate) fn new(
        owner_runtime_instance_id: u64,
        counters: Arc<SignalOwnerServiceCounters>,
    ) -> Arc<Self> {
        Arc::new(Self {
            owner_runtime_instance_id,
            lifecycle_identity: next_lifecycle_identity(),
            status: Mutex::new(SignalOwnerLifecycleStatus {
                observation: SignalOwnerLifecycleObservation::Open,
                admitted_operations: 0,
            }),
            drain: Condvar::new(),
            counters,
        })
    }

    pub(crate) fn admit(
        self: &Arc<Self>,
        owner_runtime_instance_id: u64,
    ) -> Result<SignalOwnerOperationAdmission, SignalOwnerAdmissionDenial> {
        if owner_runtime_instance_id != self.owner_runtime_instance_id {
            return Err(SignalOwnerAdmissionDenial::ForeignOwner);
        }
        let mut status = self.lock_status();
        if status.observation != SignalOwnerLifecycleObservation::Open {
            return Err(SignalOwnerAdmissionDenial::OwnerUnavailable);
        }
        status.admitted_operations += 1;
        Ok(SignalOwnerOperationAdmission {
            lifecycle: Arc::clone(self),
            owner_runtime_instance_id,
            lifecycle_identity: self.lifecycle_identity,
        })
    }

    pub(crate) fn close(
        &self,
        owner_runtime_instance_id: u64,
    ) -> Result<(), SignalOwnerCloseDenial> {
        if owner_runtime_instance_id != self.owner_runtime_instance_id {
            return Err(SignalOwnerCloseDenial);
        }
        let mut status = self.lock_status();
        if status.observation == SignalOwnerLifecycleObservation::Open {
            status.observation = SignalOwnerLifecycleObservation::Closing;
            self.counters.record_close_batch();
        }
        while status.observation == SignalOwnerLifecycleObservation::Closing {
            if status.admitted_operations == 0 {
                status.observation = SignalOwnerLifecycleObservation::Closed;
                self.drain.notify_all();
                break;
            }
            status = self
                .drain
                .wait(status)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
        Ok(())
    }

    pub(crate) fn observation(&self) -> SignalOwnerLifecycleObservation {
        self.lock_status().observation
    }

    pub(crate) fn owner_runtime_instance_id(&self) -> u64 {
        self.owner_runtime_instance_id
    }

    pub(crate) fn lifecycle_identity(&self) -> SignalOwnerLifecycleIdentity {
        self.lifecycle_identity
    }

    pub(crate) fn counters(&self) -> Arc<SignalOwnerServiceCounters> {
        Arc::clone(&self.counters)
    }

    fn release_operation(&self) {
        let mut status = self.lock_status();
        debug_assert!(status.admitted_operations > 0);
        status.admitted_operations = status.admitted_operations.saturating_sub(1);
        if status.observation == SignalOwnerLifecycleObservation::Closing
            && status.admitted_operations == 0
        {
            self.drain.notify_all();
        }
    }

    fn lock_status(&self) -> MutexGuard<'_, SignalOwnerLifecycleStatus> {
        self.status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

#[derive(Debug)]
pub(crate) struct SignalOwnerOperationAdmission {
    lifecycle: Arc<SignalOwnerLifecycleState>,
    owner_runtime_instance_id: u64,
    lifecycle_identity: SignalOwnerLifecycleIdentity,
}

impl SignalOwnerOperationAdmission {
    pub(super) fn authorize(
        &self,
        owner_runtime_instance_id: u64,
        lifecycle_identity: SignalOwnerLifecycleIdentity,
    ) -> Result<(), SignalOwnerAdmissionMismatch> {
        if self.owner_runtime_instance_id != owner_runtime_instance_id {
            return Err(SignalOwnerAdmissionMismatch::ForeignOwner);
        }
        if self.lifecycle_identity != lifecycle_identity {
            return Err(SignalOwnerAdmissionMismatch::ExpiredLifecycle);
        }
        Ok(())
    }
}

impl Drop for SignalOwnerOperationAdmission {
    fn drop(&mut self) {
        self.lifecycle.release_operation();
    }
}

fn next_lifecycle_identity() -> SignalOwnerLifecycleIdentity {
    let identity = NEXT_SIGNAL_OWNER_LIFECYCLE_IDENTITY
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .expect("Signal owner lifecycle identity exhausted");
    SignalOwnerLifecycleIdentity(identity)
}
