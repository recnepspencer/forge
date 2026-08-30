use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

use crate::state::SignalBranchId;

use super::branch_registry::SignalBranchCellConstruction;
use super::cell_incarnation::SignalBranchCellIncarnation;
use super::counters::SignalOwnerServiceCounters;
use super::lifecycle_state::{
    SignalOwnerAdmissionMismatch, SignalOwnerBranchCellHoldDenial, SignalOwnerLifecycleIdentity,
    SignalOwnerOperationAdmission,
};

const CELL_LIVE: u8 = 0;
const CELL_RETIRING: u8 = 1;
const CELL_RETIRED: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchCellAdmissionDenial {
    ForeignOwner,
    ExpiredLifecycle,
    SecondCellWhileHeld,
    RetirementInProgress,
    RetiredIncarnation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchCellPoisonRecovery {
    PreservedPartialMutation,
}

#[derive(Debug)]
pub(crate) struct SignalBranchExecutionCell<S> {
    state: Mutex<S>,
    owner_runtime_instance_id: u64,
    owner_lifecycle_identity: SignalOwnerLifecycleIdentity,
    branch_id: SignalBranchId,
    incarnation: SignalBranchCellIncarnation,
    lifecycle_posture: AtomicU8,
    counters: Arc<SignalOwnerServiceCounters>,
    recovered_poison: AtomicBool,
}

impl<S> SignalBranchExecutionCell<S> {
    pub(super) fn new(
        _construction: SignalBranchCellConstruction,
        state: S,
        owner_runtime_instance_id: u64,
        owner_lifecycle_identity: SignalOwnerLifecycleIdentity,
        branch_id: SignalBranchId,
        counters: Arc<SignalOwnerServiceCounters>,
    ) -> Self {
        Self {
            state: Mutex::new(state),
            owner_runtime_instance_id,
            owner_lifecycle_identity,
            branch_id,
            incarnation: SignalBranchCellIncarnation::issue(),
            lifecycle_posture: AtomicU8::new(CELL_LIVE),
            counters,
            recovered_poison: AtomicBool::new(false),
        }
    }

    pub(crate) fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub(crate) fn with_state<R>(
        &self,
        admission: &SignalOwnerOperationAdmission,
        operation: impl FnOnce(&mut S, &SignalBranchCellWork<'_>) -> R,
    ) -> Result<R, SignalBranchCellAdmissionDenial> {
        self.validate_admission(admission)?;
        let _cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)?;
        self.counters.record_target_cell_contact();
        let mut state = self.lock_state_after_contention_observation();
        self.require_live_posture()?;
        let work = SignalBranchCellWork {
            counters: &self.counters,
        };
        Ok(operation(&mut state, &work))
    }

    pub(super) fn begin_retirement(&self) -> Result<(), SignalBranchCellAdmissionDenial> {
        match self.lifecycle_posture.compare_exchange(
            CELL_LIVE,
            CELL_RETIRING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(()),
            Err(CELL_RETIRING) => Err(SignalBranchCellAdmissionDenial::RetirementInProgress),
            Err(CELL_RETIRED) => Err(SignalBranchCellAdmissionDenial::RetiredIncarnation),
            Err(_) => unreachable!("branch cell lifecycle posture is owner-defined"),
        }
    }

    pub(super) fn cancel_retirement(&self) {
        let result = self.lifecycle_posture.compare_exchange(
            CELL_RETIRING,
            CELL_LIVE,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        debug_assert!(result.is_ok(), "only a pending retirement may reopen");
    }

    pub(super) fn finish_retirement(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<(), SignalBranchCellAdmissionDenial> {
        self.validate_admission(admission)?;
        let _cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)?;
        self.counters.record_target_cell_contact();
        let _state = self.lock_state_after_contention_observation();
        match self.lifecycle_posture.compare_exchange(
            CELL_RETIRING,
            CELL_RETIRED,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(()),
            Err(CELL_LIVE) => Err(SignalBranchCellAdmissionDenial::RetirementInProgress),
            Err(CELL_RETIRED) => Err(SignalBranchCellAdmissionDenial::RetiredIncarnation),
            Err(_) => unreachable!("branch cell lifecycle posture is owner-defined"),
        }
    }

    pub(crate) fn poison_recovery(&self) -> Option<SignalBranchCellPoisonRecovery> {
        self.recovered_poison
            .load(Ordering::Acquire)
            .then_some(SignalBranchCellPoisonRecovery::PreservedPartialMutation)
    }

    fn validate_admission(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<(), SignalBranchCellAdmissionDenial> {
        admission
            .authorize(
                self.owner_runtime_instance_id,
                self.owner_lifecycle_identity,
            )
            .map_err(SignalBranchCellAdmissionDenial::from)
    }

    fn lock_state_after_contention_observation(&self) -> MutexGuard<'_, S> {
        match self.state.try_lock() {
            Ok(state) => state,
            Err(TryLockError::WouldBlock) => {
                self.counters.record_target_cell_wait();
                match self.state.lock() {
                    Ok(state) => state,
                    Err(poisoned) => self.recover_poisoned_state(poisoned),
                }
            }
            Err(TryLockError::Poisoned(poisoned)) => self.recover_poisoned_state(poisoned),
        }
    }

    fn recover_poisoned_state<'a>(
        &self,
        poisoned: std::sync::PoisonError<MutexGuard<'a, S>>,
    ) -> MutexGuard<'a, S> {
        self.recovered_poison.store(true, Ordering::Release);
        poisoned.into_inner()
    }

    fn require_live_posture(&self) -> Result<(), SignalBranchCellAdmissionDenial> {
        match self.lifecycle_posture.load(Ordering::Acquire) {
            CELL_LIVE => Ok(()),
            CELL_RETIRING => Err(SignalBranchCellAdmissionDenial::RetirementInProgress),
            CELL_RETIRED => Err(SignalBranchCellAdmissionDenial::RetiredIncarnation),
            _ => unreachable!("branch cell lifecycle posture is owner-defined"),
        }
    }
}

impl From<SignalOwnerAdmissionMismatch> for SignalBranchCellAdmissionDenial {
    fn from(mismatch: SignalOwnerAdmissionMismatch) -> Self {
        match mismatch {
            SignalOwnerAdmissionMismatch::ForeignOwner => Self::ForeignOwner,
            SignalOwnerAdmissionMismatch::ExpiredLifecycle => Self::ExpiredLifecycle,
        }
    }
}

impl From<SignalOwnerBranchCellHoldDenial> for SignalBranchCellAdmissionDenial {
    fn from(denial: SignalOwnerBranchCellHoldDenial) -> Self {
        match denial {
            SignalOwnerBranchCellHoldDenial::SecondCellWhileHeld => Self::SecondCellWhileHeld,
        }
    }
}

pub(crate) struct SignalBranchCellWork<'a> {
    counters: &'a SignalOwnerServiceCounters,
}

impl SignalBranchCellWork<'_> {
    pub(crate) fn record_canonical_movement(&self) {
        self.counters.record_canonical_movement();
    }

    pub(crate) fn record_retention_registry_contact(&self) {
        self.counters.record_retention_registry_contact();
    }

    pub(crate) fn record_fork_source_capture(&self) {
        self.counters.record_fork_source_capture();
    }

    pub(crate) fn record_forked_mutable_graph_node_copy(&self) {
        self.counters.record_forked_mutable_graph_node_copy();
    }

    pub(crate) fn record_diagnostic_event(&self) {
        self.counters.record_diagnostic_event();
    }

    pub(crate) fn record_dropped_diagnostic_event(&self) {
        self.counters.record_dropped_diagnostic_event();
    }
}
