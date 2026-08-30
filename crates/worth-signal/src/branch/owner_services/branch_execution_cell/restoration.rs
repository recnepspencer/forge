use std::sync::atomic::Ordering;

use crate::branch::{
    AdmittedSignalBranchBasis, AdmittedSignalBranchSnapshot, SignalBranchObservation,
    SignalBranchRestoreDenial,
};
use crate::data::error::SignalError;

use super::super::owner_metadata::SignalOwnerSnapshotStateBinding;
use super::super::{
    SignalBranchCellState, SignalOwnerCancellationToken, SignalOwnerOperationAdmission,
    SignalOwnerUnavailable,
};
use super::{SignalBranchCellAdmissionDenial, SignalBranchCellWork, SignalBranchExecutionCell};

pub(crate) struct SignalBranchRestoreCellOutcome {
    pub(crate) observation: SignalBranchObservation,
}

impl<D, I, T> SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Restores one owner-admitted snapshot into its exact target cell.
    pub(crate) fn restore_exact(
        &self,
        admission: &SignalOwnerOperationAdmission,
        expected: &AdmittedSignalBranchBasis,
        admitted_snapshot: &AdmittedSignalBranchSnapshot,
        snapshot_state: SignalOwnerSnapshotStateBinding<D, I, T>,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalBranchRestoreCellOutcome, SignalBranchRestoreDenial> {
        if admitted_snapshot.owner_runtime_instance_id() != self.owner_runtime_instance_id {
            return Err(SignalBranchRestoreDenial::ForeignSnapshotOwner {
                expected_runtime_instance_id: self.owner_runtime_instance_id,
                observed_runtime_instance_id: admitted_snapshot.owner_runtime_instance_id(),
            });
        }
        let snapshot = admitted_snapshot.snapshot();
        if snapshot.meta.branch_id != self.branch_id {
            return Err(SignalBranchRestoreDenial::CrossBranchSnapshot {
                branch_id: self.branch_id,
                snapshot_branch_id: snapshot.meta.branch_id,
            });
        }
        let snapshot_state = snapshot_state.into_state_for(
            self.owner_runtime_instance_id,
            self.owner_lifecycle_identity,
            admitted_snapshot,
        )?;
        cancellation
            .preflight_cell_wait()
            .map_err(|_| SignalBranchRestoreDenial::CancelledNoMovement)?;
        self.validate_admission(admission)
            .map_err(|denial| map_restore_cell_denial(denial, self.branch_id))?;
        let _cell_hold = admission
            .hold_branch_cell(self.incarnation)
            .map_err(SignalBranchCellAdmissionDenial::from)
            .map_err(|denial| map_restore_cell_denial(denial, self.branch_id))?;
        self.counters.record_target_cell_contact();
        self.contacts.fetch_add(1, Ordering::SeqCst);
        let mut state = self
            .lock_state_after_contention_observation()
            .map_err(|denial| map_restore_cell_denial(denial, self.branch_id))?;
        self.require_live_posture()
            .map_err(|denial| map_restore_cell_denial(denial, self.branch_id))?;
        let live = state.observation().map_err(owner_restore_failure)?;
        if let Err(mismatch) = live.compare(expected.observation()) {
            return Err(SignalBranchRestoreDenial::BasisMismatch {
                axes: mismatch.axes().to_vec(),
            });
        }
        let prepared = state
            .prepare_restore(snapshot_state, snapshot)
            .map_err(owner_restore_failure)?;
        let observation = prepared.observation.clone();
        let permit = cancellation
            .preflight_movement()
            .map_err(|_| SignalBranchRestoreDenial::CancelledNoMovement)?;
        state.commit_restore(prepared);
        SignalBranchCellWork {
            counters: &self.counters,
            movements: &self.movements,
        }
        .record_canonical_movement(&permit);
        Ok(SignalBranchRestoreCellOutcome { observation })
    }
}

pub(in crate::branch::owner_services) fn map_restore_cell_denial(
    denial: SignalBranchCellAdmissionDenial,
    branch_id: crate::state::SignalBranchId,
) -> SignalBranchRestoreDenial {
    match denial {
        SignalBranchCellAdmissionDenial::ForeignOwner
        | SignalBranchCellAdmissionDenial::ExpiredLifecycle => {
            SignalBranchRestoreDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        SignalBranchCellAdmissionDenial::SecondCellWhileHeld => {
            SignalBranchRestoreDenial::OwnerCellMisuse { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetirementInProgress => {
            SignalBranchRestoreDenial::RetirementInProgress { branch_id }
        }
        SignalBranchCellAdmissionDenial::RetiredIncarnation => {
            SignalBranchRestoreDenial::RetiredBranch { branch_id }
        }
        SignalBranchCellAdmissionDenial::PoisonedIncarnation => {
            SignalBranchRestoreDenial::QuarantinedBranch { branch_id }
        }
    }
}

fn owner_restore_failure(error: SignalError) -> SignalBranchRestoreDenial {
    SignalBranchRestoreDenial::OwnerDeniedNoMovement { error }
}
