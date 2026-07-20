use super::super::super::*;
use worth_store_physical_format::store_namespace::{
    NamespaceInitializationAttempt, StagedNamespaceName,
};

pub(super) fn owner() -> (tempfile::TempDir, FilesystemMediaOwner) {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("admit media owner");
    (parent, owner)
}

pub(super) fn staged_path(owner: &FilesystemMediaOwner, value: u8) -> StagedNamespacePath {
    let attempt =
        NamespaceInitializationAttempt::from_nonzero_bytes([value; 16]).expect("nonzero attempt");
    owner.staged_identity_path(&StagedNamespaceName::for_identity(attempt))
}

pub(super) fn synchronized_staged<'owner>(
    owner: &'owner FilesystemMediaOwner,
    value: u8,
    bytes: &[u8],
) -> SynchronizedStagedNamespaceFile<'owner> {
    let staged = match StagedNamespaceFile::create(owner, staged_path(owner, value)) {
        StagedNamespaceFileOutcome::Created(staged) => staged,
        other => panic!("create staged file: {other:?}"),
    };
    let completed = match staged.write_all(bytes) {
        StagedNamespaceWriteOutcome::Completed(completed) => completed,
        other => panic!("write staged file: {other:?}"),
    };
    match completed.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        other => panic!("synchronize staged file: {other:?}"),
    }
}

pub(super) fn created(outcome: NamespaceFileOpenOutcome<'_>) -> NamespaceFileHandle<'_> {
    match outcome.into_result() {
        NamespaceFileOpenResult::Opened {
            kind: NamespaceFileOpenKind::CreatedNew,
            handle,
        } => handle,
        other => panic!("create-new failed: {other:?}"),
    }
}
