use std::io::{Read, Write};

use sha2::{Digest, Sha256};

use super::{
    observe_physical_backup_artifact, PhysicalBackupMaterializationDenial,
    PhysicalBackupMaterializationSession, PhysicalBackupSource,
};

#[test]
fn oversized_session_identity_fails_before_target_allocation() {
    let directory = tempfile::tempdir().expect("directory");
    let source_path = directory.path().join("source.bin");
    let bytes = vec![0x31; 16];
    std::fs::write(&source_path, &bytes).expect("source");
    let target = directory.path().join("target");
    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        &"a".repeat(129),
        [source(&source_path, &bytes)],
        19,
    )
    .err()
    .expect("oversized identity must be rejected");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::InvalidSessionIdentity
    ));
    assert!(!target.exists());
}

#[test]
fn source_inside_reserved_staging_root_is_never_opened_as_materialization_input() {
    let directory = tempfile::tempdir().expect("directory");
    let target = directory.path().join("target");
    let staging = target.join(".incomplete-contained-source");
    std::fs::create_dir_all(&staging).expect("hostile staging tree");
    let source_path = staging.join("source.bin");
    let bytes = vec![0x52; 32];
    std::fs::write(&source_path, &bytes).expect("source");

    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "contained-source",
        [source(&source_path, &bytes)],
        19,
    )
    .err()
    .expect("session output cannot contain its own source");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::SourceInsideSessionOutput { .. }
    ));
    assert_eq!(std::fs::read(&source_path).expect("source survives"), bytes);
    assert!(!staging.join("materialization.session").exists());
}

#[test]
fn preexisting_output_hard_link_cannot_alias_and_overwrite_a_source() {
    let directory = tempfile::tempdir().expect("directory");
    let source_path = directory.path().join("source.bin");
    let bytes = vec![0x73; 32];
    std::fs::write(&source_path, &bytes).expect("source");
    let target = directory.path().join("target");
    let staging = target.join(".incomplete-hard-link-source");
    std::fs::create_dir_all(&staging).expect("hostile staging tree");
    std::fs::hard_link(&source_path, staging.join("artifact.bin")).expect("source hard link");

    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "hard-link-source",
        [source(&source_path, &bytes)],
        19,
    )
    .err()
    .expect("output hard link cannot alias input media");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::OutputAliasesMaterializationFile { .. }
    ));
    assert_eq!(std::fs::read(&source_path).expect("source survives"), bytes);
    assert!(!staging.join("materialization.session").exists());
}

#[test]
fn foreign_staging_residue_cannot_be_renamed_into_a_completed_bundle() {
    let directory = tempfile::tempdir().expect("directory");
    let source_path = directory.path().join("source.bin");
    let bytes = vec![0x84; 32];
    std::fs::write(&source_path, &bytes).expect("source");
    let target = directory.path().join("target");
    let staging = target.join(".incomplete-foreign-residue");
    std::fs::create_dir_all(&staging).expect("hostile staging tree");
    let residue = staging.join("foreign.bin");
    std::fs::write(&residue, b"foreign").expect("foreign residue");

    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "foreign-residue",
        [source(&source_path, &bytes)],
        19,
    )
    .err()
    .expect("foreign staging media must fail closed");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::UnexpectedStagingEntry { .. }
    ));
    assert_eq!(
        std::fs::read(residue).expect("residue untouched"),
        b"foreign"
    );
    assert!(!staging.join("materialization.session").exists());
}

#[test]
fn reused_session_identity_rejects_a_different_source_set_without_touching_progress() {
    let directory = tempfile::tempdir().expect("directory");
    let first_path = directory.path().join("first.bin");
    let second_path = directory.path().join("second.bin");
    let first_bytes = vec![0x31; 256];
    let second_bytes = vec![0x72; 256];
    std::fs::write(&first_path, &first_bytes).expect("first source");
    std::fs::write(&second_path, &second_bytes).expect("second source");
    let target = directory.path().join("target");

    let mut first = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "content-bound-session",
        [source(&first_path, &first_bytes)],
        17,
    )
    .expect("first session");
    first.advance().expect("first chunk");
    drop(first);
    let progress_path = target
        .join(".incomplete-content-bound-session")
        .join("artifact.bin");
    let progress_before = std::fs::read(&progress_path).expect("partial output");

    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "content-bound-session",
        [source(&second_path, &second_bytes)],
        17,
    )
    .err()
    .expect("same label cannot name a different physical source set");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::SessionIdentityMismatch { .. }
    ));
    assert_eq!(
        std::fs::read(progress_path).expect("unchanged partial output"),
        progress_before
    );
}

