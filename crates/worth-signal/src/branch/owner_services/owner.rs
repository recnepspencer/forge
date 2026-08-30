use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

use crate::branch::{
    AdmittedSignalBranchBasis, SignalBranchAdmissionLease, SignalBranchBasisDescriptor,
    SignalBranchForkOperationDenial, SignalBranchRetentionAcquisitionDenial,
    SignalBranchRetentionBinding, SignalBranchRetentionLease, SignalBranchRetentionRegistry,
    SignalBranchRetentionTerminalCounts, ValidatedSignalBranchName,
};
use crate::logic::transaction::SignalOwnerPartition;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::mutation_port::{map_fork_registry_denial, SignalOwnerForkReservation};
use super::owner_metadata::SignalOwnerMetadata;

use super::{
    SignalBranchCellState, SignalBranchExecutionCell, SignalBranchRegistry,
    SignalBranchRegistryDenial, SignalBranchRetirement, SignalOwnerAdmissionDenial,
    SignalOwnerLifecycleState, SignalOwnerOperationAdmission, SignalOwnerServiceCostSnapshot,
    SignalOwnerServiceCounters, SignalOwnerUnavailable,
};

pub(crate) const DEFAULT_MAXIMUM_LIVE_SIGNAL_BRANCHES: usize = 4_096;
pub(crate) const DEFAULT_MAXIMUM_SIGNAL_BRANCH_RESERVATIONS: usize = 64;

/// Why the current runtime cannot enter the independent owner-service posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalOwnerServiceIssuanceDenial {
    EventSubscriberStateConfigured,
    ObservationRegistrationStateConfigured,
    ManagedQueueStateConfigured { bound_queue_count: u32 },
    LiveBranchCapacityExhausted { maximum_live_branches: usize },
}

/// Sole strong owner state retained by a sealed, non-cloneable runtime root.
pub(crate) struct SignalOwner<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime_instance_id: u64,
    definition_basis: u64,
    next_branch_id: AtomicU64,
    lifecycle: Arc<SignalOwnerLifecycleState>,
    registry: Arc<SignalBranchRegistry<SignalBranchCellState<D, I, T>>>,
    retention: SignalBranchRetentionRegistry,
    pub(super) metadata: SignalOwnerMetadata<D, I, T>,
    counters: Arc<SignalOwnerServiceCounters>,
}

/// Non-cloneable root field. Before sealing it contains no competing owner.
pub(crate) struct SignalOwnerRoot<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    state: SignalOwnerRootState<D, I, T>,
}

enum SignalOwnerRootState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    Unsealed {
        runtime_instance_id: u64,
        definition_basis: u64,
    },
    Sealed(Arc<SignalOwner<D, I, T>>),
}

impl<D, I, T> SignalOwnerRoot<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime_instance_id: u64, definition_basis: u64) -> Self {
        Self {
            state: SignalOwnerRootState::Unsealed {
                runtime_instance_id,
                definition_basis,
            },
        }
    }

    pub(crate) fn is_sealed(&self) -> bool {
        matches!(self.state, SignalOwnerRootState::Sealed(_))
    }

    pub(crate) fn seal(&mut self, partition: SignalOwnerPartition<D, I, T>) {
        let (runtime_instance_id, definition_basis) = match self.state {
            SignalOwnerRootState::Unsealed {
                runtime_instance_id,
                definition_basis,
            } => (runtime_instance_id, definition_basis),
            SignalOwnerRootState::Sealed(_) => {
                panic!("Signal owner root cannot consume a second canonical partition")
            }
        };
        let owner = SignalOwner::from_partition(runtime_instance_id, definition_basis, partition);
        self.state = SignalOwnerRootState::Sealed(owner);
    }

    pub(crate) fn downgrade_owner(
        &self,
    ) -> Result<Weak<SignalOwner<D, I, T>>, SignalOwnerUnavailable> {
        match &self.state {
            SignalOwnerRootState::Unsealed { .. } => Err(SignalOwnerUnavailable),
            SignalOwnerRootState::Sealed(owner) => Ok(Arc::downgrade(owner)),
        }
    }
}

impl<D, I, T> Drop for SignalOwnerRoot<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        if let SignalOwnerRootState::Sealed(owner) = &self.state {
            let _ = owner.lifecycle.close(owner.runtime_instance_id);
        }
    }
}

