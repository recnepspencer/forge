use super::*;

#[test]
fn dedup_and_control_truth_remain_aligned_on_feature_branch_history() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = forge_relational::facade::history::BranchId("layout-feature".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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
        "feature-only",
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

    let aspect_read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result, got {other:?}"),
    };
    let dedup_read = store.execute_dedup_backed_read(request.clone()).unwrap();
    let control = store.read_aspect_layout_control_truth(request).unwrap();

    assert_eq!(
        aspect_read.plan().request().target().branch_id(),
        control.branch_id()
    );
    assert_eq!(
        aspect_read.plan().request().target().frontier_commit_id(),
        control.frontier_commit_id()
    );
    assert_eq!(dedup_read.read().plan(), aspect_read.plan());
    assert_eq!(
        dedup_read.structural_block_lookup().slice_ids(),
        aspect_read.plan().slice_ids()
    );
    assert_eq!(
        aspect_read.semantic_truth_digest(),
        control.authoritative_truth_digest()
    );
    assert_eq!(
        aspect_read.authoritative_commit_count(),
        control.authoritative_commit_count()
    );
    assert_eq!(
        dedup_read.read().semantic_truth_digest(),
        control.authoritative_truth_digest()
    );
    assert_eq!(
        dedup_read.read().authoritative_commit_count(),
        control.authoritative_commit_count()
    );
}

#[test]
fn structural_block_identity_is_stable_across_equivalent_branch_publications() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let main_branch = root.branch_context.clone();
    let feature_branch =
        forge_relational::facade::history::BranchId("layout-semantic-block".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root.clone()).unwrap();
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

    update_entity_on_branch(&mut runtime, entity_id, "same-value", None);
    let main_head = latest_envelope(&runtime);
    store.append_canonical_commit(main_head.clone()).unwrap();

    update_entity_on_branch(
        &mut runtime,
        entity_id,
        "same-value",
        Some(feature_branch.clone()),
    );
    let feature_head = latest_envelope(&runtime);
    store.append_canonical_commit(feature_head.clone()).unwrap();

    let main_request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(main_branch.clone(), main_head.commit.commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );
    let feature_request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(feature_branch, feature_head.commit.commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    );

    let main_materialized = store
        .materialize_milestone_6_layout_support(main_request.clone())
        .unwrap();
    let feature_materialized = store
        .materialize_milestone_6_layout_support(feature_request.clone())
        .unwrap();

    assert_eq!(
        main_materialized.block_reuse().structural_block_id(),
        feature_materialized.block_reuse().structural_block_id()
    );
    assert_eq!(
        main_materialized.block_reuse().slice_ids(),
        feature_materialized.block_reuse().slice_ids()
    );

    let lookup = store
        .structural_block_lookup(crate::StructuralBlockLookup::new(
            main_materialized
                .block_reuse()
                .structural_block_id()
                .clone(),
        ))
        .unwrap();
    assert_eq!(
        lookup
            .supporting_layout_materialization_artifact_ids()
            .len(),
        2
    );
    assert!(lookup
        .supporting_layout_materialization_artifact_ids()
        .contains(&main_materialized.artifact_id().to_string()));
    assert!(lookup
        .supporting_layout_materialization_artifact_ids()
        .contains(&feature_materialized.artifact_id().to_string()));
}

#[test]
fn aspect_layout_execution_preserves_explicit_fallback() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let decision = store
        .execute_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::Generalized {
                descriptor: "wildcard-join".to_string(),
            },
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ))
        .unwrap();

    match decision {
        crate::AspectLayoutReadExecutionDecision::Fallback(plan) => {
            assert_eq!(
                plan.performance().strategy,
                AspectReadRegime::ExplicitBroadFallback
            );
        }
        other => panic!("expected fallback execution result, got {other:?}"),
    }
}

#[test]
fn layout_admission_and_witness_flow_are_backend_stable() {
    for lane in [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite] {
        let (store, branch_id, commit_id) = store_with_root_commit_for_lane(lane);
        let plan = admitted_plan(
            &store,
            AspectLayoutReadRequest::new(
                AspectLayoutTarget::new(branch_id.clone(), commit_id),
                AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                    "entity-a".to_string(),
                    "entity-b".to_string(),
                ])),
                AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
            ),
        );
        let reuse = store.admit_structural_block_reuse(plan.clone()).unwrap();
        let frozen = store.freeze_chunk_model(plan.clone()).unwrap();
        let milestone_7 = store
            .admit_milestone_7_independent_layout_reference(plan.clone())
            .unwrap();
        let milestone_9 = store
            .admit_milestone_9_physical_chunk_reference(frozen.clone())
            .unwrap();

        assert_eq!(
            plan.performance().complexity_status,
            ComplexityStatus::Verified
        );
        assert_eq!(reuse.structural_block_id(), plan.structural_block_id());
        assert_eq!(milestone_7.branch_id(), &branch_id);
        assert_eq!(
            milestone_9.determinism_digest(),
            frozen.witness().determinism_digest(),
            "lane {} diverged on chunk determinism",
            lane.label()
        );
    }
}
