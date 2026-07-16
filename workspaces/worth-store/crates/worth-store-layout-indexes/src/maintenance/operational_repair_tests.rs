use sha2::{Digest, Sha256};

use super::{
    DerivedIndexRepairExecutionDenial, DerivedIndexRepairRequest, LayoutOperationalRepairOwner,
};

#[test]
fn derived_repair_is_copy_on_write_and_idempotent_after_publication() {
    let world = tempfile::tempdir().expect("world");
    let target = world.path().join("index.current");
    let replacement = world.path().join("index.next");
    std::fs::write(&target, b"old-derived-index").expect("target");
    std::fs::write(&replacement, b"rebuilt-derived-index").expect("replacement");
    let request = request(&target, &replacement);
    let plan = LayoutOperationalRepairOwner::lower(request).expect("lowered owner plan");
    let expected_plan = plan.fingerprint();

    let first = LayoutOperationalRepairOwner::execute(plan.clone()).expect("first publication");
    let reopened = LayoutOperationalRepairOwner::execute(plan).expect("idempotent reopen");

    assert_eq!(first, reopened);
    assert_eq!(first.plan_fingerprint(), expected_plan);
    assert_eq!(
        std::fs::read(target).expect("current index"),
        b"rebuilt-derived-index"
    );
}

#[test]
fn restart_recovery_revalidates_the_durable_owner_effect() {
    let world = tempfile::tempdir().expect("world");
    let target = world.path().join("index.current");
    let replacement = world.path().join("index.next");
    std::fs::write(&target, b"old-derived-index").expect("target");
    std::fs::write(&replacement, b"rebuilt-derived-index").expect("replacement");
    let plan = LayoutOperationalRepairOwner::lower(request(&target, &replacement))
        .expect("lowered owner plan");
    let executed = LayoutOperationalRepairOwner::execute(plan.clone()).expect("publication");
    std::fs::remove_file(replacement).expect("source may retire after owner publication");

    assert_eq!(
        LayoutOperationalRepairOwner::recover_applied(&plan).expect("durable effect"),
        executed
    );

    std::fs::write(&target, b"corrupt-after-publication").expect("corrupt target");
    assert!(matches!(
        LayoutOperationalRepairOwner::recover_applied(&plan),
        Err(DerivedIndexRepairExecutionDenial::PersistedEffectMismatch)
    ));
}

#[test]
fn derived_repair_rejects_target_drift_before_mutation() {
    let world = tempfile::tempdir().expect("world");
    let target = world.path().join("index.current");
    let replacement = world.path().join("index.next");
    std::fs::write(&target, b"old-derived-index").expect("target");
    std::fs::write(&replacement, b"rebuilt-derived-index").expect("replacement");
    let plan = LayoutOperationalRepairOwner::lower(request(&target, &replacement))
        .expect("lowered owner plan");
    std::fs::write(&target, b"concurrent-new-generation").expect("drifted target");

    assert!(matches!(
        LayoutOperationalRepairOwner::execute(plan),
        Err(DerivedIndexRepairExecutionDenial::StaleTarget)
    ));
    assert_eq!(
        std::fs::read(target).expect("unchanged drifted target"),
        b"concurrent-new-generation"
    );
}

fn request(target: &std::path::Path, replacement: &std::path::Path) -> DerivedIndexRepairRequest {
    DerivedIndexRepairRequest::new(
        [3; 32],
        target,
        Sha256::digest(b"old-derived-index").into(),
        replacement,
        Sha256::digest(b"rebuilt-derived-index").into(),
        7,
        8,
        1024,
    )
}
