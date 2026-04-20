use super::*;

#[test]
fn layout_slice_identity_is_stable_across_equivalent_scope_ordering() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let left = admitted_plan(
        &store,
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id.clone(), commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-c".to_string(),
                "entity-a".to_string(),
                "entity-b".to_string(),
            ])),
            AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
        ),
    );
    let right = admitted_plan(
        &store,
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-b".to_string(),
                "entity-c".to_string(),
                "entity-a".to_string(),
            ])),
            AspectProjectionSet::new(vec!["status".to_string(), "profile".to_string()]),
        ),
    );

    assert_eq!(left.slice_ids(), right.slice_ids());
    assert_eq!(left.structural_block_id(), right.structural_block_id());
}

#[test]
fn single_entity_scope_is_admitted_with_verified_direct_regime() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let plan = admitted_plan(
        &store,
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ),
    );

    assert_eq!(
        plan.performance().strategy,
        AspectReadRegime::DirectLayoutSlice
    );
    assert_eq!(
        plan.performance().complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(plan.performance().layout_slices_read, 1);
}
#[test]
fn entity_set_uniform_scope_is_admitted() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let plan = admitted_plan(
        &store,
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-a".to_string(),
                "entity-b".to_string(),
            ])),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ),
    );

    assert_eq!(
        plan.performance().strategy,
        AspectReadRegime::BlockReuseBacked
    );
    assert_eq!(plan.slice_ids().len(), 2);
}
#[test]
fn milestone_6_layout_materialization_persists_and_roundtrips_across_backends() {
    for lane in [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite] {
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let root = latest_envelope(&runtime);
        let branch_id = root.branch_context.clone();
        let commit_id = root.commit.commit_id;
        let request = entity_set_request(branch_id.clone(), commit_id);

        match lane {
            StoreLane::InMemory => {
                let mut store = crate::ForgeStoreBuilder::new().in_memory().build().unwrap();
                store.append_canonical_commit(root.clone()).unwrap();
                let materialized = store
                    .materialize_milestone_6_layout_support(request.clone())
                    .unwrap();
                let fetched = store.fetch_milestone_6_layout_support(request).unwrap();
                assert_eq!(materialized, fetched);
                assert_eq!(
                    materialized.block_reuse().structural_block_id(),
                    materialized.admitted_plan().structural_block_id()
                );
            }
            StoreLane::LocalFile => {
                let path = unique_test_store_path("layout-materialization");
                let mut store = crate::ForgeStoreBuilder::new()
                    .local_file(path.clone())
                    .build()
                    .unwrap();
                store.append_canonical_commit(root.clone()).unwrap();
                let materialized = store
                    .materialize_milestone_6_layout_support(request.clone())
                    .unwrap();
                drop(store);
                let reopened = crate::ForgeStoreBuilder::new()
                    .local_file(path)
                    .build()
                    .unwrap();
                let fetched = reopened.fetch_milestone_6_layout_support(request).unwrap();
                assert_eq!(materialized, fetched);
            }
            StoreLane::Sqlite => {
                let path = unique_test_sqlite_path("layout-materialization");
                let mut store = crate::ForgeStoreBuilder::new()
                    .sqlite_file(path.clone())
                    .build()
                    .unwrap();
                store.append_canonical_commit(root).unwrap();
                let materialized = store
                    .materialize_milestone_6_layout_support(request.clone())
                    .unwrap();
                drop(store);
                let reopened = crate::ForgeStoreBuilder::new()
                    .sqlite_file(path)
                    .build()
                    .unwrap();
                let fetched = reopened.fetch_milestone_6_layout_support(request).unwrap();
                assert_eq!(materialized, fetched);
            }
        }
    }
}

#[test]
fn proof_only_layout_support_lane_stays_unpublished() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = entity_set_request(branch_id, commit_id);

    let prepared = store
        .prepare_milestone_6_layout_support(request.clone(), Milestone6LayoutSupportLane::ProofOnly)
        .unwrap();

    assert_eq!(
        prepared.requested_lane(),
        Milestone6LayoutSupportLane::ProofOnly
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
    assert_eq!(store.counters().aspect_layout_plan_count, 1);
    assert_eq!(store.counters().aspect_layout_admitted_count, 1);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 0);
    assert_eq!(store.counters().chunk_model_freeze_count, 0);

    let error = store.fetch_milestone_6_layout_support(request).unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::AspectLayoutArtifactMissing
    );
}

#[test]
fn on_demand_materialized_layout_support_lane_publishes_support_once() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = entity_set_request(branch_id, commit_id);

    let prepared = store
        .prepare_milestone_6_layout_support(
            request.clone(),
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap();
    let fetched = store
        .fetch_milestone_6_layout_support(request.clone())
        .unwrap();

    assert_eq!(
        prepared.requested_lane(),
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        prepared.resolved_lane(),
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        prepared.publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(
        prepared.layout_materialization_artifact_id(),
        Some(fetched.artifact_id())
    );
    assert_eq!(store.counters().aspect_layout_plan_count, 3);
    assert_eq!(store.counters().aspect_layout_admitted_count, 3);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);

    let prepared_again = store
        .prepare_milestone_6_layout_support(
            request,
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap();
    assert_eq!(
        prepared_again.layout_materialization_artifact_id(),
        Some(fetched.artifact_id())
    );
    assert_eq!(
        prepared_again.publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
    assert_eq!(store.counters().aspect_layout_plan_count, 4);
    assert_eq!(store.counters().aspect_layout_admitted_count, 4);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);
}
