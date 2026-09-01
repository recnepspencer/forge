use std::marker::PhantomData;
use std::sync::{Arc, Weak};

use crate::branch::{
    AdmittedSignalBranchBasis, AdmittedSignalBranchSnapshot, SignalBranchAdvanceDenial,
    SignalBranchAdvanceOutcome, SignalBranchForkOperationDenial, SignalBranchForkOutcome,
    SignalBranchRestoreDenial, SignalBranchSnapshotCaptureDenial,
    SignalBranchSnapshotCaptureOutcome, ValidatedSignalBranchName,
};
use crate::data::error::SignalError;
use crate::logic::transaction::SignalTransaction;

use super::{SignalOwner, SignalOwnerCancellationToken, SignalOwnerUnavailable};

#[path = "mutation_port/denials.rs"]
mod denials;

#[cfg(test)]
mod tests;

use denials::{
    map_advance_admission_denial, map_advance_registry_denial, map_capture_admission_denial,
    map_capture_registry_denial, map_fork_admission_denial, map_fork_registry_denial,
    map_restore_admission_denial, map_restore_registry_denial,
};

/// Public concrete weak mutation service issued by a sealed owner root.
pub struct SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    owner: Weak<SignalOwner<D, I, T>>,
    diagnostic_owner_runtime_instance_id: u64,
    type_contract: PhantomData<fn(E, Ctx)>,
}

impl<D, I, E, Ctx, T> Clone for SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            diagnostic_owner_runtime_instance_id: self.diagnostic_owner_runtime_instance_id,
            type_contract: PhantomData,
        }
    }
}

impl<D, I, E, Ctx, T> SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        owner: Weak<SignalOwner<D, I, T>>,
        diagnostic_owner_runtime_instance_id: u64,
    ) -> Self {
        Self {
            owner,
            diagnostic_owner_runtime_instance_id,
            type_contract: PhantomData,
        }
    }

    pub(crate) fn diagnostic_owner_runtime_instance_id(&self) -> u64 {
        self.diagnostic_owner_runtime_instance_id
    }

    pub fn fork_exact(
        &self,
        requested_identity: ValidatedSignalBranchName,
        source: &AdmittedSignalBranchBasis,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalBranchForkOutcome, SignalBranchForkOperationDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchForkOperationDenial::OwnerUnavailable)?;
        let admission = owner.admit().map_err(map_fork_admission_denial)?;
        let source_branch_id = source.owner_branch_id();
        let source_cell = owner
            .lookup_cell(&admission, source_branch_id)
            .map_err(|denial| map_fork_registry_denial(denial, source_branch_id))?;
        let output = owner.reserve_fork_output(&admission, &source_cell)?;
        let ready = output.fork(source, requested_identity, cancellation)?;
        let (created_branch, created_basis) = ready.into_destination_parts();
        Ok(SignalBranchForkOutcome::owner_issued(
            created_branch,
            created_basis,
        ))
    }

    pub fn advance_exact<F>(
        &self,
        expected: &AdmittedSignalBranchBasis,
        runtime_ctx: &mut Ctx,
        cancellation: &SignalOwnerCancellationToken,
        apply: F,
    ) -> Result<SignalBranchAdvanceOutcome, SignalBranchAdvanceDenial>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchAdvanceDenial::OwnerUnavailable)?;
        let admission = owner.admit().map_err(map_advance_admission_denial)?;
        let branch_id = expected.owner_branch_id();
        let cell = owner
            .lookup_cell(&admission, branch_id)
            .map_err(|denial| map_advance_registry_denial(denial, branch_id))?;
        let output = owner.reserve_advance_output(&admission, &cell)?;
        let ready = output.advance(expected, runtime_ctx, cancellation, apply)?;
        let (advanced_basis, transaction) = ready.into_parts();
        Ok(SignalBranchAdvanceOutcome::owner_issued(
            advanced_basis,
            transaction,
        ))
    }

    pub fn capture_exact(
        &self,
        expected: &AdmittedSignalBranchBasis,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalBranchSnapshotCaptureOutcome, SignalBranchSnapshotCaptureDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchSnapshotCaptureDenial::OwnerUnavailable)?;
        let admission = owner.admit().map_err(map_capture_admission_denial)?;
        let branch_id = expected.owner_branch_id();
        let cell = owner
            .lookup_cell(&admission, branch_id)
            .map_err(|denial| map_capture_registry_denial(denial, branch_id))?;
        owner
            .reserve_snapshot_outputs(&admission, &cell)?
            .capture(expected, cancellation)
            .map(|ready| ready.into_outcome())
    }

    pub fn restore_exact(
        &self,
        expected: &AdmittedSignalBranchBasis,
        snapshot: &AdmittedSignalBranchSnapshot,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<AdmittedSignalBranchBasis, SignalBranchRestoreDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchRestoreDenial::OwnerUnavailable)?;
        let admission = owner.admit().map_err(map_restore_admission_denial)?;
        let branch_id = expected.owner_branch_id();
        let cell = owner
            .lookup_cell(&admission, branch_id)
            .map_err(|denial| map_restore_registry_denial(denial, branch_id))?;
        owner
            .reserve_restore_output(&admission, &cell)?
            .restore(expected, snapshot, cancellation)
            .map(|ready| ready.into_basis())
    }

    pub(super) fn upgrade_owner(
        &self,
    ) -> Result<Arc<SignalOwner<D, I, T>>, SignalOwnerUnavailable> {
        SignalOwner::upgrade(&self.owner)
    }
}
