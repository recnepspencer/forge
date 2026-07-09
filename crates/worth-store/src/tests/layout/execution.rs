use super::*;

#[test]
fn aspect_layout_execution_reads_published_scope_and_chunk_families() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();

    let read = match store.execute_aspect_layout_read(request).unwrap() {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result, got {other:?}"),
    };
    let expected_scope_membership_artifact_id =
        crate::layout::layout_scope_membership_artifact_id(materialized.admitted_plan().request())
            .unwrap();
    let expected_chunk_membership_artifact_id =
        crate::layout::chunk_membership_artifact_id(materialized.frozen_layout());

    assert_eq!(read.plan(), materialized.admitted_plan());
    assert_eq!(
        read.layout_materialization_artifact_id(),
        Some(materialized.artifact_id())
    );
    assert_eq!(
        read.scope_membership_artifact_id(),
        Some(expected_scope_membership_artifact_id.as_str())
    );
    assert_eq!(
        read.structural_block_artifact_id(),
        crate::layout::structural_block_artifact_id(
            materialized.block_reuse().structural_block_id()
        )
    );
    assert_eq!(
        read.chunk_membership_artifact_id(),
        Some(expected_chunk_membership_artifact_id.as_str())
    );
    assert_eq!(
        read.semantic_truth_digest(),
        materialized.semantic_truth_digest()
    );
    assert_eq!(
        read.authoritative_commit_count(),
        materialized.authoritative_commit_count()
    );
}

#[test]
fn aspect_layout_execution_in_proof_only_lane_stays_unpublished() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let read = match store
        .execute_aspect_layout_read_in_lane(request.clone(), Milestone6LayoutSupportLane::ProofOnly)
        .unwrap()
    {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted proof-only execution result, got {other:?}"),
    };

    assert_eq!(
        read.requested_layout_support_lane(),
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        read.resolved_layout_support_lane(),
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        read.layout_support_publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(read.scope_membership_artifact_id(), None);
    assert_eq!(read.chunk_membership_artifact_id(), None);
    assert_eq!(read.layout_materialization_artifact_id(), None);
    assert!(store.fetch_milestone_6_layout_support(request).is_err());
}

#[test]
fn aspect_layout_execution_in_on_demand_lane_materializes_support() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let read = match store
        .execute_aspect_layout_read_in_lane(
            request.clone(),
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap()
    {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted on-demand execution result, got {other:?}"),
    };
    let fetched = store.fetch_milestone_6_layout_support(request).unwrap();

    assert_eq!(
        read.requested_layout_support_lane(),
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        read.resolved_layout_support_lane(),
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        read.layout_support_publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(
        read.layout_materialization_artifact_id(),
        Some(fetched.artifact_id())
    );
}

#[test]
fn dedup_backed_read_uses_published_block_lookup() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();

    let read = store.execute_dedup_backed_read(request).unwrap();
    assert_eq!(
        read.structural_block_lookup().structural_block_id(),
        materialized.block_reuse().structural_block_id()
    );
    assert_eq!(read.read().plan(), materialized.admitted_plan());
    assert_eq!(
        read.read().semantic_truth_digest(),
        materialized.semantic_truth_digest()
    );
    assert_eq!(
        read.read().authoritative_commit_count(),
        materialized.authoritative_commit_count()
    );
    assert!(read
        .structural_block_lookup()
        .supporting_layout_materialization_artifact_ids()
        .contains(&materialized.artifact_id().to_string()));
}

#[test]
fn dedup_backed_read_in_proof_only_lane_preserves_semantic_parity_without_publication() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let dedup = store
        .execute_dedup_backed_read_in_lane(request.clone(), Milestone6LayoutSupportLane::ProofOnly)
        .unwrap();
    let control = store
        .read_aspect_layout_control_truth(request.clone())
        .unwrap();

    assert_eq!(
        dedup.read().requested_layout_support_lane(),
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        dedup.read().resolved_layout_support_lane(),
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        dedup.read().layout_support_publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(dedup.read().layout_materialization_artifact_id(), None);
    assert!(dedup
        .structural_block_lookup()
        .supporting_layout_materialization_artifact_ids()
        .is_empty());
    assert_eq!(
        dedup.read().semantic_truth_digest(),
        control.authoritative_truth_digest()
    );
    assert!(store.fetch_milestone_6_layout_support(request).is_err());
}

#[test]
fn dedup_backed_read_in_on_demand_lane_uses_published_support() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let dedup = store
        .execute_dedup_backed_read_in_lane(
            request.clone(),
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap();
    let fetched = store.fetch_milestone_6_layout_support(request).unwrap();

    assert_eq!(
        dedup.read().requested_layout_support_lane(),
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        dedup.read().resolved_layout_support_lane(),
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        dedup.read().layout_support_publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(
        dedup.read().layout_materialization_artifact_id(),
        Some(fetched.artifact_id())
    );
    assert!(dedup
        .structural_block_lookup()
        .supporting_layout_materialization_artifact_ids()
        .contains(&fetched.artifact_id().to_string()));
}
