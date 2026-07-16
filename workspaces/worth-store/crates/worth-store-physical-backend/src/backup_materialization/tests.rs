use sha2::{Digest, Sha256};

use super::{
    observe_physical_backup_artifact, PhysicalBackupMaterializationCounterScope,
    PhysicalBackupMaterializationSession, PhysicalBackupSource,
};

#[test]
fn crash_resume_revalidates_prefix_and_converges_to_source_bytes() {
    let directory = tempfile::tempdir().expect("temp directory");
    let source_path = directory.path().join("source.bin");
    let target = directory.path().join("target");
    let bytes = (0..=255).cycle().take(4097).collect::<Vec<u8>>();
    std::fs::write(&source_path, &bytes).expect("source");
    let source = || {
        let physical_identity = observe_physical_backup_artifact(&source_path, 127)
            .expect("source observation")
            .physical_identity();
        PhysicalBackupSource::new(
            &source_path,
            "artifact.bin",
            bytes.len() as u64,
            Sha256::digest(&bytes).into(),
            physical_identity,
        )
        .expect("source declaration")
    };
    let mut interrupted = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "resume-case",
        [source()],
        127,
    )
    .expect("session");
    for _ in 0..5 {
        assert!(interrupted.advance().expect("advance"));
    }
    drop(interrupted);

    let mut resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "resume-case",
        [source()],
        127,
    )
    .expect("resume");
    let resumed_open = resumed.counters();
    assert_eq!(resumed_open.source_bytes_read(), 635);
    assert_eq!(resumed_open.output_bytes_written(), 0);
    assert_eq!(resumed_open.resume_validation_bytes(), 1_270);
    assert_eq!(resumed_open.resumed_artifacts(), 1);
    assert_eq!(resumed_open.resumed_bytes(), 635);
    assert_eq!(resumed_open.rollback_bytes(), 0);
    assert_eq!(resumed_open.resumed_sessions(), 1);
    assert_eq!(resumed_open.sync_operations(), 0);
    assert_eq!(resumed_open.peak_buffer_bytes(), 127);
    assert_eq!(
        resumed_open.scope(),
        PhysicalBackupMaterializationCounterScope::CurrentRecoveredExecution
    );
    while resumed.advance().expect("advance resumed") {}
    let bundle = resumed.publish(b"manifest").expect("publish");
    let counters = bundle.counters();
    assert_eq!(counters.source_bytes_read(), bytes.len() as u64);
    assert_eq!(counters.output_bytes_written(), bytes.len() as u64 - 635);
    assert_eq!(
        counters.logically_materialized_bytes(),
        Some(bytes.len() as u64)
    );
    assert_eq!(counters.manifest_bytes_written(), 8);
    assert_eq!(counters.sync_operations(), 4);
    assert_eq!(counters.artifact_sync_operations(), 1);
    assert_eq!(
        std::fs::read(bundle.root().join("artifact.bin")).expect("published bytes"),
        bytes
    );
}

