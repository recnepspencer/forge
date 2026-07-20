use super::super::super::*;
use super::fixture::{created, owner, staged_path, synchronized_staged};
use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

#[test]
fn substituted_staged_name_cannot_publish_bytes_from_another_file() {
    let (root, owner) = owner();
    let path = staged_path(&owner, 25);
    let staged = match StagedNamespaceFile::create(&owner, path.clone()) {
        StagedNamespaceFileOutcome::Created(staged) => staged,
        other => panic!("stage file: {other:?}"),
    };
    let completed = match staged.write_all(b"synchronized-original") {
        StagedNamespaceWriteOutcome::Completed(completed) => completed,
        other => panic!("write stage: {other:?}"),
    };
    let synchronized = match completed.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        other => panic!("synchronize stage: {other:?}"),
    };
    let named = root.path().join("store").join(path.as_path());
    let displaced = named.with_extension("displaced");
    std::fs::rename(&named, &displaced).expect("displace synchronized file");
    std::fs::write(&named, b"unsynchronized-impostor").expect("install impostor");

    assert!(matches!(
        synchronized.replace(owner.identity_publication_target()),
        AtomicReplacementOutcome::Denied(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
    assert!(!root.path().join("store/namespace/identity").exists());
    assert_eq!(std::fs::read(displaced).unwrap(), b"synchronized-original");
    assert_eq!(std::fs::read(named).unwrap(), b"unsynchronized-impostor");
    assert_eq!(owner.counters().stale_handle_denials(), 1);
}

#[test]
fn substituted_deletion_name_cannot_be_removed_by_an_old_handle() {
    let (root, owner) = owner();
    let path = staged_path(&owner, 26);
    drop(created(owner.create_new(&path)));
    let handle = match owner.open_existing_for_mutation(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        other => panic!("open deletion capability: {other:?}"),
    };
    let named = root.path().join("store").join(path.as_path());
    let displaced = named.with_extension("displaced");
    std::fs::rename(&named, &displaced).expect("displace opened file");
    std::fs::write(&named, b"replacement").expect("install replacement");

    assert!(matches!(
        owner.delete_namespace_file(handle),
        NamespaceDeletionOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
    assert!(displaced.is_file());
    assert_eq!(std::fs::read(named).unwrap(), b"replacement");
    assert_eq!(owner.counters().stale_handle_denials(), 1);
}

#[test]
fn missing_staged_name_cannot_be_published_from_an_unlinked_handle() {
    let (root, owner) = owner();
    let path = staged_path(&owner, 30);
    let synchronized = synchronized_staged(&owner, 30, b"unlinked-original");
    std::fs::remove_file(root.path().join("store").join(path.as_path()))
        .expect("unlink synchronized stage");

    assert!(matches!(
        synchronized.replace(owner.identity_publication_target()),
        AtomicReplacementOutcome::Denied(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
                && failure.context().io_kind() == Some(std::io::ErrorKind::NotFound)
    ));
    assert!(!root.path().join("store/namespace/identity").exists());
}

#[test]
fn missing_deletion_name_is_a_before_effect_denial() {
    let (root, owner) = owner();
    let path = staged_path(&owner, 31);
    drop(created(owner.create_new(&path)));
    let handle = match owner.open_existing_for_mutation(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        other => panic!("open deletion capability: {other:?}"),
    };
    std::fs::remove_file(root.path().join("store").join(path.as_path()))
        .expect("unlink deletion target");

    assert!(matches!(
        owner.delete_namespace_file(handle),
        NamespaceDeletionOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
                && failure.context().io_kind() == Some(std::io::ErrorKind::NotFound)
    ));
}

#[test]
fn publication_target_from_another_owner_is_denied_before_effect() {
    let (_root_a, owner_a) = owner();
    let (_root_b, owner_b) = owner();
    let synchronized = synchronized_staged(&owner_a, 32, b"candidate");

    assert!(matches!(
        synchronized.replace(owner_b.identity_publication_target()),
        AtomicReplacementOutcome::Denied(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
}

#[test]
fn publication_requires_write_file_sync_rename_and_directory_sync() {
    let (root, owner) = owner();
    let staged = match StagedNamespaceFile::create(&owner, staged_path(&owner, 20)) {
        StagedNamespaceFileOutcome::Created(staged) => staged,
        other => panic!("staging creation failed: {other:?}"),
    };
    let completed = match staged.write_all(b"framed-identity-bytes") {
        StagedNamespaceWriteOutcome::Completed(completed) => completed,
        other => panic!("staged write failed: {other:?}"),
    };
    assert_eq!(completed.bytes(), 21);
    let synchronized = match completed.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        other => panic!("file sync failed: {other:?}"),
    };
    let replaced = match synchronized.replace(owner.identity_publication_target()) {
        AtomicReplacementOutcome::Replaced(replaced) => replaced,
        other => panic!("replacement failed: {other:?}"),
    };
    let published = match replaced.synchronize_publication() {
        DurableNamespacePublicationOutcome::Published(published) => published,
        other => panic!("directory sync failed: {other:?}"),
    };
    assert_eq!(
        published.destination().role(),
        MediaPathRole::Namespace(StoreNamespaceRelativeRole::IdentityRecord)
    );
    assert_eq!(
        published
            .store_root_synchronization()
            .expect("initial scaffold publication requires root synchronization")
            .handle(),
        owner.root_directory_handle().identity()
    );
    assert!(published.root_parent_synchronization().is_some());
    let summary = published.summary();
    let write = summary.write();
    let parent_sync = summary
        .root_parent_synchronization()
        .expect("absent-root publication requires parent synchronization");
    assert!(summary.create_operation() < write.first_operation());
    assert!(write.first_operation() <= write.last_operation());
    assert!(write.last_operation() < summary.file_state_synchronization().operation());
    assert!(summary.file_state_synchronization().operation() < summary.rename_operation());
    assert!(summary.rename_operation() < summary.namespace_directory_synchronization().operation());
    assert!(
        summary.namespace_directory_synchronization().operation()
            < summary
                .store_root_synchronization()
                .expect("initial scaffold publication requires root synchronization")
                .operation()
    );
    assert!(
        summary
            .store_root_synchronization()
            .expect("initial scaffold publication requires root synchronization")
            .operation()
            < parent_sync.operation()
    );
    assert_eq!(
        std::fs::read(root.path().join("store/namespace/identity")).expect("OS observe identity"),
        b"framed-identity-bytes"
    );

    let identity = owner.identity_record_path();
    let deletion_handle = match owner.open_existing_for_mutation(&identity).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        other => panic!("published identity did not reopen: {other:?}"),
    };
    let removed = match owner.delete_namespace_file(deletion_handle) {
        NamespaceDeletionOutcome::Removed(removed) => removed,
        other => panic!("delete failed: {other:?}"),
    };
    assert!(matches!(
        removed.synchronize_removal(),
        DurableDeletionOutcome::Durable(_)
    ));
    assert!(!root.path().join("store/namespace/identity").exists());
}

#[test]
fn later_same_directory_replacement_does_not_repeat_scaffold_barriers() {
    let (_root, owner) = owner();
    let first = synchronized_staged(&owner, 27, b"first");
    let first = match first.replace(owner.identity_publication_target()) {
        AtomicReplacementOutcome::Replaced(replaced) => replaced,
        other => panic!("first replacement: {other:?}"),
    };
    assert!(matches!(
        first.synchronize_publication(),
        DurableNamespacePublicationOutcome::Published(_)
    ));

    let second = synchronized_staged(&owner, 28, b"second");
    let second = match second.replace(owner.identity_publication_target()) {
        AtomicReplacementOutcome::Replaced(replaced) => replaced,
        other => panic!("second replacement: {other:?}"),
    };
    let published = match second.synchronize_publication() {
        DurableNamespacePublicationOutcome::Published(published) => published,
        other => panic!("second publication: {other:?}"),
    };
    assert!(published.store_root_synchronization().is_none());
    assert!(published.root_parent_synchronization().is_none());
}

#[test]
fn wrong_type_destination_naturally_classifies_rename_failure() {
    let (root, owner) = owner();
    let synchronized = synchronized_staged(&owner, 29, b"candidate");
    std::fs::create_dir(root.path().join("store/namespace/identity"))
        .expect("wrong-type destination");

    assert!(matches!(
        synchronized.replace(owner.identity_publication_target()),
        AtomicReplacementOutcome::Indeterminate(indeterminate)
            if indeterminate.stage() == NamespacePublicationStage::AtomicReplacement
                && matches!(
                    indeterminate.failure().kind(),
                    MediaOperationFailureKind::IndeterminateEffect {
                        attempted: MediaAttemptedEffect::AtomicReplacement,
                        ..
                    }
                )
    ));
    assert!(root.path().join("store/namespace/identity").is_dir());
}
