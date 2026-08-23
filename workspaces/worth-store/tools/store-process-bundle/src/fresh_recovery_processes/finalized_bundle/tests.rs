use std::path::Path;

use super::{combine_errors, directory, promotion};
use crate::fresh_recovery_processes::{artifact_binding, targets::WriterProcessRole};

#[test]
fn promotion_rebinds_a_role_typed_executable_after_copy() {
    let temporary = tempfile::tempdir().unwrap();
    let source = temporary.path().join("source.exe");
    std::fs::write(&source, b"source executable").unwrap();
    let artifact = artifact_binding::test_bound::<WriterProcessRole>(source);
    let bundle = temporary.path().join("bundle");
    std::fs::create_dir(&bundle).unwrap();

    let promoted = promotion::promote_artifact(&artifact, &bundle, "writer").unwrap();

    assert_eq!(promoted.digest(), artifact.digest());
    assert_ne!(promoted.path(), artifact.path());
    assert!(promoted.path().is_file());
}

#[test]
fn digest_mismatch_is_rejected_before_a_promoted_artifact_is_bound() {
    let temporary = tempfile::tempdir().unwrap();
    let source = temporary.path().join("source.exe");
    let destination = temporary.path().join("destination.exe");
    std::fs::write(&source, b"source executable").unwrap();
    std::fs::write(&destination, b"different executable").unwrap();
    let artifact = artifact_binding::test_bound::<WriterProcessRole>(source);

    let error = artifact.rebind_promoted(destination).err().unwrap();

    assert!(
        error.contains("promoted executable digest changed"),
        "{error}"
    );
}

#[test]
fn partial_bundle_rollback_removes_sealed_directory() {
    let temporary = tempfile::tempdir().unwrap();
    let bundle = temporary.path().join("partial-bundle");
    std::fs::create_dir(&bundle).unwrap();
    std::fs::write(bundle.join("writer.exe"), b"partial").unwrap();
    directory::seal(&bundle).unwrap();

    directory::remove(&bundle).unwrap();

    assert!(!bundle.exists());
}

#[test]
fn writable_rollback_helper_is_idempotent_for_partial_directories() {
    let temporary = tempfile::tempdir().unwrap();
    let bundle = temporary.path().join("partial-bundle");
    std::fs::create_dir(&bundle).unwrap();
    std::fs::write(bundle.join("writer.exe"), b"partial").unwrap();
    directory::seal(&bundle).unwrap();

    directory::make_writable(&bundle).unwrap();
    directory::make_writable(&bundle).unwrap();

    assert!(Path::new(&bundle).is_dir());
}

#[test]
fn cleanup_errors_are_retained_with_the_primary_failure() {
    let error = combine_errors(
        "build failed".to_owned(),
        Err("scratch close failed".to_owned()),
    );
    assert_eq!(error, "build failed; cleanup failed: scratch close failed");
}
