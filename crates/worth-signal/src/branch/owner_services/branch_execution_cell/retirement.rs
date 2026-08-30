use std::sync::atomic::Ordering;

use crate::branch::{
    PlannedSignalBranchRetirement, SignalBranchRetirementDenial, SignalBranchRetirementReason,
};
use crate::state::SignalBranchHandle;

use super::super::{
    SignalBranchCellState, SignalOwnerCancellationToken, SignalOwnerOperationAdmission,
    SignalOwnerUnavailable,
};
use super::{
    SignalBranchCellAdmissionDenial, SignalBranchCellRetirementPosture, SignalBranchCellWork,
    SignalBranchExecutionCell, CELL_LIVE, CELL_RETIRED, CELL_RETIRING,
};

pub(crate) struct SignalBranchRetirementCellOutcome {
    pub(crate) retired_branch: SignalBranchHandle,
    pub(crate) reason: SignalBranchRetirementReason,
    pub(crate) terminal_basis_digest: String,
}

impl<D, I, T> SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Consumes one owner-issued plan under its sole target-cell incarnation.
    pub(crate) fn retire_exact(
        &self,
        admission: &SignalOwnerOperationAdmission,
        plan: PlannedSignalBranchRetirement,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalBranchRetirementCellOutcome, SignalBranchRetirementDenial> {
        cancellation
            .preflight_cell_wait()
            .map_err(|_| SignalBranchRetirementDenial::CancelledNoMovement)?;
        self.validate_admission(admission)
            .map_err(|denial| map_retirement_cell_denial(denial, self.branch_id))?;
        let _cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)
            .map_err(|denial| map_retirement_cell_denial(denial, self.branch_id))?;
        self.counters.record_target_cell_contact();
        self.contacts.fetch_add(1, Ordering::SeqCst);
        let state = self
            .lock_state_after_contention_observation()
            .map_err(|denial| map_retirement_cell_denial(denial, self.branch_id))?;
        self.lifecycle_posture
            .compare_exchange(
                CELL_LIVE,
                CELL_RETIRING,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map_err(|_| SignalBranchRetirementDenial::UnknownBranch {
                branch_id: self.branch_id,
            })?;
        let mut posture = SignalBranchCellRetirementPosture {
            lifecycle_posture: &self.lifecycle_posture,
            retired: false,
        };
        if plan.branch().id != state.branch_id()
            || state.observation().map_or(true, |live| {
                live.compare(plan.admitted_basis().observation()).is_err()
            })
        {
            return Err(SignalBranchRetirementDenial::CanonicalBasisMismatch);
        }
        let permit = cancellation
            .preflight_movement()
            .map_err(|_| SignalBranchRetirementDenial::CancelledNoMovement)?;
        let retired_branch = state.handle().clone();
        SignalBranchCellWork {
            counters: &self.counters,
            movements: &self.movements,
        }
        .record_canonical_movement(&permit);
        self.lifecycle_posture
            .store(CELL_RETIRED, Ordering::Release);
        posture.retired = true;
        Ok(SignalBranchRetirementCellOutcome {
            retired_branch,
            reason: plan.reason(),
            terminal_basis_digest: plan.terminal_basis_digest,
        })
    }
}

fn map_retirement_cell_denial(
    denial: SignalBranchCellAdmissionDenial,
    branch_id: crate::state::SignalBranchId,
) -> SignalBranchRetirementDenial {
    match denial {
        SignalBranchCellAdmissionDenial::ForeignOwner
        | SignalBranchCellAdmissionDenial::ExpiredLifecycle => {
            SignalBranchRetirementDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        _ => SignalBranchRetirementDenial::UnknownBranch { branch_id },
    }
}
