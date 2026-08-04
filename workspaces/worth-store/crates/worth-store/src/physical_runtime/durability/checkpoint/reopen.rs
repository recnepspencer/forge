mod binding_compaction;

pub(in crate::physical_runtime) use binding_compaction::{
    reopen_binding_compaction, NamespaceDurablePhysicalBindingCompactionReopen,
    PhysicalBindingCompactionRecordStreamFailure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::physical_runtime) struct PhysicalBindingCompactionReopenCounters {
    pub(super) checkpoint_artifact_bytes: u64,
    pub(super) checkpoint_bytes_read: u64,
    pub(super) dirty_body_bytes_skipped: u64,
    pub(super) binding_records_read: u64,
}

pub(in crate::physical_runtime) enum ReopenedPhysicalBindingCompaction {
    GenerationZero,
    NamespaceDurable(NamespaceDurablePhysicalBindingCompactionReopen),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBindingCompactionReopenFailure {
    Media(worth_store_physical_backend::ArtifactTreeFailure),
    ArtifactTooShort,
    ArtifactLayoutMismatch,
    ForeignStore,
    Format(worth_store_physical_format::CheckpointStreamDecodeDenial),
    CounterOverflow,
    AllocationRejected,
}

impl PhysicalBindingCompactionReopenCounters {
    pub(in crate::physical_runtime) const fn checkpoint_artifact_bytes(self) -> u64 {
        self.checkpoint_artifact_bytes
    }

    pub(in crate::physical_runtime) const fn checkpoint_bytes_read(self) -> u64 {
        self.checkpoint_bytes_read
    }

    pub(in crate::physical_runtime) const fn dirty_body_bytes_skipped(self) -> u64 {
        self.dirty_body_bytes_skipped
    }

    pub(in crate::physical_runtime) const fn binding_records_read(self) -> u64 {
        self.binding_records_read
    }
}

impl ReopenedPhysicalBindingCompaction {
    pub(in crate::physical_runtime) const fn wal_cutoff_lsn_exclusive(&self) -> Option<u64> {
        match self {
            Self::GenerationZero => None,
            Self::NamespaceDurable(reopened) => Some(reopened.wal_cutoff_lsn_exclusive()),
        }
    }
}