#[test]
fn corrupt_partial_output_rolls_back_to_a_verified_prefix_and_converges() {
    let directory = tempfile::tempdir().expect("temp directory");
    let source_path = directory.path().join("source.bin");
    let target = directory.path().join("target");
    let bytes = vec![3u8; 1024];
    std::fs::write(&source_path, &bytes).expect("source");
    let source = || {
        let physical_identity = observe_physical_backup_artifact(&source_path, 64)
            .expect("source observation")
            .physical_identity();
        PhysicalBackupSource::new(
            &source_path,
            "artifact.bin",
            bytes.len() as u64,
            Sha256::digest(&bytes).into(),
            physical_identity,
        )
        .expect("source declaration")
    };
    let mut interrupted = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "corrupt-resume",
        [source()],
        64,
    )
    .expect("session");
    interrupted.advance().expect("first chunk");
    drop(interrupted);
    std::fs::write(
        target
            .join(".incomplete-corrupt-resume")
            .join("artifact.bin"),
        vec![9u8; 64],
    )
    .expect("substitute prefix");
    let mut resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "corrupt-resume",
        [source()],
        64,
    )
    .expect("safe rollback to verified prefix");
    let resumed_open = resumed.counters();
    assert_eq!(resumed_open.source_bytes_read(), 64);
    assert_eq!(resumed_open.output_bytes_written(), 0);
    assert_eq!(resumed_open.resume_validation_bytes(), 128);
    assert_eq!(resumed_open.resumed_artifacts(), 0);
    assert_eq!(resumed_open.resumed_bytes(), 0);
    assert_eq!(resumed_open.rollback_bytes(), 64);
    assert_eq!(resumed_open.resumed_sessions(), 1);
    assert_eq!(resumed_open.sync_operations(), 1);
    while resumed.advance().expect("resume") {}
    let bundle = resumed.publish(b"manifest").expect("publish");
    let counters = bundle.counters();
    assert_eq!(counters.source_bytes_read(), bytes.len() as u64 + 64);
    assert_eq!(counters.output_bytes_written(), bytes.len() as u64);
    assert_eq!(counters.manifest_bytes_written(), 8);
    assert_eq!(counters.rollback_bytes(), 64);
    assert_eq!(counters.sync_operations(), 5);
    assert_eq!(counters.artifact_sync_operations(), 1);
    assert_eq!(
        std::fs::read(bundle.root().join("artifact.bin")).expect("published bytes"),
        bytes,
    );
}

#[test]
fn large_bundle_streaming_keeps_memory_fixed_and_syncs_per_artifact_not_chunk() {
    const ARTIFACTS: usize = 4;
    const ARTIFACT_BYTES: usize = 16 * 1024 * 1024;
    const BUFFER_BYTES: usize = 64 * 1024;

    let directory = tempfile::tempdir().expect("temp directory");
    let target = directory.path().join("target");
    let content = vec![0x5a; ARTIFACT_BYTES];
    let digest: [u8; 32] = Sha256::digest(&content).into();
    let mut sources = Vec::new();
    for index in 0..ARTIFACTS {
        let path = directory.path().join(format!("source-{index}.bin"));
        std::fs::write(&path, &content).expect("write source");
        let physical_identity = observe_physical_backup_artifact(&path, BUFFER_BYTES)
            .expect("source observation")
            .physical_identity();
        sources.push(
            PhysicalBackupSource::new(
                path,
                format!("artifact-{index}.bin"),
                ARTIFACT_BYTES as u64,
                digest,
                physical_identity,
            )
            .expect("source declaration"),
        );
    }

    let mut session = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "large-stream",
        sources,
        BUFFER_BYTES,
    )
    .expect("large streaming session");
    while session.advance().expect("stream chunk") {}
    let bundle = session.publish(b"manifest").expect("publish bundle");
    let counters = bundle.counters();
    let source_bytes = (ARTIFACTS * ARTIFACT_BYTES) as u64;

    assert_eq!(counters.source_bytes_read(), source_bytes);
    assert_eq!(counters.output_bytes_written(), source_bytes);
    assert_eq!(counters.manifest_bytes_written(), 8);
    assert_eq!(counters.peak_buffer_bytes(), BUFFER_BYTES as u64);
    assert_eq!(counters.sync_operations(), ARTIFACTS as u64 + 5);
    assert_eq!(counters.artifact_sync_operations(), ARTIFACTS as u64);
    assert_eq!(counters.resume_validation_bytes(), 0);
    assert_eq!(counters.resumed_artifacts(), 0);
    assert_eq!(counters.resumed_bytes(), 0);
    assert_eq!(counters.rollback_bytes(), 0);
    assert_eq!(counters.resumed_sessions(), 0);
    assert_eq!(counters.logically_materialized_bytes(), Some(source_bytes));
    assert_eq!(
        counters.scope(),
        PhysicalBackupMaterializationCounterScope::CompleteUninterruptedExecution
    );
}

