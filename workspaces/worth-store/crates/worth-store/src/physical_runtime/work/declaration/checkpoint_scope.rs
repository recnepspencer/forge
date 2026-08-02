use worth_store_physical_format::PhysicalCheckpointIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalCheckpointWorkAction {
    CreateCandidate { byte_count: u64 },
    AppendCandidate { offset: u64, byte_count: u64 },
    SynchronizeCandidate,
    RemoveCandidate,
    PublishCandidate,
    SynchronizeNamespace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalCheckpointWorkScope {
    checkpoint: PhysicalCheckpointIdentity,
    action: PhysicalCheckpointWorkAction,
}

impl PhysicalCheckpointWorkScope {
    pub(in crate::physical_runtime) const fn new(
        checkpoint: PhysicalCheckpointIdentity,
        action: PhysicalCheckpointWorkAction,
    ) -> Option<Self> {
        match action {
            PhysicalCheckpointWorkAction::CreateCandidate { byte_count: 0 } => None,
            PhysicalCheckpointWorkAction::AppendCandidate { byte_count: 0, .. } => None,
            _ => Some(Self { checkpoint, action }),
        }
    }

    pub(in crate::physical_runtime) const fn checkpoint(self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub(in crate::physical_runtime) const fn action(self) -> PhysicalCheckpointWorkAction {
        self.action
    }

    pub(in crate::physical_runtime) const fn accounted_bytes(self) -> u64 {
        match self.action {
            PhysicalCheckpointWorkAction::CreateCandidate { byte_count }
            | PhysicalCheckpointWorkAction::AppendCandidate { byte_count, .. } => byte_count,
            PhysicalCheckpointWorkAction::SynchronizeCandidate
            | PhysicalCheckpointWorkAction::RemoveCandidate
            | PhysicalCheckpointWorkAction::PublishCandidate
            | PhysicalCheckpointWorkAction::SynchronizeNamespace => 1,
        }
    }
}
