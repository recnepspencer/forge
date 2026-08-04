use super::super::*;
use worth_proof::TransitionOutcome;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

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

fn root_manifest_artifact(
    media: &QualifiedFilesystemMedia,
    generation: u64,
) -> (RecordArtifactFile, ArtifactTreeFile) {
    let tree = media.artifact_tree();
    let records = ArtifactTreeDirectory::families().child("records").unwrap();
    let roots = records.child("roots").unwrap();
    if !tree.directory_exists(&records).unwrap() {
        tree.create_directory(&records).unwrap();
    }
    if !tree.directory_exists(&roots).unwrap() {
        tree.create_directory(&roots).unwrap();
    }
    let logical = RecordArtifactFile::RootManifest { generation };
    let physical = roots.file(&logical.file_name()).unwrap();
    (logical, physical)
}

fn new_write_range(bytes: &[u8]) -> ArtifactNewWriteRange {
    ArtifactNewWriteRange::new(bytes.len() as u64).unwrap()
}

#[test]
fn new_artifact_write_retains_distinct_create_and_write_identities() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(
        &parent.path().join("complete"),
        MediaFaultSchedule::default(),
    );
    let (logical, physical) = root_manifest_artifact(&media, 1);
    let bytes = b"root-manifest";

    let completed =
        match media
            .artifact_tree()
            .write_new_exact(&physical, new_write_range(bytes), bytes)
        {
            ArtifactNewWriteOutcome::Completed(completed) => completed,
            outcome => panic!("new artifact write did not complete: {outcome:?}"),
        };

    assert_ne!(
        completed.create_operation(),
        completed.write_operation(),
        "create and exact write are separate backend effects"
    );
    assert_eq!(completed.artifact(), &physical);
    assert_eq!(completed.range(), new_write_range(bytes));
    assert_eq!(
        std::fs::read(
            parent
                .path()
                .join("complete/families/records/roots")
                .join(logical.file_name())
        )
        .unwrap(),
        bytes
    );
    media.close();
}

#[test]
fn admitted_artifact_roots_support_exact_files_without_synthetic_child_directories() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(
        &parent.path().join("root-files"),
        MediaFaultSchedule::default(),
    );
    let tree = media.artifact_tree();
    for (directory, relative) in [
        (ArtifactTreeDirectory::families(), "families/current"),
        (ArtifactTreeDirectory::staging(), "staging/candidate"),
    ] {
        let artifact = directory
            .file(relative.rsplit('/').next().unwrap())
            .unwrap();
        let bytes = relative.as_bytes();
        assert!(matches!(
            tree.write_new_exact(&artifact, new_write_range(bytes), bytes),
            ArtifactNewWriteOutcome::Completed(_)
        ));
        assert_eq!(
            std::fs::read(parent.path().join("root-files").join(relative)).unwrap(),
            bytes
        );
    }
    media.close();
}

#[test]
fn existing_new_artifact_target_is_denied_before_effect() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(
        &parent.path().join("existing"),
        MediaFaultSchedule::default(),
    );
    let (_logical, physical) = root_manifest_artifact(&media, 2);
    let bytes = b"unchanged";
    media.artifact_tree().write_new(&physical, bytes).unwrap();

    assert!(matches!(
        media
            .artifact_tree()
            .write_new_exact(&physical, new_write_range(bytes), bytes),
        ArtifactNewWriteOutcome::DeniedBeforeEffect(failure)
            if failure.kind() == ArtifactTreeFailureKind::AlreadyExists
    ));
    media.close();
}

#[test]
fn durable_artifact_truncation_shortens_only_an_existing_file() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(
        &parent.path().join("truncate"),
        MediaFaultSchedule::default(),
    );
    let (_logical, physical) = root_manifest_artifact(&media, 20);
    let tree = media.artifact_tree();
    tree.write_new(&physical, b"verified-prefix-interrupted-tail")
        .unwrap();

    tree.truncate_file_durably(&physical, 15).unwrap();
    assert_eq!(
        tree.read_bounded(&physical, 15).unwrap(),
        b"verified-prefix"
    );
    assert_eq!(
        tree.truncate_file_durably(&physical, 15)
            .unwrap_err()
            .kind(),
        ArtifactTreeFailureKind::AccessLimitExceeded,
    );
    media.close();
}

#[test]
fn denied_exact_write_after_create_is_indeterminate() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline = qualified(
        &baseline_parent.path().join("baseline"),
        MediaFaultSchedule::default(),
    );
    root_manifest_artifact(&baseline, 3);
    let write_ordinal = baseline
        .counters()
        .attempts_for(MediaOperationRole::PositionedWrite)
        + 1;
    baseline.close();

    let schedule = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        write_ordinal,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    )])
    .unwrap();
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(&parent.path().join("faulted"), schedule);
    let (logical, physical) = root_manifest_artifact(&media, 3);
    let bytes = b"never-written";

    let failure =
        match media
            .artifact_tree()
            .write_new_exact(&physical, new_write_range(bytes), bytes)
        {
            ArtifactNewWriteOutcome::Indeterminate(failure) => failure,
            outcome => panic!("post-create denial was not indeterminate: {outcome:?}"),
        };

    assert_eq!(failure.completed_bytes(), 0);
    assert_eq!(failure.write_operation(), None);
    assert_eq!(
        std::fs::metadata(
            parent
                .path()
                .join("faulted/families/records/roots")
                .join(logical.file_name())
        )
        .unwrap()
        .len(),
        0,
        "the create effect is retained even though the write was denied"
    );
    media.close();
}

#[test]
fn append_exact_at_eof_extends_without_weakening_exact_overwrite_bounds() {
    let parent = tempfile::tempdir().unwrap();
    let media = qualified(&parent.path().join("append"), MediaFaultSchedule::default());
    let (logical, physical) = root_manifest_artifact(&media, 4);
    let tree = media.artifact_tree();
    let initial = b"abc";
    assert!(matches!(
        tree.write_new_exact(&physical, new_write_range(initial), initial),
        ArtifactNewWriteOutcome::Completed(_)
    ));
    let continuation = RecordFrameCoordinate::new(logical, 3, 3).unwrap();
    assert!(matches!(
        tree.append_exact_at_eof(&physical, continuation, b"def"),
        ArtifactRangeWriteOutcome::Completed(_)
    ));
    assert!(matches!(
        tree.write_exact_at(
            &physical,
            RecordFrameCoordinate::new(logical, 6, 1).unwrap(),
            b"x"
        ),
        ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure)
            if failure.kind() == ArtifactTreeFailureKind::AccessLimitExceeded
    ));
    for offset in [2, 7] {
        assert!(matches!(
            tree.append_exact_at_eof(
                &physical,
                RecordFrameCoordinate::new(logical, offset, 1).unwrap(),
                b"x"
            ),
            ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure)
                if failure.kind() == ArtifactTreeFailureKind::AccessLimitExceeded
        ));
    }
    assert_eq!(
        std::fs::read(
            parent
                .path()
                .join("append/families/records/roots")
                .join(logical.file_name())
        )
        .unwrap(),
        b"abcdef"
    );
    media.close();
}
