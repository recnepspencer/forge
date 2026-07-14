use super::*;

#[test]
fn maintenance_recovery_report_scaffolds_non_snapshot_families_without_faking_presence() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let report = store.maintenance_recovery_report().unwrap();
    let snapshot_entry = report
        .entries()
        .iter()
        .find(|entry| entry.family() == MaintenanceArtifactFamily::Snapshot)
        .expect("snapshot entry should be present");
    assert_eq!(
        snapshot_entry.disposition(),
        MaintenanceRecoveryDisposition::RequireRebuild
    );

    for family in [
        MaintenanceArtifactFamily::Compaction,
        MaintenanceArtifactFamily::Reclaim,
        MaintenanceArtifactFamily::Capsule,
    ] {
        let entry = report
            .entries()
            .iter()
            .find(|entry| entry.family() == family)
            .expect("scaffolded maintenance family should be present");
        assert_eq!(
            entry.disposition(),
            MaintenanceRecoveryDisposition::NotPresent
        );
    }
    assert_eq!(
        store.counters().interrupted_maintenance_recovery_count,
        0,
        "read-only maintenance reports must not mutate recovery counters"
    );
}
