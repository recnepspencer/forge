use super::super::*;
use worth_proof::TransitionOutcome;
use worth_store_physical_format::store_namespace::{
    NamespaceInitializationAttempt, StagedNamespaceName,
};

fn fixed_name(byte: u8) -> StagedNamespaceName {
    let attempt = NamespaceInitializationAttempt::from_nonzero_bytes([byte; 16]).unwrap();
    StagedNamespaceName::for_identity(attempt)
}

#[test]
fn schedules_reject_ambiguous_or_semantically_impossible_matches() {
    let zero = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        0,
        MediaFaultDirective::AllowPrefix { bytes: 1 },
    )]);
    assert!(matches!(zero, Err(MediaFaultScheduleDenial::ZeroOrdinal)));

    let mismatch = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::ReadMetadata,
        1,
        MediaFaultDirective::AllowPrefix { bytes: 1 },
    )]);
    assert!(matches!(
        mismatch,
        Err(MediaFaultScheduleDenial::DirectiveRoleMismatch)
    ));

    let rule = || {
        MediaFaultRule::for_certification(
            MediaOperationRole::PositionedRead,
            1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
        )
    };
    let duplicate = MediaFaultSchedule::for_certification(vec![rule(), rule()]);
    assert!(matches!(
        duplicate,
        Err(MediaFaultScheduleDenial::DuplicateSemanticMatch)
    ));
}

fn qualified(root: &std::path::Path, schedule: MediaFaultSchedule) -> QualifiedFilesystemMedia {
    let request = FilesystemQualificationRequest::certification(
        root,
        FilesystemAccessPosture::CoordinatedServiceAccount,
    )
    .with_fault_schedule(schedule);
    match FilesystemMediaOwner::qualify(request).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("qualification failed"),
    }
}

#[test]
fn internal_partial_transfer_continuation_is_counted_as_a_retry() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("partial-publication");
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        1,
        MediaFaultDirective::AllowPrefix { bytes: 3 },
    )])
    .unwrap();
    let media = qualified(&root, schedule);
    let counters = media.counters();
    assert_eq!(counters.retry_attempts(), 1);
    assert_eq!(
        counters.partial_effects_for(MediaOperationRole::PositionedWrite),
        1
    );
    assert!(counters.is_conserved());
    media.close();
}

fn initialize(root: &std::path::Path) {
    let request = FilesystemQualificationRequest::production(
        root,
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    let media = match FilesystemMediaOwner::qualify(request).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("initialization must succeed"),
    };
    media.close();
}

fn visible_files(root: &std::path::Path) -> Vec<(std::path::PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    for directory in [
        root.to_owned(),
        root.join("namespace"),
        root.join("families"),
        root.join("staging"),
    ] {
        for entry in std::fs::read_dir(&directory).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() && entry.file_name() != "mutation.lock" {
                files.push((
                    entry.path().strip_prefix(root).unwrap().to_owned(),
                    std::fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

#[test]
fn empty_schedule_is_a_bitwise_and_counter_pass_through() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    initialize(&root);

    let plain = qualified(&root, MediaFaultSchedule::default());
    let plain_counters = plain.counters();
    let plain_files = visible_files(&root);
    plain.close();

    let decorated = qualified(
        &root,
        MediaFaultSchedule::for_certification(Vec::new()).unwrap(),
    );
    assert_eq!(decorated.counters(), plain_counters);
    assert_eq!(visible_files(&root), plain_files);
    decorated.close();
}

fn prefix_program(root: &std::path::Path) -> (MediaCounterSnapshot, Vec<u8>) {
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        1,
        MediaFaultDirective::AllowPrefix { bytes: 3 },
    )])
    .unwrap();
    let owner = FilesystemMediaOwner::admit_with_schedule(root, schedule).unwrap();
    let path = owner.staged_identity_path(&fixed_name(7));
    let handle = match owner.create_new(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        _ => panic!("create must succeed"),
    };
    let outcome = handle.positioned_write(PositionedWriteRequest::new(0, b"abcdef"));
    assert!(matches!(
        outcome.result(),
        MediaOperationResult::Failed(failure)
            if matches!(failure.kind(), MediaOperationFailureKind::PartialTransfer(_))
    ));
    drop(handle);
    let bytes = std::fs::read(root.join("namespace").join(fixed_name(7).as_str())).unwrap();
    let counters = owner.counters();
    owner.close();
    (counters, bytes)
}

#[test]
fn semantic_schedule_is_deterministic_and_conserved() {
    let parent = tempfile::tempdir().unwrap();
    let left = prefix_program(&parent.path().join("left"));
    let right = prefix_program(&parent.path().join("right"));
    assert!(left.0.same_counter_values(right.0));
    assert_eq!(left.1, right.1);
    let left_match = left.0.first_fault_match().unwrap();
    let right_match = right.0.first_fault_match().unwrap();
    assert_eq!(left_match.role(), right_match.role());
    assert_eq!(left_match.role_ordinal(), right_match.role_ordinal());
    assert_eq!(left_match.operation(), right_match.operation());
    assert_eq!(
        left_match.handle().unwrap().generation(),
        right_match.handle().unwrap().generation()
    );
    assert_eq!(
        left.0.first_fault_terminal(),
        right.0.first_fault_terminal()
    );
    assert_eq!(left.1, b"abc");
    assert!(left.0.is_conserved());
    assert_eq!(
        left.0
            .requested_bytes_for(MediaOperationRole::PositionedWrite),
        6
    );
    assert_eq!(
        left.0
            .completed_bytes_for(MediaOperationRole::PositionedWrite),
        3
    );
    assert_eq!(left.0.partial_effects(), 1);
    assert_eq!(left.0.first_fault_match().unwrap().requested_bytes(), 6);
    assert_eq!(left.0.first_fault_completed_bytes(), Some(3));
    assert_eq!(left.0.live_file_handles(), 0);
    assert_eq!(left.0.peak_file_handles(), 1);
}

#[test]
fn post_effect_fault_preserves_bytes_and_reports_indeterminate_once() {
    let parent = tempfile::tempdir().unwrap();
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        1,
        MediaFaultDirective::IndeterminateAfterEffect,
    )])
    .unwrap();
    let owner = FilesystemMediaOwner::admit_with_schedule(parent.path(), schedule).unwrap();
    let path = owner.staged_identity_path(&fixed_name(9));
    let handle = match owner.create_new(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        _ => panic!("create must succeed"),
    };
    let outcome = handle.positioned_write(PositionedWriteRequest::new(0, b"real-bytes"));
    assert!(matches!(
        outcome.result(),
        MediaOperationResult::Failed(failure)
            if matches!(failure.kind(), MediaOperationFailureKind::IndeterminateEffect { .. })
    ));
    assert_eq!(
        std::fs::read(parent.path().join("namespace").join(fixed_name(9).as_str())).unwrap(),
        b"real-bytes"
    );
    let counters = owner.counters();
    assert!(counters.is_conserved());
    assert_eq!(counters.indeterminate_effects(), 1);
    assert_eq!(counters.first_fault_match().unwrap().requested_bytes(), 10);
    assert_eq!(counters.first_fault_completed_bytes(), Some(10));
    owner.close();
}

