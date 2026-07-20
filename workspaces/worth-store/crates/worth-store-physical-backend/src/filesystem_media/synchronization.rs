use super::{MediaHandleIdentity, MediaOperationFailure};

macro_rules! define_sync_fact {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            pub(super) operation: super::MediaOperationIdentity,
            pub(super) handle: MediaHandleIdentity,
        }

        impl $name {
            pub const fn operation(self) -> super::MediaOperationIdentity {
                self.operation
            }

            pub const fn handle(self) -> MediaHandleIdentity {
                self.handle
            }
        }
    };
}

define_sync_fact!(FileDataSynchronization);
define_sync_fact!(FileStateSynchronization);
define_sync_fact!(DirectoryPublicationSynchronization);
define_sync_fact!(StoreRootPublicationSynchronization);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootParentPublicationSynchronization {
    pub(super) operation: super::MediaOperationIdentity,
}

impl RootParentPublicationSynchronization {
    pub const fn operation(self) -> super::MediaOperationIdentity {
        self.operation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDataSynchronizationOutcome {
    Synchronized(FileDataSynchronization),
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStateSynchronizationOutcome {
    Synchronized(FileStateSynchronization),
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectoryPublicationSynchronizationOutcome {
    Synchronized(DirectoryPublicationSynchronization),
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreRootPublicationSynchronizationOutcome {
    NotRequired,
    Synchronized(StoreRootPublicationSynchronization),
    Failed(MediaOperationFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootParentPublicationSynchronizationOutcome {
    NotRequired,
    Synchronized(RootParentPublicationSynchronization),
    Failed(MediaOperationFailure),
}
