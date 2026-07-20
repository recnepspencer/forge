use super::super::super::*;
use super::fixture::{created, owner, staged_path};

#[test]
fn final_file_link_is_not_followed_by_open() {
    let (root, owner) = owner();
    let path = staged_path(&owner, 22);
    drop(created(owner.create_new(&path)));
    let physical_path = root.path().join("store").join(path.as_path());
    std::fs::remove_file(&physical_path).expect("remove replaceable fixture");
    let outside = root.path().join("outside-sentinel");
    std::fs::write(&outside, b"outside-must-not-change").expect("write outside sentinel");
    create_file_link(&outside, &physical_path).expect("create file link fixture");

    assert!(matches!(
        owner.open_existing(&path).into_result(),
        NamespaceFileOpenResult::Failed(_)
    ));
    assert_eq!(
        std::fs::read(outside).expect("read outside sentinel"),
        b"outside-must-not-change"
    );
}

#[test]
fn file_handle_from_another_root_cannot_authorize_deletion() {
    let (root_a, owner_a) = owner();
    let (root_b, owner_b) = owner();
    let path_a = staged_path(&owner_a, 23);
    let path_b = staged_path(&owner_b, 23);
    let foreign_handle = created(owner_a.create_new(&path_a));
    drop(created(owner_b.create_new(&path_b)));

    assert!(matches!(
        owner_b.delete_namespace_file(foreign_handle),
        NamespaceDeletionOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
    assert!(root_a.path().join("store").join(path_a.as_path()).is_file());
    assert!(root_b.path().join("store").join(path_b.as_path()).is_file());
    assert_eq!(owner_b.counters().confinement_denials(), 1);
}

#[cfg(unix)]
fn create_file_link(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_link(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
