use std::sync::atomic::Ordering;

use crate::branch::{AdmittedSignalBranchBasis, SignalBranchForkOperationDenial};
use crate::data::error::SignalError;

use super::super::mutation_port::{SignalOwnerForkCellBuilder, SignalPreparedOwnerFork};
use super::super::{
    SignalBranchCellState, SignalOwnerCancellationToken, SignalOwnerOperationAdmission,
    SignalOwnerUnavailable,
};
use super::{SignalBranchCellAdmissionDenial, SignalBranchCellWork, SignalBranchExecutionCell};

impl<D, I, T> SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Captures and linearizes exactly one source cell for an owner reservation.
    pub(crate) fn capture_fork_source_exact<'a>(
        &self,
        admission: &SignalOwnerOperationAdmission,
        source: &AdmittedSignalBranchBasis,
        mut builder: SignalOwnerForkCellBuilder<'a, D, I, T>,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalPreparedOwnerFork<'a, D, I, T>, SignalBranchForkOperationDenial> {
        cancellation
            .preflight_cell_wait()
            .map_err(|_| SignalBranchForkOperationDenial::CancelledNoMovement)?;
        self.validate_admission(admission)
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        let cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        self.counters.record_target_cell_contact();
        self.contacts.fetch_add(1, Ordering::SeqCst);
        let mut state = self
            .lock_state_after_contention_observation()
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        self.require_live_posture()
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        let live = state.observation().map_err(owner_fork_failure)?;
        if let Err(mismatch) = live.compare(source.observation()) {
            return Err(SignalBranchForkOperationDenial::BasisMismatch {
                axes: mismatch.axes().to_vec(),
            });
        }
        let captured = state.fork_state(builder.destination());
        let prepared_cell = match builder.prepare_cell(captured.state) {
            Ok(prepared) => prepared,
            Err(denial) => {
                drop(state);
                drop(cell_hold);
                drop(builder);
                return Err(denial);
            }
        };
        if let Err(denial) = builder.validate_prepared_cell(&prepared_cell) {
            drop(state);
            drop(cell_hold);
            drop(prepared_cell);
            drop(builder);
            return Err(denial);
        }
        let _movement = match cancellation.preflight_movement() {
            Ok(movement) => movement,
            Err(_) => {
                drop(state);
                drop(cell_hold);
                drop(prepared_cell);
                drop(builder);
                return Err(SignalBranchForkOperationDenial::CancelledNoMovement);
            }
        };
        state.commit_fork_source_boundary();
        SignalBranchCellWork {
            counters: &self.counters,
            movements: &self.movements,
        }
        .record_fork_source_capture(captured.work);
        drop(state);
        drop(cell_hold);
        Ok(builder.bind_prepared_cell(prepared_cell))
    }
}

pub(in crate::branch::owner_services) fn map_fork_cell_denial(
    denial: SignalBranchCellAdmissionDenial,
    branch_id: crate::state::SignalBranchId,
) -> SignalBranchForkOperationDenial {
    match denial {
        SignalBranchCellAdmissionDenial::ForeignOwner
        | SignalBranchCellAdmissionDenial::ExpiredLifecycle => {
            SignalBranchForkOperationDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        SignalBranchCellAdmissionDenial::SecondCellWhileHeld => {
            SignalBranchForkOperationDenial::OwnerCellMisuse { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetirementInProgress => {
            SignalBranchForkOperationDenial::RetirementInProgress { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetiredIncarnation => {
            SignalBranchForkOperationDenial::RetiredBranch { branch_id }
        }
        SignalBranchCellAdmissionDenial::PoisonedIncarnation => {
            SignalBranchForkOperationDenial::QuarantinedBranch { branch_id }
        }
    }
}

fn owner_fork_failure(error: SignalError) -> SignalBranchForkOperationDenial {
    SignalBranchForkOperationDenial::OwnerDeniedNoMovement { error }
}
