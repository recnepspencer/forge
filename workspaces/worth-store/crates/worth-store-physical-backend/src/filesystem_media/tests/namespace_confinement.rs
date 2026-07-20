use super::super::*;

#[test]
fn root_bound_file_path_cannot_cross_equal_layouts() {
    let (_root_a, owner_a) = owner();
    let (_root_b, owner_b) = owner();
    let path_a = owner_a.identity_record_path();

    assert!(matches!(
        owner_b.open_existing(&path_a).into_result(),
        NamespaceFileOpenResult::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
}

#[test]
fn root_bound_directory_capability_cannot_cross_equal_layouts() {
    let (_root_a, owner_a) = owner();
    let (_root_b, owner_b) = owner();

    let denial = owner_b
        .require_owned_directory(owner_a.families().handle())
        .expect_err("foreign directory capability must be denied");
    assert_eq!(
        denial.kind(),
        NamespaceConfinementDenialKind::AuthorityMismatch
    );
    assert!(matches!(
        owner_b.synchronize_directory_publication(owner_a.families().handle()),
        DirectoryPublicationSynchronizationOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
}

#[test]
fn final_root_link_is_denied_through_production_admission() {
    let outside = tempfile::tempdir().expect("outside root");
    let container = tempfile::tempdir().expect("link container");
    let link = container.path().join("linked-root");
    create_directory_link(outside.path(), &link).expect("create root link fixture");

    let denial = FilesystemMediaOwner::admit(&link, FilesystemMediaAdmissionAuthority::for_test())
        .expect_err("linked root must not become ambient authority");
    assert_eq!(
        denial,
        FilesystemMediaOwnerAdmissionDenial::Confinement(NamespaceConfinementDenial::structural(
            NamespaceConfinementDenialKind::LinkLikeEntry
        ))
    );
}

#[test]
fn regular_file_root_is_denied_without_replacing_or_widening_it() {
    let parent = tempfile::tempdir().expect("root parent");
    let root = parent.path().join("store");
    std::fs::write(&root, b"sentinel").expect("create wrong-type root");
    let readonly_before = std::fs::metadata(&root)
        .expect("root metadata")
        .permissions()
        .readonly();

    let denial = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect_err("regular file cannot become a Store root");
    assert_eq!(
        denial,
        FilesystemMediaOwnerAdmissionDenial::Confinement(NamespaceConfinementDenial::structural(
            NamespaceConfinementDenialKind::EntryTypeMismatch
        ))
    );
    assert_eq!(
        std::fs::read(&root).expect("sentinel survives"),
        b"sentinel"
    );
    assert_eq!(
        std::fs::metadata(&root)
            .expect("root metadata after denial")
            .permissions()
            .readonly(),
        readonly_before
    );
}

fn owner() -> (tempfile::TempDir, FilesystemMediaOwner) {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("admit media owner");
    (parent, owner)
}

#[cfg(unix)]
fn create_directory_link(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_link(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
