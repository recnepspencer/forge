use std::path::Path;

use worth_store_physical_format::store_namespace::{
    NamespaceInitializationAttempt, StagedNamespaceName,
};

use super::{
    AllocationRequest, AppendRequest, AtomicReplacementOutcome, DurableDeletionOutcome,
    DurableNamespacePublicationOutcome, FilesystemMediaOwner, MediaAllocationMode,
    MediaAllocationResult, MediaMetadataResult, MediaOperationResult, NamespaceDeletionOutcome,
    NamespaceDirectoryListingResult, NamespaceEntryBatchResult, NamespaceFileOpenResult,
    PositionedWriteRequest, StagedNamespaceFile, StagedNamespaceFileOutcome,
    StagedNamespaceSynchronizationOutcome, StagedNamespaceWriteOutcome, TruncateRequest,
};

const OPERATION_BUFFER_BYTES: usize = 64 * 1_024;
const POSITIONED_PAYLOAD_BYTES: usize = 1_024 * 1_024;
const APPEND_PAYLOAD_BYTES: usize = 257;
const PUBLICATION_BYTES: &[u8] = b"qualification-publication";

pub(super) fn run_bounded_qualification(
    owner: &FilesystemMediaOwner,
    root: &Path,
) -> Result<(), ()> {
    let names = [
        qualification_name()?,
        qualification_name()?,
        qualification_name()?,
    ];
    let result = execute(owner, root, &names);
    let mut cleaned = true;
    for name in &names {
        owner.boundary().counters().cleanup_action();
        cleaned &= durable_cleanup(owner, name).is_ok();
    }
    let mut no_residue = true;
    for name in &names {
        let exists = root.join("namespace").join(name.as_str()).exists();
        if exists {
            owner.boundary().counters().preserve_residue();
        }
        no_residue &= !exists;
    }
    result
        .and(cleaned.then_some(()).ok_or(()))
        .and(no_residue.then_some(()).ok_or(()))
}

fn execute(
    owner: &FilesystemMediaOwner,
    root: &Path,
    names: &[StagedNamespaceName; 3],
) -> Result<(), ()> {
    exercise_file_operations(owner, &names[0])?;
    exercise_replacement(owner, root, &names[1], &names[2])
}

fn exercise_file_operations(
    owner: &FilesystemMediaOwner,
    name: &StagedNamespaceName,
) -> Result<(), ()> {
    let path = owner.staged_identity_path(name);
    let handle = match owner.create_new(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        _ => return Err(()),
    };
    let mut buffer = [0_u8; OPERATION_BUFFER_BYTES];
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = deterministic_byte(index);
    }
    let mut offset = 0;
    while offset < POSITIONED_PAYLOAD_BYTES {
        require_completed(
            handle.positioned_write(PositionedWriteRequest::new(offset as u64, &buffer)),
        )?;
        offset += buffer.len();
    }
    let mut tail = [0_u8; APPEND_PAYLOAD_BYTES];
    for (index, byte) in tail.iter_mut().enumerate() {
        *byte = deterministic_byte(POSITIONED_PAYLOAD_BYTES + index);
    }
    require_completed(handle.append(AppendRequest::new(&tail)))?;
    if !matches!(handle.metadata().result(), MediaMetadataResult::Observed(_)) {
        return Err(());
    }
    let mut listing = match owner.begin_directory_listing(owner.namespace_directory()) {
        NamespaceDirectoryListingResult::Opened(listing) => listing,
        NamespaceDirectoryListingResult::Failed(_) => return Err(()),
    };
    if !matches!(
        listing.next_batch(64).result(),
        NamespaceEntryBatchResult::Observed(_)
    ) {
        return Err(());
    }
    require_completed(handle.truncate(TruncateRequest::new(8)))?;
    if !matches!(
        handle
            .allocate(AllocationRequest::new(
                8,
                8,
                MediaAllocationMode::LogicalLengthOnly,
            ))
            .result(),
        MediaAllocationResult::Completed(_)
    ) {
        return Err(());
    }
    if !matches!(
        handle.synchronize_state(),
        super::FileStateSynchronizationOutcome::Synchronized(_)
    ) {
        return Err(());
    }
    Ok(())
}

fn deterministic_byte(offset: usize) -> u8 {
    ((offset as u64).wrapping_mul(131).wrapping_add(17) & 0xff) as u8
}

fn exercise_replacement(
    owner: &FilesystemMediaOwner,
    root: &Path,
    source_name: &StagedNamespaceName,
    destination_name: &StagedNamespaceName,
) -> Result<(), ()> {
    let staged = match StagedNamespaceFile::create(owner, owner.staged_identity_path(source_name)) {
        StagedNamespaceFileOutcome::Created(staged) => staged,
        _ => return Err(()),
    };
    let completed = match staged.write_all(PUBLICATION_BYTES) {
        StagedNamespaceWriteOutcome::Completed(completed) => completed,
        _ => return Err(()),
    };
    let synchronized = match completed.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        _ => return Err(()),
    };
    let replaced = match synchronized.replace(owner.staged_publication_target(destination_name)) {
        AtomicReplacementOutcome::Replaced(replaced) => replaced,
        _ => return Err(()),
    };
    if !matches!(
        replaced.synchronize_publication(),
        DurableNamespacePublicationOutcome::Published(_)
    ) {
        return Err(());
    }
    let observed =
        std::fs::read(root.join("namespace").join(destination_name.as_str())).map_err(|_| ())?;
    (observed == PUBLICATION_BYTES).then_some(()).ok_or(())
}

fn require_completed(outcome: super::MediaOperationOutcome) -> Result<(), ()> {
    matches!(outcome.result(), MediaOperationResult::Completed(_))
        .then_some(())
        .ok_or(())
}

fn durable_cleanup(owner: &FilesystemMediaOwner, name: &StagedNamespaceName) -> Result<(), ()> {
    let path = owner.staged_identity_path(name);
    let handle = match owner.open_existing_for_mutation(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        NamespaceFileOpenResult::Failed(failure)
            if failure.context().io_kind() == Some(std::io::ErrorKind::NotFound) =>
        {
            return Ok(())
        }
        NamespaceFileOpenResult::Failed(_) => return Err(()),
    };
    let removed = match owner.delete_namespace_file(handle) {
        NamespaceDeletionOutcome::Removed(removed) => removed,
        _ => return Err(()),
    };
    matches!(
        removed.synchronize_removal(),
        DurableDeletionOutcome::Durable(_)
    )
    .then_some(())
    .ok_or(())
}

fn qualification_name() -> Result<StagedNamespaceName, ()> {
    loop {
        let mut bytes = [0_u8; 16];
        getrandom::fill(&mut bytes).map_err(|_| ())?;
        if let Some(attempt) = NamespaceInitializationAttempt::from_nonzero_bytes(bytes) {
            return Ok(StagedNamespaceName::for_identity(attempt));
        }
    }
}
