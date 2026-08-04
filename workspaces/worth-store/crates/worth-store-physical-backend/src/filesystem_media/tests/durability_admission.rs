use super::super::*;
use worth_proof::TransitionOutcome;

#[cfg(feature = "store-runtime-owner")]
#[test]
fn basis_is_stable_for_one_media_and_distinct_across_roots() {
    let parent = tempfile::tempdir().unwrap();
    let first = qualified(&parent.path().join("first"));
    let first_basis = first.physical_durability_admission_basis().unwrap();
    let repeated = first.physical_durability_admission_basis().unwrap();
    assert_eq!(first_basis.identity(), repeated.identity());
    assert_eq!(first_basis.store_identity(), first.store_identity());
    assert_eq!(
        first_basis.file_sync_claim().kind(),
        crate::BackendCapabilityKind::Fsync
    );
    assert_eq!(
        first_basis.directory_sync_claim().kind(),
        crate::BackendCapabilityKind::DirectorySync
    );
    assert_eq!(
        first_basis.durable_rename_claim().kind(),
        crate::BackendCapabilityKind::DurableRename
    );

    let second = qualified(&parent.path().join("second"));
    let second_basis = second.physical_durability_admission_basis().unwrap();
    assert_ne!(first_basis.identity(), second_basis.identity());
    assert_ne!(first_basis.store_identity(), second_basis.store_identity());
    first.close();
    second.close();
}

fn qualified(root: &std::path::Path) -> QualifiedFilesystemMedia {
    let request = FilesystemQualificationRequest::production(
        root,
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    match FilesystemMediaOwner::qualify(request).into_raw() {
        TransitionOutcome::Success(qualified) => qualified,
        TransitionOutcome::Denied(value) => panic!("root qualification denied: {value:?}"),
        TransitionOutcome::Deferred(value) => panic!("root qualification deferred: {value:?}"),
        TransitionOutcome::Stale(value) => panic!("root qualification stale: {value:?}"),
        TransitionOutcome::RebindRequired(value) => panic!("root qualification rebind: {value:?}"),
        TransitionOutcome::Failed(value) => panic!("root qualification failed: {value:?}"),
    }
}