#[test]
fn barrier_fault_denies_before_the_real_sync_call() {
    let parent = tempfile::tempdir().unwrap();
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::SynchronizeFileState,
        1,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    )])
    .unwrap();
    let owner = FilesystemMediaOwner::admit_with_schedule(parent.path(), schedule).unwrap();
    let path = owner.staged_identity_path(&fixed_name(11));
    let handle = match owner.create_new(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        _ => panic!("create must succeed"),
    };
    let result = handle.synchronize_state();
    assert!(matches!(
        result,
        FileStateSynchronizationOutcome::Failed(failure)
            if failure.kind() == MediaOperationFailureKind::DeniedBeforeEffect
    ));
    let counters = owner.counters();
    assert_eq!(counters.file_syncs(), 0);
    assert_eq!(counters.first_fault_match().unwrap().requested_bytes(), 0);
    assert_eq!(counters.first_fault_completed_bytes(), Some(0));
    assert_eq!(
        counters.attempts_for(MediaOperationRole::SynchronizeFileState),
        1
    );
    assert!(counters.is_conserved());
    drop(handle);
    owner.close();
}

#[test]
fn failed_containing_directory_barriers_are_reestablished_on_reopen() {
    for role in [
        MediaOperationRole::SynchronizeStoreRootPublication,
        MediaOperationRole::SynchronizeRootParentPublication,
    ] {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join(role.metric_name());
        let schedule =
            MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
                role,
                1,
                MediaFaultDirective::FailBarrier {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
            )])
            .unwrap();
        let request = FilesystemQualificationRequest::production(
            &root,
            FilesystemAccessPosture::CoordinatedServiceAccount,
        )
        .with_fault_schedule(schedule);
        assert!(matches!(
            FilesystemMediaOwner::qualify(request).into_raw(),
            worth_proof::TransitionOutcome::Failed(
                MediaQualificationFailure::IdentityPublication { .. }
            )
        ));
        assert!(root.join("namespace/identity").is_file());

        let reopened = FilesystemMediaOwner::qualify(FilesystemQualificationRequest::production(
            &root,
            FilesystemAccessPosture::CoordinatedServiceAccount,
        ))
        .into_raw();
        let worth_proof::TransitionOutcome::Success(reopened) = reopened else {
            panic!("reopen must reestablish every containing barrier");
        };
        assert_eq!(reopened.counters().attempts_for(role), 1);
        reopened.close();
    }
}

#[test]
fn replacement_observation_fault_never_manufactures_destination_bytes() {
    let parent = tempfile::tempdir().unwrap();
    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::AtomicReplace,
        1,
        MediaFaultDirective::InterruptReplacementObservation,
    )])
    .unwrap();
    let owner = FilesystemMediaOwner::admit_with_schedule(parent.path(), schedule).unwrap();
    let staged =
        match StagedNamespaceFile::create(&owner, owner.staged_identity_path(&fixed_name(13))) {
            StagedNamespaceFileOutcome::Created(staged) => staged,
            _ => panic!("staging create must succeed"),
        };
    let written = match staged.write_all(b"replacement-truth") {
        StagedNamespaceWriteOutcome::Completed(written) => written,
        _ => panic!("staging write must succeed"),
    };
    let synchronized = match written.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        _ => panic!("staging sync must succeed"),
    };
    let destination = fixed_name(14);
    assert!(matches!(
        synchronized.replace(owner.staged_publication_target(&destination)),
        AtomicReplacementOutcome::Indeterminate(_)
    ));
    assert_eq!(
        std::fs::read(parent.path().join("namespace").join(destination.as_str())).unwrap(),
        b"replacement-truth"
    );
    let counters = owner.counters();
    assert_eq!(counters.replacements(), 1);
    assert_eq!(counters.indeterminate_effects(), 1);
    assert!(counters.is_conserved());
    owner.close();
}
