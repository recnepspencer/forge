use super::*;

#[test]
fn policy_eager_layout_support_lane_resolves_to_proof_only_without_trigger() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = entity_set_request(branch_id, commit_id);

    let prepared = store
        .prepare_milestone_6_layout_support_with_policy(
            request.clone(),
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(true, true, 2),
        )
        .unwrap();

    assert_eq!(
        prepared.requested_lane(),
        Milestone6LayoutSupportLane::PolicyEagerMaterialized
    );
    assert_eq!(
        prepared.resolved_lane(),
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        prepared.publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(prepared.request(), &request);
    assert_eq!(prepared.layout_materialization_artifact_id(), None);
    assert_eq!(
        store.counters().milestone_6_policy_eager_resolution_count,
        1
    );
    assert_eq!(store.counters().milestone_6_policy_eager_publish_count, 0);
    assert_eq!(
        store
            .counters()
            .milestone_6_policy_eager_reuse_existing_count,
        0
    );
}

#[test]
fn policy_eager_layout_support_lane_materializes_when_branch_is_hot() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = entity_set_request(branch_id, commit_id);

    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let prepared = store
        .prepare_milestone_6_layout_support_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(true, false, 0),
        )
        .unwrap();

    assert_eq!(
        prepared.requested_lane(),
        Milestone6LayoutSupportLane::PolicyEagerMaterialized
    );
    assert_eq!(
        prepared.resolved_lane(),
        Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting
    );
    assert_eq!(
        prepared.publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
    assert_eq!(
        prepared.layout_materialization_artifact_id(),
        Some(materialized.artifact_id())
    );
    assert_eq!(
        store.counters().milestone_6_policy_eager_resolution_count,
        1
    );
    assert_eq!(store.counters().milestone_6_policy_eager_publish_count, 0);
    assert_eq!(
        store
            .counters()
            .milestone_6_policy_eager_reuse_existing_count,
        1
    );
}

#[test]
fn policy_eager_layout_support_lane_materializes_at_repeated_scope_threshold() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = entity_set_request(branch_id, commit_id);
    let policy = Milestone6LayoutSupportPolicy::new(false, true, 2);

    let first = store
        .prepare_milestone_6_layout_support_with_policy(
            request.clone(),
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            policy,
        )
        .unwrap();
    let second = store
        .prepare_milestone_6_layout_support_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            policy,
        )
        .unwrap();

    assert_eq!(
        first.resolved_lane(),
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        second.resolved_lane(),
        Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
    );
    assert_eq!(
        second.publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert!(second.layout_materialization_artifact_id().is_some());
    assert_eq!(
        store.counters().milestone_6_policy_eager_resolution_count,
        2
    );
    assert_eq!(store.counters().milestone_6_policy_eager_publish_count, 1);
    assert_eq!(
        store
            .counters()
            .milestone_6_policy_eager_reuse_existing_count,
        0
    );
}
