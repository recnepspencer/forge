use crate::{BackendCapabilityKind, WalDurabilityBarrierSet};

use super::StoreDurabilityFileSyncKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityPublicationKind {
    WalFrame,
    Checkpoint,
    Manifest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreDurabilityRequirement {
    publication: StoreDurabilityPublicationKind,
    required_barriers: WalDurabilityBarrierSet,
    required_file_sync: StoreDurabilityFileSyncKind,
    requires_directory_sync: bool,
    requires_parent_namespace_durable: bool,
    requires_rename_durable: bool,
    requires_ordering_barrier: bool,
}

impl StoreDurabilityRequirement {
    pub const fn wal_ordering_barrier(required_barriers: WalDurabilityBarrierSet) -> Self {
        Self {
            publication: StoreDurabilityPublicationKind::WalFrame,
            required_barriers,
            required_file_sync: StoreDurabilityFileSyncKind::Fdatasync,
            requires_directory_sync: false,
            requires_parent_namespace_durable: false,
            requires_rename_durable: false,
            requires_ordering_barrier: true,
        }
    }

    pub const fn checkpoint_publication(required_barriers: WalDurabilityBarrierSet) -> Self {
        Self {
            publication: StoreDurabilityPublicationKind::Checkpoint,
            required_barriers,
            required_file_sync: StoreDurabilityFileSyncKind::Fsync,
            requires_directory_sync: true,
            requires_parent_namespace_durable: true,
            requires_rename_durable: true,
            requires_ordering_barrier: true,
        }
    }

    pub const fn manifest_publication(required_barriers: WalDurabilityBarrierSet) -> Self {
        Self {
            publication: StoreDurabilityPublicationKind::Manifest,
            required_barriers,
            required_file_sync: StoreDurabilityFileSyncKind::Fsync,
            requires_directory_sync: true,
            requires_parent_namespace_durable: true,
            requires_rename_durable: true,
            requires_ordering_barrier: true,
        }
    }

    pub const fn with_parent_namespace_durability(mut self) -> Self {
        self.requires_directory_sync = true;
        self.requires_parent_namespace_durable = true;
        self
    }

    pub const fn with_rename_durability(mut self) -> Self {
        self.requires_directory_sync = true;
        self.requires_parent_namespace_durable = true;
        self.requires_rename_durable = true;
        self
    }

    pub const fn publication(self) -> StoreDurabilityPublicationKind {
        self.publication
    }

    pub const fn required_barriers(self) -> WalDurabilityBarrierSet {
        self.required_barriers
    }

    pub const fn required_file_sync(self) -> StoreDurabilityFileSyncKind {
        self.required_file_sync
    }

    pub const fn requires_fsync(self) -> bool {
        matches!(self.required_file_sync, StoreDurabilityFileSyncKind::Fsync)
    }

    pub const fn requires_fdatasync(self) -> bool {
        matches!(
            self.required_file_sync,
            StoreDurabilityFileSyncKind::Fdatasync
        )
    }

    pub const fn requires_directory_sync(self) -> bool {
        self.requires_directory_sync
    }

    pub const fn requires_parent_namespace_durable(self) -> bool {
        self.requires_parent_namespace_durable
    }

    pub const fn requires_rename_durable(self) -> bool {
        self.requires_rename_durable
    }

    pub const fn requires_ordering_barrier(self) -> bool {
        self.requires_ordering_barrier
    }

    pub const fn required_capability(self) -> BackendCapabilityKind {
        if self.requires_rename_durable {
            BackendCapabilityKind::DurableRename
        } else if self.requires_directory_sync {
            BackendCapabilityKind::DirectorySync
        } else {
            BackendCapabilityKind::Fsync
        }
    }
}
