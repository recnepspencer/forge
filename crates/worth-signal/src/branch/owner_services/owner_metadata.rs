use std::sync::{Mutex, MutexGuard};

use crate::branch::{
    AdmittedSignalBranchSnapshot, SignalBranchRestoreDenial, SignalBranchRetirementReceipt,
    SignalBranchSnapshotCaptureDenial,
};
use crate::logic::transaction::{
    SignalOwnerMetadataState, SignalOwnerSnapshotReservationDenial, SnapshotBranchState,
    SnapshotStatePacket,
};
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::lifecycle_state::SignalOwnerLifecycleIdentity;
use super::{SignalOwnerOperationAdmission, SignalOwnerUnavailable};

/// Short-lived owner metadata; canonical live branch truth never enters this lock.
pub(super) struct SignalOwnerMetadata<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime_instance_id: u64,
    lifecycle_identity: SignalOwnerLifecycleIdentity,
    state: Mutex<SignalOwnerMetadataState<D, I, T>>,
}

impl<D, I, T> SignalOwnerMetadata<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn new(
        state: SignalOwnerMetadataState<D, I, T>,
        runtime_instance_id: u64,
        lifecycle_identity: SignalOwnerLifecycleIdentity,
    ) -> Self {
        Self {
            runtime_instance_id,
            lifecycle_identity,
            state: Mutex::new(state),
        }
    }

    pub(super) fn reserve_snapshot<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
    ) -> Result<SignalOwnerSnapshotReservation<'a, D, I, T>, SignalBranchSnapshotCaptureDenial>
    {
        let _hold = self
            .authorize(admission)
            .map_err(SignalBranchSnapshotCaptureDenial::OwnerUnavailable)?;
        let snapshot_id = self
            .lock()
            .reserve_snapshot()
            .map_err(|denial| match denial {
                SignalOwnerSnapshotReservationDenial::CapacityExhausted {
                    maximum_stored_snapshots,
                } => SignalBranchSnapshotCaptureDenial::SnapshotCapacityExhausted {
                    maximum_stored_snapshots,
                },
                SignalOwnerSnapshotReservationDenial::IdentityExhausted { next_snapshot_id } => {
                    SignalBranchSnapshotCaptureDenial::SnapshotIdentityExhausted {
                        next_snapshot_id,
                    }
                }
            })?;
        Ok(SignalOwnerSnapshotReservation {
            metadata: self,
            admission,
            snapshot_id,
            installed: false,
        })
    }

    pub(super) fn snapshot_state(
        &self,
        admission: &SignalOwnerOperationAdmission,
        admitted_snapshot: &AdmittedSignalBranchSnapshot,
    ) -> Result<Option<SignalOwnerSnapshotStateBinding<D, I, T>>, SignalBranchRestoreDenial> {
        let _hold = self
            .authorize(admission)
            .map_err(SignalBranchRestoreDenial::OwnerUnavailable)?;
        if admitted_snapshot.owner_runtime_instance_id() != self.runtime_instance_id {
            return Err(SignalBranchRestoreDenial::ForeignSnapshotOwner {
                expected_runtime_instance_id: self.runtime_instance_id,
                observed_runtime_instance_id: admitted_snapshot.owner_runtime_instance_id(),
            });
        }
        let snapshot = admitted_snapshot.snapshot();
        Ok(self
            .lock()
            .snapshot_state(snapshot.meta.branch_id, snapshot.meta.snapshot_id)
            .map(|state| SignalOwnerSnapshotStateBinding {
                runtime_instance_id: self.runtime_instance_id,
                lifecycle_identity: self.lifecycle_identity,
                branch_id: snapshot.meta.branch_id,
                snapshot_id: snapshot.meta.snapshot_id,
                state,
            }))
    }

    pub(super) fn reserve_fork_child<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        parent_branch_id: SignalBranchId,
        child_branch_id: SignalBranchId,
    ) -> Result<SignalOwnerForkLineageReservation<'a, D, I, T>, SignalOwnerUnavailable> {
        let _hold = self.authorize(admission)?;
        self.lock()
            .record_fork_child(parent_branch_id, child_branch_id);
        Ok(SignalOwnerForkLineageReservation {
            metadata: self,
            admission,
            parent_branch_id,
            child_branch_id,
            committed: false,
        })
    }

    pub(super) fn branch_children(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<Vec<SignalBranchId>, SignalOwnerUnavailable> {
        let _hold = self.authorize(admission)?;
        Ok(self.lock().branch_children(branch_id))
    }

    pub(super) fn is_merge_participant(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<bool, SignalOwnerUnavailable> {
        let _hold = self.authorize(admission)?;
        Ok(self.lock().is_merge_participant(branch_id))
    }

    pub(super) fn complete_retirement(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
        parent_branch_id: Option<SignalBranchId>,
        receipt: SignalBranchRetirementReceipt,
    ) -> Result<u32, SignalOwnerUnavailable> {
        let _hold = self.authorize(admission)?;
        let mut state = self.lock();
        let reclaimed = state.remove_retired_branch(branch_id, parent_branch_id);
        state.retain_retirement_receipt(receipt);
        Ok(reclaimed)
    }

    pub(super) fn retirement_receipt(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<Option<SignalBranchRetirementReceipt>, SignalOwnerUnavailable> {
        let _hold = self.authorize(admission)?;
        Ok(self.lock().branch_retirement_receipt(branch_id))
    }

    fn lock(&self) -> MutexGuard<'_, SignalOwnerMetadataState<D, I, T>> {
        match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn authorize<'a>(
        &self,
        admission: &'a SignalOwnerOperationAdmission,
    ) -> Result<super::lifecycle_state::SignalOwnerMetadataHold<'a>, SignalOwnerUnavailable> {
        admission
            .authorize(self.runtime_instance_id, self.lifecycle_identity)
            .map_err(|_| SignalOwnerUnavailable)?;
        admission
            .hold_owner_metadata()
            .map_err(|_| SignalOwnerUnavailable)
    }
}

