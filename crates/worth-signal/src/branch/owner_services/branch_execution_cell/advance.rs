use crate::branch::{
    AdmittedSignalBranchBasis, SignalBranchAdvanceDenial, SignalBranchObservation,
};
use crate::data::error::SignalError;
use crate::logic::transaction::{SignalTransaction, TransactionResult};

use super::super::{
    SignalBranchCellState, SignalOwnerCancellationToken, SignalOwnerOperationAdmission,
    SignalOwnerUnavailable,
};
use super::{SignalBranchCellAdmissionDenial, SignalBranchCellWork, SignalBranchExecutionCell};

pub(crate) struct SignalBranchAdvanceCellOutcome {
    observation: SignalBranchObservation,
    transaction: TransactionResult,
}

impl SignalBranchAdvanceCellOutcome {
    pub(crate) fn into_parts(self) -> (SignalBranchObservation, TransactionResult) {
        (self.observation, self.transaction)
    }
}

impl<D, I, T> SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Exact one-cell mutation seam consumed by the Phase 4 mutation port.
    /// Cancellation and complete-basis comparison occur before the canonical
    /// transaction callback can mutate branch truth.
    pub(crate) fn advance_exact<E, Ctx, F>(
        &self,
        admission: &SignalOwnerOperationAdmission,
        expected: &AdmittedSignalBranchBasis,
        runtime_ctx: &mut Ctx,
        cancellation: &SignalOwnerCancellationToken,
        apply: F,
    ) -> Result<SignalBranchAdvanceCellOutcome, SignalBranchAdvanceDenial>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        self.validate_admission(admission)
            .map_err(|denial| map_advance_cell_denial(denial, self.branch_id))?;
        cancellation
            .preflight_cell_wait()
            .map_err(|_| SignalBranchAdvanceDenial::CancelledNoMovement)?;
        let _cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)
            .map_err(|denial| map_advance_cell_denial(denial, self.branch_id))?;
        self.counters.record_target_cell_contact();
        self.contacts
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut state = self
            .lock_state_after_contention_observation()
            .map_err(|denial| map_advance_cell_denial(denial, self.branch_id))?;
        self.require_live_posture()
            .map_err(|denial| map_advance_cell_denial(denial, self.branch_id))?;

        let live = state
            .observation()
            .map_err(|error| SignalBranchAdvanceDenial::MutationFailedNoMovement { error })?;
        if let Err(mismatch) = live.compare(expected.observation()) {
            return Err(SignalBranchAdvanceDenial::BasisMismatch {
                axes: mismatch.axes().to_vec(),
            });
        }
        let observation = state
            .next_advance_observation()
            .map_err(|error| SignalBranchAdvanceDenial::MutationFailedNoMovement { error })?;
        let permit = cancellation
            .preflight_movement()
            .map_err(|_| SignalBranchAdvanceDenial::CancelledNoMovement)?;
        let transaction = state
            .execute_canonical_transaction(&permit, runtime_ctx, apply)
            .map_err(|error| SignalBranchAdvanceDenial::MutationFailedNoMovement { error })?;
        SignalBranchCellWork {
            counters: &self.counters,
            movements: &self.movements,
        }
        .record_canonical_movement(&permit);
        Ok(SignalBranchAdvanceCellOutcome {
            observation,
            transaction,
        })
    }
}

pub(in crate::branch::owner_services) fn map_advance_cell_denial(
    denial: SignalBranchCellAdmissionDenial,
    branch_id: crate::state::SignalBranchId,
) -> SignalBranchAdvanceDenial {
    match denial {
        SignalBranchCellAdmissionDenial::ForeignOwner
        | SignalBranchCellAdmissionDenial::ExpiredLifecycle => {
            SignalBranchAdvanceDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        SignalBranchCellAdmissionDenial::SecondCellWhileHeld => {
            SignalBranchAdvanceDenial::OwnerCellMisuse { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetirementInProgress => {
            SignalBranchAdvanceDenial::RetirementInProgress { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetiredIncarnation => {
            SignalBranchAdvanceDenial::RetiredBranch { branch_id }
        }
        SignalBranchCellAdmissionDenial::PoisonedIncarnation => {
            SignalBranchAdvanceDenial::QuarantinedBranch { branch_id }
        }
    }
}
