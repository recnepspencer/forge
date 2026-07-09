use crate::{BackendTargetProfile, CapabilityEvidenceClass, WalDurabilityBarrierSet};

use super::{
    StoreDurabilityCounterSnapshot, StoreDurabilityRequirement, StoreDurabilityWriteAccepted,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityFileSyncKind {
    Fdatasync,
    Fsync,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityExecutionProof<S> {
    binding: StoreDurabilityExecutionBinding<S>,
    completed_barriers: WalDurabilityBarrierSet,
    file_sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
    delayed_syncs: u64,
    failed_syncs: u64,
    _seal: StoreDurabilityExecutionSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StoreDurabilityExecutionSeal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreOwnedDurabilityExecution {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoreDurabilityExecutionBinding<S> {
    scope: S,
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    requirement: StoreDurabilityRequirement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurabilityExecutionRequest<S> {
    binding: StoreDurabilityExecutionBinding<S>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreDurabilityExecutionObservation {
    completed_barriers: WalDurabilityBarrierSet,
    file_sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
    delayed_syncs: u64,
    failed_syncs: u64,
}

pub trait PhysicalStoreDurabilityExecutor<S> {
    type Error;

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error>;
}

pub struct StoreDurabilityExecutionSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedDurabilityExecution,
}

impl StoreOwnedDurabilityExecution {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    fn complete<S>(
        self,
        binding: StoreDurabilityExecutionBinding<S>,
        observation: StoreDurabilityExecutionObservation,
    ) -> StoreDurabilityExecutionProof<S> {
        StoreDurabilityExecutionProof {
            binding,
            completed_barriers: observation.completed_barriers,
            file_sync: observation.file_sync,
            directory_sync_completed: observation.directory_sync_completed,
            rename_completed: observation.rename_completed,
            ordering_barrier_completed: observation.ordering_barrier_completed,
            delayed_syncs: observation.delayed_syncs,
            failed_syncs: observation.failed_syncs,
            _seal: StoreDurabilityExecutionSeal,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }
}

impl<S> StoreDurabilityExecutionBinding<S> {
    fn from_accepted(accepted: &StoreDurabilityWriteAccepted<S>) -> Self
    where
        S: Clone,
    {
        Self {
            scope: accepted.scope().clone(),
            profile: accepted.profile(),
            evidence_class: accepted.evidence_class(),
            requirement: accepted.requirement(),
        }
    }
}

impl<S> StoreDurabilityExecutionRequest<S> {
    pub(crate) fn from_accepted(accepted: &StoreDurabilityWriteAccepted<S>) -> Self
    where
        S: Clone,
    {
        Self {
            binding: StoreDurabilityExecutionBinding::from_accepted(accepted),
        }
    }

    pub const fn scope(&self) -> &S {
        &self.binding.scope
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.binding.profile
    }

    pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
        self.binding.evidence_class
    }

    pub const fn requirement(&self) -> StoreDurabilityRequirement {
        self.binding.requirement
    }
}

impl StoreDurabilityExecutionObservation {
    pub const fn new(
        completed_barriers: WalDurabilityBarrierSet,
        file_sync: StoreDurabilityFileSyncKind,
    ) -> Self {
        Self {
            completed_barriers,
            file_sync,
            directory_sync_completed: false,
            rename_completed: false,
            ordering_barrier_completed: false,
            delayed_syncs: 0,
            failed_syncs: 0,
        }
    }

    pub const fn with_directory_sync_completed(mut self) -> Self {
        self.directory_sync_completed = true;
        self
    }

    pub const fn with_rename_completed(mut self) -> Self {
        self.rename_completed = true;
        self
    }

    pub const fn with_ordering_barrier_completed(mut self) -> Self {
        self.ordering_barrier_completed = true;
        self
    }

    pub const fn with_delayed_syncs(mut self, delayed_syncs: u64) -> Self {
        self.delayed_syncs = delayed_syncs;
        self
    }

    pub const fn with_failed_syncs(mut self, failed_syncs: u64) -> Self {
        self.failed_syncs = failed_syncs;
        self
    }
}

impl<'backend, Backend> StoreDurabilityExecutionSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedDurabilityExecution,
    ) -> Self {
        Self { backend, authority }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedDurabilityExecution::store_owned())
    }

    pub fn execute<S>(
        &mut self,
        accepted: &StoreDurabilityWriteAccepted<S>,
    ) -> Result<StoreDurabilityExecutionProof<S>, Backend::Error>
    where
        Backend: PhysicalStoreDurabilityExecutor<S>,
        S: Clone,
    {
        let request = StoreDurabilityExecutionRequest::from_accepted(accepted);
        let binding = StoreDurabilityExecutionBinding::from_accepted(accepted);
        let observation = self.backend.execute_durability(request)?;
        Ok(self.authority.complete(binding, observation))
    }
}

impl<S> StoreDurabilityExecutionProof<S> {
    pub const fn completed_barriers(&self) -> WalDurabilityBarrierSet {
        self.completed_barriers
    }

    pub const fn file_sync(&self) -> StoreDurabilityFileSyncKind {
        self.file_sync
    }

    pub const fn directory_sync_completed(&self) -> bool {
        self.directory_sync_completed
    }

    pub const fn rename_completed(&self) -> bool {
        self.rename_completed
    }

    pub const fn ordering_barrier_completed(&self) -> bool {
        self.ordering_barrier_completed
    }

    pub const fn failed_syncs(&self) -> u64 {
        self.failed_syncs
    }

    pub(crate) fn binds_accepted(&self, accepted: &StoreDurabilityWriteAccepted<S>) -> bool
    where
        S: Eq,
    {
        self.binding.scope == *accepted.scope()
            && self.binding.profile == accepted.profile()
            && self.binding.evidence_class == accepted.evidence_class()
            && self.binding.requirement == accepted.requirement()
    }

    pub(crate) const fn apply_boundary_counters(
        &self,
        counters: StoreDurabilityCounterSnapshot,
    ) -> StoreDurabilityCounterSnapshot {
        counters
            .with_flush_completed()
            .with_file_sync_completed(self.file_sync)
            .with_delayed_syncs(self.delayed_syncs)
            .with_failed_syncs(self.failed_syncs)
    }
}
