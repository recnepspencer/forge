use super::*;

#[test]
fn exact_source_lease_recovers_then_releases_durably_and_idempotently() {
    let world = tempfile::tempdir().expect("world");
    let source = world.path().join("source");
    let registry_root = world.path().join("leases");
    std::fs::create_dir_all(&source).expect("source directory");
    std::fs::write(source.join("wal.segment"), b"source closure").expect("source artifact");
    let registry = RecoverySourceLeaseRegistry::open(&registry_root).expect("registry");
    let lease = registry
        .admit_pitr_source_cut(RecoverySourceLeaseRequest::new(
            [1; 32],
            [2; 32],
            &source,
            vec!["wal.segment".into()],
        ))
        .expect("admitted source cut")
        .lease();
    let retry = lease.clone();
    let identity = lease.binding_fingerprint();

    drop(registry);
    let reopened = RecoverySourceLeaseRegistry::open(&registry_root).expect("reopen registry");
    let recovered = reopened.recover_active().expect("recover active leases");
    let [RecoveredRecoverySourceLease::PointInTimeRecovery(recovered)] = recovered.as_slice()
    else {
        panic!("exactly one PITR lease must recover");
    };
    assert_eq!(recovered.binding_fingerprint(), identity);
    let first = recovered.clone().release().expect("recovered release");
    let original_retry = lease.release().expect("original handle retry");
    let second = retry.release().expect("idempotent release");
    assert_eq!(first, original_retry);
    assert_eq!(first, second);
    assert_eq!(first.lease_identity(), identity);
    assert!(std::fs::read_dir(registry_root)
        .expect("lease directory")
        .next()
        .is_none());
}
