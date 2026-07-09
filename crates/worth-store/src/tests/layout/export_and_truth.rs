use super::*;

#[test]
fn aspect_layout_control_truth_matches_admitted_execution_target() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch =
        worth_relational::facade::history::BranchId("layout-control-feature".to_string());

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();
    update_entity_on_branch(
        &mut runtime,
        entity_id,
        "feature-control",
        Some(feature_branch.clone()),
    );
    let feature_head = latest_envelope(&runtime);
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(feature_branch.clone(), feature_head.commit.commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );
    store.append_canonical_commit(feature_head).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result, got {other:?}"),
    };
    let control = store.read_aspect_layout_control_truth(request).unwrap();

    assert_eq!(control.branch_id(), &feature_branch);
    assert_eq!(
        control.frontier_commit_id(),
        read.plan().request().target().frontier_commit_id()
    );
    assert_eq!(
        control.scope_class(),
        read.plan().request().scope_class().label()
    );
    assert_eq!(
        control.authoritative_truth_digest(),
        read.semantic_truth_digest()
    );
    assert_eq!(
        control.authoritative_commit_count(),
        read.authoritative_commit_count()
    );
}

#[test]
fn chunk_model_export_matches_materialized_chunk_reference() {
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

    let export = store.export_milestone_6_chunk_model(request).unwrap();
    assert_eq!(
        export.physical_chunk_id(),
        materialized.milestone_9_reference().physical_chunk_id()
    );
    assert_eq!(
        export.determinism_digest(),
        materialized.milestone_9_reference().determinism_digest()
    );
    assert_eq!(
        export.chunk_member_count(),
        materialized.milestone_9_reference().chunk_member_count()
    );
    assert_eq!(
        export.layout_materialization_artifact_id(),
        Some(materialized.artifact_id())
    );
}

#[test]
fn chunk_model_export_in_proof_only_lane_is_explicitly_unsupported() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let error = store
        .export_milestone_6_chunk_model_in_lane(request, Milestone6LayoutSupportLane::ProofOnly)
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::AspectLayoutArtifactMissing
    );
}

#[test]
fn chunk_model_export_in_policy_eager_lane_reflects_resolved_lane() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let first_error = store
        .export_milestone_6_chunk_model_in_lane_with_policy(
            request.clone(),
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap_err();
    assert_eq!(
        first_error.kind(),
        &crate::StoreErrorKind::AspectLayoutArtifactMissing
    );

    let export = store
        .export_milestone_6_chunk_model_in_lane_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap();
    assert_eq!(
        export.requested_layout_support_lane(),
        Milestone6LayoutSupportLane::PolicyEagerMaterialized
    );
    assert_eq!(
        export.resolved_layout_support_lane(),
        Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
    );
    assert_eq!(
        export.layout_support_publication_disposition(),
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert!(export.layout_materialization_artifact_id().is_some());
}

#[test]
fn aspect_layout_control_truth_supports_linear_main_branch_targets() {
    let (mut store, branch_id, commit_id) = store_with_root_commit();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id.clone(), commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let control = store.read_aspect_layout_control_truth(request).unwrap();

    assert_eq!(control.branch_id(), &branch_id);
    assert_eq!(control.frontier_commit_id(), commit_id);
    assert_eq!(control.scope_class(), "entity_set_uniform");
    let read = match store
        .execute_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-a".to_string(),
                "entity-b".to_string(),
            ])),
            AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
        ))
        .unwrap()
    {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted main-branch execution result, got {other:?}"),
    };
    assert_eq!(
        control.authoritative_truth_digest(),
        read.semantic_truth_digest()
    );
    assert_eq!(
        control.authoritative_commit_count(),
        read.authoritative_commit_count()
    );
}
