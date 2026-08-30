use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use super::cell_incarnation::SignalBranchCellIncarnation;
use super::counters::SignalOwnerServiceCounters;
use super::SignalOwnerLifecycleObservation;

static NEXT_SIGNAL_OWNER_LIFECYCLE_IDENTITY: AtomicU64 = AtomicU64::new(1);
const OWNER_METADATA_HOLD: u64 = u64::MAX;

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
pub(crate) enum SignalOwnerLifecyclePoisonRecovery {
    PreservedLifecycleStatus,
}

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
    recovered_poison: AtomicBool,
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
            recovered_poison: AtomicBool::new(false),
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
            held_branch_cell_incarnation: AtomicU64::new(0),
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
            status = match self.drain.wait(status) {
                Ok(status) => status,
                Err(poisoned) => self.recover_poisoned_status(poisoned),
            };
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

    pub(crate) fn poison_recovery(&self) -> Option<SignalOwnerLifecyclePoisonRecovery> {
        self.recovered_poison
            .load(Ordering::Acquire)
            .then_some(SignalOwnerLifecyclePoisonRecovery::PreservedLifecycleStatus)
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
        match self.status.lock() {
            Ok(status) => status,
            Err(poisoned) => self.recover_poisoned_status(poisoned),
        }
    }

    fn recover_poisoned_status<'a>(
        &self,
        poisoned: std::sync::PoisonError<MutexGuard<'a, SignalOwnerLifecycleStatus>>,
    ) -> MutexGuard<'a, SignalOwnerLifecycleStatus> {
        self.recovered_poison.store(true, Ordering::Release);
        poisoned.into_inner()
    }

    #[cfg(test)]
    pub(super) fn poison_status_for_test(&self) {
        let _status = self.lock_status();
        panic!("inject lifecycle-status poison");
    }
}

#[derive(Debug)]
pub(crate) struct SignalOwnerOperationAdmission {
    lifecycle: Arc<SignalOwnerLifecycleState>,
    owner_runtime_instance_id: u64,
    lifecycle_identity: SignalOwnerLifecycleIdentity,
    held_branch_cell_incarnation: AtomicU64,
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

    pub(super) fn hold_branch_cell(
        &self,
        incarnation: SignalBranchCellIncarnation,
    ) -> Result<SignalOwnerBranchCellHold<'_>, SignalOwnerBranchCellHoldDenial> {
        self.held_branch_cell_incarnation
            .compare_exchange(0, incarnation.get(), Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| SignalOwnerBranchCellHoldDenial::SecondCellWhileHeld)?;
        Ok(SignalOwnerBranchCellHold {
            held_incarnation: &self.held_branch_cell_incarnation,
            incarnation,
        })
    }

    pub(super) fn hold_owner_metadata(
        &self,
    ) -> Result<SignalOwnerMetadataHold<'_>, SignalOwnerMetadataHoldDenial> {
        self.held_branch_cell_incarnation
            .compare_exchange(0, OWNER_METADATA_HOLD, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| SignalOwnerMetadataHoldDenial::BranchCellOrMetadataAlreadyHeld)?;
        Ok(SignalOwnerMetadataHold {
            held_posture: &self.held_branch_cell_incarnation,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SignalOwnerBranchCellHoldDenial {
    SecondCellWhileHeld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SignalOwnerMetadataHoldDenial {
    BranchCellOrMetadataAlreadyHeld,
}

pub(super) struct SignalOwnerBranchCellHold<'a> {
    held_incarnation: &'a AtomicU64,
    incarnation: SignalBranchCellIncarnation,
}

impl Drop for SignalOwnerBranchCellHold<'_> {
    fn drop(&mut self) {
        let released = self.held_incarnation.swap(0, Ordering::Release);
        debug_assert_eq!(released, self.incarnation.get());
    }
}

pub(super) struct SignalOwnerMetadataHold<'a> {
    held_posture: &'a AtomicU64,
}

impl Drop for SignalOwnerMetadataHold<'_> {
    fn drop(&mut self) {
        let released = self.held_posture.swap(0, Ordering::Release);
        debug_assert_eq!(released, OWNER_METADATA_HOLD);
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