#[test]
fn independent_process_cannot_open_an_active_materialization_session() {
    const CHILD_TARGET: &str = "WORTH_STORE_ACTIVE_SESSION_TARGET";
    const CHILD_SOURCE: &str = "WORTH_STORE_ACTIVE_SESSION_SOURCE";
    const CHILD_READY: &str = "WORTH_STORE_ACTIVE_SESSION_READY";

    if let (Some(target), Some(source_path), Some(ready)) = (
        std::env::var_os(CHILD_TARGET),
        std::env::var_os(CHILD_SOURCE),
        std::env::var_os(CHILD_READY),
    ) {
        let source_path = std::path::PathBuf::from(source_path);
        let bytes = std::fs::read(&source_path).expect("child source");
        let _session = PhysicalBackupMaterializationSession::open_or_resume(
            target,
            "active-session",
            [source(&source_path, &bytes)],
            19,
        )
        .expect("child owns session lock");
        std::fs::write(ready, b"ready").expect("signal acquired lock");
        let mut release = [0_u8; 1];
        std::io::stdin()
            .read_exact(&mut release)
            .expect("parent releases child");
        return;
    }

    let directory = tempfile::tempdir().expect("directory");
    let source_path = directory.path().join("source.bin");
    let bytes = vec![0x5a; 128];
    std::fs::write(&source_path, &bytes).expect("source");
    let target = directory.path().join("target");
    let ready = directory.path().join("ready");
    let mut child = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .arg("--exact")
        .arg("backup_materialization::session_identity_tests::independent_process_cannot_open_an_active_materialization_session")
        .arg("--nocapture")
        .env(CHILD_TARGET, &target)
        .env(CHILD_SOURCE, &source_path)
        .env(CHILD_READY, &ready)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("session owner child");
    wait_for_ready(&ready, &mut child);

    let denial = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "active-session",
        [source(&source_path, &bytes)],
        19,
    )
    .err()
    .expect("second process cannot share staging ownership");
    assert!(matches!(
        denial,
        PhysicalBackupMaterializationDenial::SessionBusy { .. }
    ));
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(b"x")
        .expect("release child");
    assert!(child.wait().expect("child exit").success());
}

#[test]
fn distinct_session_identities_make_concurrent_progress_under_one_target_parent() {
    let directory = tempfile::tempdir().expect("directory");
    let first_path = directory.path().join("first.bin");
    let second_path = directory.path().join("second.bin");
    let first_bytes = vec![0x41; 257];
    let second_bytes = vec![0x82; 263];
    std::fs::write(&first_path, &first_bytes).expect("first source");
    std::fs::write(&second_path, &second_bytes).expect("second source");
    let target = directory.path().join("target");

    let mut first = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "concurrent-first",
        [source(&first_path, &first_bytes)],
        19,
    )
    .expect("first session");
    let mut second = PhysicalBackupMaterializationSession::open_or_resume(
        &target,
        "concurrent-second",
        [source(&second_path, &second_bytes)],
        19,
    )
    .expect("a distinct identity must not contend on a global lock");

    loop {
        let first_progress = first.advance().expect("first progress");
        let second_progress = second.advance().expect("second progress");
        if !first_progress && !second_progress {
            break;
        }
    }
    let first = first.publish(b"manifest-first").expect("first publish");
    let second = second.publish(b"manifest-second").expect("second publish");
    assert_eq!(
        std::fs::read(first.root().join("artifact.bin")).expect("first output"),
        first_bytes
    );
    assert_eq!(
        std::fs::read(second.root().join("artifact.bin")).expect("second output"),
        second_bytes
    );
}

fn source(path: &std::path::Path, bytes: &[u8]) -> PhysicalBackupSource {
    let identity = observe_physical_backup_artifact(path, 19)
        .expect("source observation")
        .physical_identity();
    PhysicalBackupSource::new(
        path,
        "artifact.bin",
        bytes.len() as u64,
        Sha256::digest(bytes).into(),
        identity,
    )
    .expect("source declaration")
}

fn wait_for_ready(path: &std::path::Path, child: &mut std::process::Child) {
    for _ in 0..500 {
        if path.exists() {
            return;
        }
        assert!(child.try_wait().expect("child status").is_none());
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    panic!("child did not acquire the materialization lock");
}
