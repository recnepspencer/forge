#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationShutdown {
    observation: crate::physical_runtime::PhysicalMutationObservation,
}

pub(in crate::physical_runtime) struct PhysicalMutationTerminalState {
    shutdown: PhysicalMutationShutdown,
    completed_unobserved: Box<[crate::physical_runtime::CompletedUnobservedPhysicalMutation]>,
    cost: PhysicalMutationCostSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalMutationCostSnapshot {
    pub(super) groups_formed: u64,
    pub(super) data_writes: u64,
    pub(super) data_bytes: u64,
    pub(super) records: u64,
    pub(super) acknowledgments: u64,
    pub(super) peak_group_members: u64,
}

impl PhysicalMutationShutdown {
    pub(in crate::physical_runtime) const fn from_observation(
        observation: crate::physical_runtime::PhysicalMutationObservation,
    ) -> Self {
        Self { observation }
    }

    pub const fn started(self) -> u64 {
        self.observation.started()
    }

    pub const fn completed(self) -> u64 {
        self.observation.completed()
    }

    pub const fn proven_no_effect(self) -> u64 {
        self.observation.proven_no_effect()
    }

    pub const fn indeterminate(self) -> u64 {
        self.observation.indeterminate()
    }

    pub const fn completed_unobserved(self) -> u64 {
        self.observation.completed_unobserved()
    }

    pub const fn worker_panics(self) -> u64 {
        self.observation.worker_panics()
    }

    pub const fn cancellation_accepted(self) -> u64 {
        self.observation.cancellation_accepted()
    }

    pub const fn cancellation_effectful(self) -> u64 {
        self.observation.cancellation_effectful()
    }

    pub const fn cancellation_terminal(self) -> u64 {
        self.observation.cancellation_terminal()
    }

    pub const fn cancellation_stale(self) -> u64 {
        self.observation.cancellation_stale()
    }

    pub const fn cancellation_runtime_closing(self) -> u64 {
        self.observation.cancellation_runtime_closing()
    }

    pub const fn requires_inspection(self) -> bool {
        self.observation.requires_inspection()
    }
}

impl PhysicalMutationTerminalState {
    pub(in crate::physical_runtime) fn new(
        shutdown: PhysicalMutationShutdown,
        completed_unobserved: Vec<crate::physical_runtime::CompletedUnobservedPhysicalMutation>,
        cost: PhysicalMutationCostSnapshot,
    ) -> Self {
        Self {
            shutdown,
            completed_unobserved: completed_unobserved.into_boxed_slice(),
            cost,
        }
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        PhysicalMutationShutdown,
        Box<[crate::physical_runtime::CompletedUnobservedPhysicalMutation]>,
        PhysicalMutationCostSnapshot,
    ) {
        (self.shutdown, self.completed_unobserved, self.cost)
    }
}

impl PhysicalMutationCostSnapshot {
    pub(in crate::physical_runtime) const fn values(self) -> [u64; 6] {
        [
            self.groups_formed,
            self.data_writes,
            self.data_bytes,
            self.records,
            self.acknowledgments,
            self.peak_group_members,
        ]
    }
}
