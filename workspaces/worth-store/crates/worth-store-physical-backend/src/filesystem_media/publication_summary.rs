use super::{
    DirectoryPublicationSynchronization, FileStateSynchronization, MediaOperationIdentity,
    RootParentPublicationSynchronization, StoreRootPublicationSynchronization,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationWriteSummary {
    first_operation: MediaOperationIdentity,
    last_operation: MediaOperationIdentity,
    primitive_attempts: u64,
    bytes: u64,
}

impl PublicationWriteSummary {
    pub(super) const fn new(
        first_operation: MediaOperationIdentity,
        last_operation: MediaOperationIdentity,
        primitive_attempts: u64,
        bytes: u64,
    ) -> Self {
        Self {
            first_operation,
            last_operation,
            primitive_attempts,
            bytes,
        }
    }

    pub const fn first_operation(self) -> MediaOperationIdentity {
        self.first_operation
    }

    pub const fn last_operation(self) -> MediaOperationIdentity {
        self.last_operation
    }

    pub const fn primitive_attempts(self) -> u64 {
        self.primitive_attempts
    }

    pub const fn bytes(self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespacePublicationSummary {
    create_operation: MediaOperationIdentity,
    write: PublicationWriteSummary,
    file_state_synchronization: FileStateSynchronization,
    rename_operation: MediaOperationIdentity,
    namespace_directory_synchronization: DirectoryPublicationSynchronization,
    store_root_synchronization: Option<StoreRootPublicationSynchronization>,
    root_parent_synchronization: Option<RootParentPublicationSynchronization>,
}

impl NamespacePublicationSummary {
    pub(super) const fn new(
        create_operation: MediaOperationIdentity,
        write: PublicationWriteSummary,
        file_state_synchronization: FileStateSynchronization,
        rename_operation: MediaOperationIdentity,
        namespace_directory_synchronization: DirectoryPublicationSynchronization,
        store_root_synchronization: Option<StoreRootPublicationSynchronization>,
        root_parent_synchronization: Option<RootParentPublicationSynchronization>,
    ) -> Self {
        Self {
            create_operation,
            write,
            file_state_synchronization,
            rename_operation,
            namespace_directory_synchronization,
            store_root_synchronization,
            root_parent_synchronization,
        }
    }

    pub const fn create_operation(self) -> MediaOperationIdentity {
        self.create_operation
    }

    pub const fn write(self) -> PublicationWriteSummary {
        self.write
    }

    pub const fn file_state_synchronization(self) -> FileStateSynchronization {
        self.file_state_synchronization
    }

    pub const fn rename_operation(self) -> MediaOperationIdentity {
        self.rename_operation
    }

    pub const fn namespace_directory_synchronization(self) -> DirectoryPublicationSynchronization {
        self.namespace_directory_synchronization
    }

    pub const fn store_root_synchronization(self) -> Option<StoreRootPublicationSynchronization> {
        self.store_root_synchronization
    }

    pub const fn root_parent_synchronization(self) -> Option<RootParentPublicationSynchronization> {
        self.root_parent_synchronization
    }
}