#[test]
fn fresh_process_crash_and_unsynced_tail_loss_resume_from_verified_bytes() {
    const CHILD_SOURCE: &str = "WORTH_STORE_BACKUP_CRASH_SOURCE";
    const CHILD_TARGET: &str = "WORTH_STORE_BACKUP_CRASH_TARGET";
    const CHILD_EXIT: i32 = 73;

    if let (Some(source_path), Some(target)) = (
        std::env::var_os(CHILD_SOURCE),
        std::env::var_os(CHILD_TARGET),
    ) {
        let source_path = std::path::PathBuf::from(source_path);
        let bytes = std::fs::read(&source_path).expect("child source bytes");
        let physical_identity = observe_physical_backup_artifact(&source_path, 127)
            .expect("child source observation")
            .physical_identity();
        let source = PhysicalBackupSource::new(
            source_path,
            "artifact.bin",
            bytes.len() as u64,
            Sha256::digest(bytes).into(),
            physical_identity,
        )
        .expect("child source declaration");
        let mut session = PhysicalBackupMaterializationSession::open_or_resume(
            target,
            "process-crash",
            [source],
            127,
        )
        .expect("child session");
        for _ in 0..5 {
            assert!(session.advance().expect("child advance"));
        }
        std::process::exit(CHILD_EXIT);
    }

    let directory = tempfile::tempdir().expect("temp directory");
    let source_path = directory.path().join("source.bin");
    let target = directory.path().join("target");
    let bytes = (0..=255).cycle().take(4097).collect::<Vec<_>>();
    std::fs::write(&source_path, &bytes).expect("source bytes");
    let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .arg("--exact")
        .arg(
            "backup_materialization::tests::fresh_process_crash_and_unsynced_tail_loss_resume_from_verified_bytes",
        )
        .arg("--nocapture")
        .env(CHILD_SOURCE, &source_path)
        .env(CHILD_TARGET, &target)
        .status()
        .expect("crashing child process");
    assert_eq!(status.code(), Some(CHILD_EXIT));

    let partial = target
        .join(".incomplete-process-crash")
        .join("artifact.bin");
    std::fs::OpenOptions::new()
        .write(true)
        .open(&partial)
        .expect("open partial output")
        .set_len(173)
        .expect("simulate loss of unsynced tail");
    let physical_identity = observe_physical_backup_artifact(&source_path, 127)
        .expect("source observation")
        .physical_identity();
    let source = PhysicalBackupSource::new(
        &source_path,
        "artifact.bin",
        bytes.len() as u64,
        Sha256::digest(&bytes).into(),
        physical_identity,
    )
    .expect("source declaration");
    let mut resumed = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "process-crash",
        [source],
        127,
    )
    .expect("resume after process crash");
    let resumed_open = resumed.counters();
    assert_eq!(resumed_open.source_bytes_read(), 173);
    assert_eq!(resumed_open.output_bytes_written(), 0);
    assert_eq!(resumed_open.resume_validation_bytes(), 346);
    assert_eq!(resumed_open.resumed_artifacts(), 1);
    assert_eq!(resumed_open.resumed_bytes(), 173);
    assert_eq!(resumed_open.rollback_bytes(), 0);
    assert_eq!(resumed_open.resumed_sessions(), 1);
    assert_eq!(resumed_open.sync_operations(), 0);
    while resumed.advance().expect("resume chunk") {}
    let bundle = resumed
        .publish(b"manifest")
        .expect("publish resumed bundle");
    let counters = bundle.counters();
    assert_eq!(counters.source_bytes_read(), bytes.len() as u64);
    assert_eq!(counters.output_bytes_written(), bytes.len() as u64 - 173);
    assert_eq!(
        counters.logically_materialized_bytes(),
        Some(bytes.len() as u64)
    );
    assert_eq!(counters.manifest_bytes_written(), 8);
    assert_eq!(counters.sync_operations(), 4);
    assert_eq!(counters.artifact_sync_operations(), 1);
    assert_eq!(
        std::fs::read(bundle.root().join("artifact.bin")).expect("published artifact"),
        bytes
    );
}