pub(crate) struct SignalOwnerSnapshotStateBinding<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime_instance_id: u64,
    lifecycle_identity: SignalOwnerLifecycleIdentity,
    branch_id: SignalBranchId,
    snapshot_id: SignalSnapshotId,
    state: SnapshotBranchState<D, I, T>,
}

impl<D, I, T> SignalOwnerSnapshotStateBinding<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::branch) fn into_state_for(
        self,
        runtime_instance_id: u64,
        lifecycle_identity: SignalOwnerLifecycleIdentity,
        admitted_snapshot: &AdmittedSignalBranchSnapshot,
    ) -> Result<SnapshotBranchState<D, I, T>, SignalBranchRestoreDenial> {
        if self.runtime_instance_id != runtime_instance_id
            || self.lifecycle_identity != lifecycle_identity
        {
            return Err(SignalBranchRestoreDenial::OwnerUnavailable(
                SignalOwnerUnavailable,
            ));
        }
        let snapshot = admitted_snapshot.snapshot();
        if admitted_snapshot.owner_runtime_instance_id() != runtime_instance_id {
            return Err(SignalBranchRestoreDenial::ForeignSnapshotOwner {
                expected_runtime_instance_id: runtime_instance_id,
                observed_runtime_instance_id: admitted_snapshot.owner_runtime_instance_id(),
            });
        }
        if self.branch_id != snapshot.meta.branch_id
            || self.snapshot_id != snapshot.meta.snapshot_id
        {
            return Err(SignalBranchRestoreDenial::UnavailableSnapshot {
                branch_id: snapshot.meta.branch_id,
                snapshot_id: snapshot.meta.snapshot_id,
            });
        }
        Ok(self.state)
    }
}

pub(crate) struct SignalOwnerSnapshotReservation<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    metadata: &'a SignalOwnerMetadata<D, I, T>,
    admission: &'a SignalOwnerOperationAdmission,
    snapshot_id: SignalSnapshotId,
    installed: bool,
}

impl<D, I, T> SignalOwnerSnapshotReservation<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn admission(&self) -> &SignalOwnerOperationAdmission {
        self.admission
    }

    pub(super) const fn snapshot_id(&self) -> SignalSnapshotId {
        self.snapshot_id
    }

    pub(crate) fn install(mut self, packet: SnapshotStatePacket<D, I, T>) {
        debug_assert!(
            self.admission.permits_owner_lock_acquisition(),
            "snapshot installation must run after target-cell release"
        );
        self.metadata
            .lock()
            .install_reserved_snapshot(self.snapshot_id, packet);
        self.installed = true;
    }
}

impl<D, I, T> Drop for SignalOwnerSnapshotReservation<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        if !self.installed {
            debug_assert!(
                self.admission.permits_owner_lock_acquisition(),
                "snapshot reservation cleanup must run after target-cell release"
            );
            self.metadata.lock().release_snapshot_capacity();
        }
    }
}

pub(super) struct SignalOwnerForkLineageReservation<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    metadata: &'a SignalOwnerMetadata<D, I, T>,
    admission: &'a SignalOwnerOperationAdmission,
    parent_branch_id: SignalBranchId,
    child_branch_id: SignalBranchId,
    committed: bool,
}

impl<D, I, T> SignalOwnerForkLineageReservation<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn commit(mut self) {
        self.committed = true;
    }
}

impl<D, I, T> Drop for SignalOwnerForkLineageReservation<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        if !self.committed {
            debug_assert!(
                self.admission.permits_owner_lock_acquisition(),
                "fork lineage cleanup must run after target-cell release"
            );
            self.metadata
                .lock()
                .remove_fork_child(self.parent_branch_id, self.child_branch_id);
        }
    }
}
