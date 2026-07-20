use super::{
    FilesystemMediaOwner, MediaOperationFailure, NamespaceFileHandle, NamespaceRelativePath,
};

#[derive(Debug)]
pub struct StagedNamespaceFile<'owner> {
    pub(super) owner: &'owner FilesystemMediaOwner,
    pub(super) path: NamespaceRelativePath,
    pub(super) handle: NamespaceFileHandle<'owner>,
    pub(super) create_operation: super::MediaOperationIdentity,
}

#[derive(Debug)]
pub enum StagedNamespaceFileOutcome<'owner> {
    Created(StagedNamespaceFile<'owner>),
    Failed(MediaOperationFailure),
}

#[derive(Debug)]
pub struct CompletedStagedNamespaceWrite<'owner> {
    pub(super) staged: StagedNamespaceFile<'owner>,
    pub(super) write: super::PublicationWriteSummary,
}

#[derive(Debug)]
pub enum StagedNamespaceWriteOutcome<'owner> {
    Completed(CompletedStagedNamespaceWrite<'owner>),
    Failed {
        staged: StagedNamespaceFile<'owner>,
        completed_bytes: u64,
        failure: MediaOperationFailure,
    },
}

#[derive(Debug)]
pub struct SynchronizedStagedNamespaceFile<'owner> {
    pub(super) completed: CompletedStagedNamespaceWrite<'owner>,
    pub(super) synchronization: super::FileStateSynchronization,
}

#[derive(Debug)]
pub enum StagedNamespaceSynchronizationOutcome<'owner> {
    Synchronized(SynchronizedStagedNamespaceFile<'owner>),
    Failed {
        completed: CompletedStagedNamespaceWrite<'owner>,
        failure: MediaOperationFailure,
    },
}

#[derive(Debug)]
pub struct CompletedAtomicReplacement<'owner> {
    pub(super) owner: &'owner FilesystemMediaOwner,
    pub(super) destination: NamespaceRelativePath,
    pub(super) create_operation: super::MediaOperationIdentity,
    pub(super) write: super::PublicationWriteSummary,
    pub(super) file_state_synchronization: super::FileStateSynchronization,
    pub(super) rename_operation: super::MediaOperationIdentity,
    pub(super) _namespace_authority:
        super::mutation_ownership::CoordinatedNamespaceMutation<'owner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespacePublicationStage {
    StagedWrite,
    FileStateSynchronization,
    AtomicReplacement,
    DirectoryPublicationSynchronization,
    StoreRootPublicationSynchronization,
    RootParentPublicationSynchronization,
}

#[derive(Debug)]
pub struct IndeterminateNamespacePublication<'owner> {
    pub(super) owner: &'owner FilesystemMediaOwner,
    pub(super) stage: NamespacePublicationStage,
    pub(super) failure: MediaOperationFailure,
}

impl<'owner> IndeterminateNamespacePublication<'owner> {
    pub(super) const fn new(
        owner: &'owner FilesystemMediaOwner,
        stage: NamespacePublicationStage,
        failure: MediaOperationFailure,
    ) -> Self {
        Self {
            owner,
            stage,
            failure,
        }
    }
}

#[derive(Debug)]
pub enum AtomicReplacementOutcome<'owner> {
    Replaced(CompletedAtomicReplacement<'owner>),
    Denied(MediaOperationFailure),
    Indeterminate(IndeterminateNamespacePublication<'owner>),
}

#[derive(Debug)]
pub struct DurablyPublishedNamespaceFile {
    pub(super) destination: NamespaceRelativePath,
    pub(super) summary: super::NamespacePublicationSummary,
}

#[derive(Debug)]
pub enum DurableNamespacePublicationOutcome<'owner> {
    Published(DurablyPublishedNamespaceFile),
    Indeterminate(IndeterminateNamespacePublication<'owner>),
}
