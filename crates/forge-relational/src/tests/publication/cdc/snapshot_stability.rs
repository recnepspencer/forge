use crate::tests::support::*;

#[test]
fn subscriber_cdc_is_snapshot_stable_under_hot_rewrite_pressure() {
    let mut pinned_runtime =
        runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let mut unpinned_runtime =
        runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);

    let baseline_pinned = create_entity_outcome(&mut pinned_runtime, "baseline");
    let baseline_unpinned = create_entity_outcome(&mut unpinned_runtime, "baseline");
    let pinned_entity = changed_entities(&baseline_pinned)[0];
    let unpinned_entity = changed_entities(&baseline_unpinned)[0];
    let pinned_snapshot = pinned_runtime.visibility_authority().snapshot();

    let rewrite_names = ["rewrite-1", "rewrite-2", "rewrite-3", "rewrite-4"];
    let mut pinned_latest = baseline_pinned.clone();
    for name in rewrite_names {
        pinned_latest = update_entity(&mut pinned_runtime, pinned_entity, name);
        let _ = update_entity(&mut unpinned_runtime, unpinned_entity, name);
    }

    let churn_names = ["churn-a", "churn-b", "churn-c"];
    for churn in churn_names {
        let _ = create_entity_outcome(&mut pinned_runtime, churn);
        let _ = create_entity_outcome(&mut unpinned_runtime, churn);
    }

    let checkpoint =
        checkpoint_for_schema_version(baseline_pinned.patch_position(), SchemaVersionId(1));
    let pinned_cdc = pinned_runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            checkpoint.clone(),
            16,
        ))
        .unwrap();
    let unpinned_cdc = unpinned_runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 16))
        .unwrap();

    assert_eq!(pinned_cdc.patches, unpinned_cdc.patches);
    assert_eq!(pinned_cdc.recovery_decision, unpinned_cdc.recovery_decision);

    let pinned_snapshot_read = pinned_runtime
        .visibility_reads()
        .read_snapshot(&pinned_snapshot)
        .unwrap();
    let pinned_latest_read = pinned_runtime
        .visibility_reads()
        .read_snapshot(&pinned_latest.snapshot)
        .unwrap();

    assert_eq!(
        read_entity_name(pinned_snapshot_read.get_entity(pinned_entity).unwrap()),
        Some("baseline")
    );
    assert_eq!(
        read_entity_name(pinned_latest_read.get_entity(pinned_entity).unwrap()),
        Some("rewrite-4")
    );

    let retention = pinned_runtime.retention_access().inspect_plan();
    assert!(retention.snapshot_pinned_entities >= 1);

    assert!(pinned_runtime
        .visibility_authority()
        .release_snapshot(&pinned_snapshot));
    let released = pinned_runtime.retention_access().inspect_plan();
    assert_eq!(released.snapshot_pinned_entities, 0);
}
