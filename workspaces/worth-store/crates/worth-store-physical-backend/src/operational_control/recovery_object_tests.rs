use super::{ControlMediaFault, ControlMediaLocation, PhysicalOperationalControlStore};

#[test]
fn recovery_objects_exceed_the_control_record_cap_and_reopen_by_content_identity() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let content = vec![0x5a; super::durable_prefix_recovery::MAX_CONTROL_PAYLOAD_BYTES + 1];

    let handle = store
        .publish_recovery_object(&content)
        .expect("publish recovery object");
    assert!(handle.bytes() > super::durable_prefix_recovery::MAX_CONTROL_PAYLOAD_BYTES as u64);
    store
        .append_at_current_tail("large-object:published", b"small-content-handle")
        .expect("small control record");

    let reopened =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(path)).expect("reopen");
    assert_eq!(
        reopened
            .read_recovery_object(handle)
            .expect("read verified object"),
        content
    );
}

#[test]
fn recovery_object_mutation_is_localized_before_control_state_can_use_it() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let handle = store
        .publish_recovery_object(b"owner-issued lease")
        .expect("publish recovery object");
    let object_root = path.with_file_name("control.log.objects");
    let object = std::fs::read_dir(object_root)
        .expect("object directory")
        .next()
        .expect("object entry")
        .expect("read object entry")
        .path();
    std::fs::write(object, b"owner-issued lie!!").expect("mutate object");

    assert!(matches!(
        store.read_recovery_object(handle),
        Err(ControlMediaFault::CorruptRecoveryObject { .. })
            | Err(ControlMediaFault::RecoveryObjectLengthMismatch { .. })
    ));
}