impl<D, I, T> SignalOwner<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn from_partition(
        runtime_instance_id: u64,
        definition_basis: u64,
        partition: SignalOwnerPartition<D, I, T>,
    ) -> Arc<Self> {
        let (metadata, next_branch_id, retention, cells) = partition.into_parts();
        let counters = Arc::new(SignalOwnerServiceCounters::default());
        let lifecycle = SignalOwnerLifecycleState::new(runtime_instance_id, Arc::clone(&counters));
        let lifecycle_identity = lifecycle.lifecycle_identity();
        let registry = Arc::new(SignalBranchRegistry::new(
            &lifecycle,
            DEFAULT_MAXIMUM_LIVE_SIGNAL_BRANCHES,
            DEFAULT_MAXIMUM_SIGNAL_BRANCH_RESERVATIONS,
        ));
        let owner = Arc::new(Self {
            runtime_instance_id,
            definition_basis,
            next_branch_id: AtomicU64::new(next_branch_id),
            lifecycle,
            registry,
            retention,
            metadata: SignalOwnerMetadata::new(metadata, runtime_instance_id, lifecycle_identity),
            counters,
        });
        let admission = owner
            .admit()
            .expect("a newly sealed Signal owner admits its canonical cells");
        for (handle, state, head_generation, restore_snapshot_id) in cells {
            let branch_id = handle.id;
            owner
                .registry
                .reserve(&admission, branch_id)
                .and_then(|reservation| {
                    reservation.install(SignalBranchCellState::new(
                        handle,
                        runtime_instance_id,
                        definition_basis,
                        state,
                        head_generation,
                        restore_snapshot_id,
                    ))
                })
                .expect("validated owner partition installs each live branch exactly once");
        }
        drop(admission);
        owner
    }

    pub(super) fn upgrade(weak: &Weak<Self>) -> Result<Arc<Self>, SignalOwnerUnavailable> {
        let owner = weak.upgrade().ok_or(SignalOwnerUnavailable)?;
        owner.counters.record_owner_upgrade_attempt();
        Ok(owner)
    }

    pub(super) fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub(super) fn definition_basis(&self) -> u64 {
        self.definition_basis
    }

    pub(super) fn lifecycle_identity(
        &self,
    ) -> super::lifecycle_state::SignalOwnerLifecycleIdentity {
        self.lifecycle.lifecycle_identity()
    }

    pub(super) fn lifecycle_observation(&self) -> super::SignalOwnerLifecycleObservation {
        self.lifecycle.observation()
    }

    pub(super) fn admit(
        self: &Arc<Self>,
    ) -> Result<SignalOwnerOperationAdmission, SignalOwnerAdmissionDenial> {
        self.lifecycle.admit(self.runtime_instance_id)
    }

    pub(super) fn lookup_cell(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<
        Arc<SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>>,
        SignalBranchRegistryDenial,
    > {
        self.registry.lookup(admission, branch_id)
    }

    pub(super) fn live_count(&self) -> usize {
        self.registry.live_count()
    }

    #[cfg(test)]
    pub(super) fn reservation_count(&self) -> usize {
        self.registry.reservation_count()
    }

    pub(super) fn begin_retirement<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<
        SignalBranchRetirement<'a, SignalBranchCellState<D, I, T>>,
        SignalBranchRegistryDenial,
    > {
        self.registry.begin_retirement(admission, branch_id)
    }

    pub(super) fn reserve_fork_destination<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission,
        source: &AdmittedSignalBranchBasis,
        requested_identity: ValidatedSignalBranchName,
    ) -> Result<SignalOwnerForkReservation<'a, D, I, T>, SignalBranchForkOperationDenial> {
        let branch_id = self
            .next_branch_id
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                current.checked_add(1)
            })
            .map(SignalBranchId)
            .map_err(|_| SignalBranchForkOperationDenial::BranchIdentityExhausted)?;
        let parent_head_snapshot_id = source
            .observation()
            .target()
            .as_basis()
            .and_then(|target| target.snapshot_id())
            .map(SignalSnapshotId);
        let handle = SignalBranchHandle {
            id: branch_id,
            name: requested_identity.into_inner(),
            parent_branch_id: Some(source.owner_branch_id()),
            head_snapshot_id: parent_head_snapshot_id,
        };
        let reservation = self
            .registry
            .reserve(admission, branch_id)
            .map_err(map_fork_registry_denial)?;
        let lineage = self
            .metadata
            .reserve_fork_child(admission, source.owner_branch_id(), branch_id)
            .map_err(SignalBranchForkOperationDenial::OwnerUnavailable)?;
        Ok(SignalOwnerForkReservation::new(
            handle,
            self.runtime_instance_id,
            self.definition_basis,
            reservation,
            lineage,
        ))
    }

    #[allow(
        dead_code,
        reason = "Phase 4 basis operations consume this frozen owner seam"
    )]
    pub(super) fn acquire_admitted_retention(
        &self,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchAdmissionLease, SignalBranchRetentionAcquisitionDenial> {
        self.counters.record_retention_registry_contact();
        self.retention.acquire_admitted(branch_id)
    }

    #[allow(
        dead_code,
        reason = "Phase 4 basis operations consume this frozen owner seam"
    )]
    pub(super) fn acquire_external_retention(
        &self,
        descriptor: SignalBranchBasisDescriptor,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        self.counters.record_retention_registry_contact();
        self.retention.acquire_external(descriptor)
    }

    #[allow(
        dead_code,
        reason = "Phase 4 basis and lifecycle operations consume this seam"
    )]
    pub(super) fn retention_binding(&self) -> SignalBranchRetentionBinding {
        self.retention.binding()
    }

    #[allow(
        dead_code,
        reason = "Phase 4 lifecycle inspection consumes this frozen seam"
    )]
    pub(super) fn retention_terminal_counts(&self) -> SignalBranchRetentionTerminalCounts {
        self.retention.terminal_counts()
    }

    #[cfg(test)]
    pub(super) fn admitted_retention_count(&self, branch_id: SignalBranchId) -> u32 {
        self.retention.admitted_count(branch_id)
    }

    #[cfg(test)]
    pub(super) fn metadata_membership_is_drained(
        &self,
        admission: &SignalOwnerOperationAdmission,
    ) -> Result<bool, SignalBranchRegistryDenial> {
        admission
            .authorize(
                self.runtime_instance_id,
                self.lifecycle.lifecycle_identity(),
            )
            .map_err(SignalBranchRegistryDenial::from)?;
        let _metadata_hold = admission
            .hold_owner_metadata()
            .map_err(|_| SignalBranchRegistryDenial::OwnerMetadataOrdering)?;
        Ok(self.metadata.membership_is_drained_unchecked())
    }

    pub(super) fn cost_snapshot(&self) -> SignalOwnerServiceCostSnapshot {
        self.counters.snapshot()
    }
}
