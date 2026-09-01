use std::sync::atomic::Ordering;

use crate::branch::{AdmittedSignalBranchBasis, SignalBranchForkOperationDenial};
use crate::data::error::SignalError;

use super::super::owner::fork_reservation::{SignalOwnerForkCellBuilder, SignalPreparedOwnerFork};
use super::super::{SignalBranchCellState, SignalOwnerCancellationToken, SignalOwnerUnavailable};
use super::SignalBranchForkSourceCustody;
use super::{SignalBranchCellAdmissionDenial, SignalBranchCellWork, SignalBranchExecutionCell};
use crate::branch::owner_services::operation_control::SignalOwnerOperationBoundary;

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SignalForkSourceStateTruth {
    pub(crate) handle: crate::state::SignalBranchHandle,
    pub(crate) graph: serde_json::Value,
    pub(crate) mutation_ledger: crate::logic::transaction::BranchMutationLedger,
    pub(crate) generation: u64,
    pub(crate) restore_snapshot_id: Option<crate::state::SignalSnapshotId>,
    pub(crate) observation: crate::branch::SignalBranchObservation,
}

impl<D, I, T> SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    #[cfg(test)]
    pub(crate) fn fork_source_state_truth_after_fault(&self) -> SignalForkSourceStateTruth {
        let state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        SignalForkSourceStateTruth {
            handle: state.handle().clone(),
            graph: serde_json::to_value(state.state().graph())
                .expect("fork source graph serializes for exact fault observation"),
            mutation_ledger: state.state().mutation_ledger().clone(),
            generation: state.head_generation(),
            restore_snapshot_id: state.restore_snapshot_id(),
            observation: state
                .observation()
                .expect("fork source truth remains internally complete"),
        }
    }

    /// Captures and linearizes exactly one source cell for an owner reservation.
    pub(in crate::branch::owner_services) fn capture_fork_source_exact<'a>(
        &self,
        source_custody: &SignalBranchForkSourceCustody<'_, '_, SignalBranchCellState<D, I, T>>,
        source: &AdmittedSignalBranchBasis,
        mut builder: SignalOwnerForkCellBuilder<'a, D, I, T>,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalPreparedOwnerFork<'a, D, I, T>, SignalBranchForkOperationDenial> {
        assert!(
            source_custody.matches(self),
            "fork capture requires custody of its exact source cell"
        );
        let admission = source_custody.admission();
        cancellation
            .preflight_cell_wait()
            .map_err(|_| SignalBranchForkOperationDenial::CancelledNoMovement)?;
        self.validate_admission(admission)
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        let cell_hold = admission
            .hold_branch_cell()
            .map_err(SignalBranchCellAdmissionDenial::from)
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        self.counters.record_target_cell_contact();
        self.contacts.fetch_add(1, Ordering::SeqCst);
        let mut state = self
            .lock_state_without_fork_custody()
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        self.require_live_posture()
            .map_err(|denial| map_fork_cell_denial(denial, self.branch_id))?;
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::ExactBasisPreflight);
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
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::BeforeCanonicalMovement);
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
        SignalBranchCellWork {
            counters: &self.counters,
            movements: &self.movements,
        }
        .record_fork_source_capture(captured.work);
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::AfterCanonicalMovement);
        admission.reach_operation_boundary(SignalOwnerOperationBoundary::ForkSourceCapture);
        drop(state);
        drop(cell_hold);
        Ok(builder.bind_prepared_cell(prepared_cell))
    }

    pub(in crate::branch::owner_services) fn commit_fork_source_boundary(
        &self,
        source_custody: &SignalBranchForkSourceCustody<'_, '_, SignalBranchCellState<D, I, T>>,
    ) {
        assert!(
            source_custody.matches(self),
            "fork source commit requires exact branch custody"
        );
        self.validate_admission(source_custody.admission())
            .expect("fork source custody retains its exact owner admission through commit");
        let mut state = self
            .lock_state_without_fork_custody()
            .expect("captured fork source remains healthy through infallible handoff");
        state.commit_fork_source_boundary();
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
        SignalBranchCellAdmissionDenial::ExecutingThreadReentry => {
            SignalBranchForkOperationDenial::OwnerReentry
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
