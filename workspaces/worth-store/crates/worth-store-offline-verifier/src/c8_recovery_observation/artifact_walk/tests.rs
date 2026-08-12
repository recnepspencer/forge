use super::*;

#[test]
fn directory_entry_budget_is_exact_and_crossing_entries_are_not_admitted() {
    let root = two_file_root();
    let exact = walk(
        root.path(),
        RecoveryObserverLimits::new(2, 1, 2, 2).unwrap(),
    )
    .unwrap();
    assert_eq!(exact.counters().directory_entries_observed(), 2);
    assert_eq!(exact.counters().artifacts_observed(), 2);

    let failure = walk(
        root.path(),
        RecoveryObserverLimits::new(1, 1, 2, 2).unwrap(),
    )
    .unwrap_err();
    assert_eq!(
        failure.denial(),
        RecoveryObserverObservationDenial::DirectoryEntryLimit {
            observed: 2,
            admitted: 1,
        }
    );
    assert_eq!(failure.counters().directory_entries_observed(), 2);
    assert_eq!(failure.counters().artifacts_admitted(), 0);
    assert_eq!(failure.counters().files_opened(), 0);
}

#[test]
fn directory_budget_rejects_the_crossing_directory_before_opening_it() {
    let root = tempfile::tempdir().unwrap();
    for name in ["a", "b"] {
        let directory = root.path().join(name);
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("page"), name.as_bytes()).unwrap();
    }
    let exact = walk(
        root.path(),
        RecoveryObserverLimits::new(4, 3, 2, 2).unwrap(),
    )
    .unwrap();
    assert_eq!(exact.counters().directories_opened(), 3);

    let failure = walk(
        root.path(),
        RecoveryObserverLimits::new(4, 2, 2, 2).unwrap(),
    )
    .unwrap_err();
    assert_eq!(
        failure.denial(),
        RecoveryObserverObservationDenial::DirectoryLimit {
            observed: 3,
            admitted: 2,
        }
    );
    assert_eq!(failure.counters().directories_opened(), 1);
    assert_eq!(failure.counters().files_opened(), 0);
}

#[test]
fn artifact_budget_rejects_the_crossing_file_before_opening_it() {
    let root = two_file_root();
    let failure = walk(
        root.path(),
        RecoveryObserverLimits::new(2, 1, 1, 2).unwrap(),
    )
    .unwrap_err();
    assert_eq!(
        failure.denial(),
        RecoveryObserverObservationDenial::ArtifactLimit {
            observed: 2,
            admitted: 1,
        }
    );
    assert_eq!(failure.counters().artifacts_observed(), 1);
    assert_eq!(failure.counters().files_opened(), 1);
    assert_eq!(failure.counters().bytes_read(), 1);
}

#[test]
fn byte_budget_is_exact_and_rejects_before_opening_the_crossing_file() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("a"), b"abc").unwrap();
    std::fs::write(root.path().join("b"), b"de").unwrap();
    let exact = walk(
        root.path(),
        RecoveryObserverLimits::new(2, 1, 2, 5).unwrap(),
    )
    .unwrap();
    assert_eq!(exact.counters().bytes_read(), 5);

    let failure = walk(
        root.path(),
        RecoveryObserverLimits::new(2, 1, 2, 4).unwrap(),
    )
    .unwrap_err();
    assert_eq!(
        failure.denial(),
        RecoveryObserverObservationDenial::ByteLimit {
            observed: 5,
            admitted: 4,
        }
    );
    assert_eq!(failure.counters().files_opened(), 1);
    assert_eq!(failure.counters().bytes_read(), 2);
}

fn two_file_root() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("a"), b"a").unwrap();
    std::fs::write(root.path().join("b"), b"b").unwrap();
    root
}
