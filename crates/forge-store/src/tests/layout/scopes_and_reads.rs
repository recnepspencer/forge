use super::*;

#[test]
fn authority_rebuild_preserves_execution_and_chunk_export_from_minimal_honest_source_set() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-authority-rebuild-surfaces");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let before_read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result before rebuild, got {other:?}"),
    };
    let before_dedup = store.execute_dedup_backed_read(request.clone()).unwrap();
    let before_export = store
        .export_milestone_6_chunk_model(request.clone())
        .unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap();
    reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    let after_read = match reopened
        .execute_aspect_layout_read(request.clone())
        .unwrap()
    {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result after rebuild, got {other:?}"),
    };
    let after_dedup = reopened.execute_dedup_backed_read(request.clone()).unwrap();
    let after_export = reopened.export_milestone_6_chunk_model(request).unwrap();

    assert_eq!(before_read.plan(), after_read.plan());
    assert_eq!(
        before_read.scope_membership_artifact_id(),
        after_read.scope_membership_artifact_id()
    );
    assert_eq!(
        before_read.chunk_membership_artifact_id(),
        after_read.chunk_membership_artifact_id()
    );
    assert_eq!(
        before_dedup.structural_block_lookup().structural_block_id(),
        after_dedup.structural_block_lookup().structural_block_id()
    );
    assert_eq!(
        before_dedup.structural_block_lookup().slice_ids(),
        after_dedup.structural_block_lookup().slice_ids()
    );
    assert_eq!(
        before_export.physical_chunk_id(),
        after_export.physical_chunk_id()
    );
    assert_eq!(
        before_export.determinism_digest(),
        after_export.determinism_digest()
    );
    assert_eq!(
        before_export.chunk_membership_artifact_id(),
        after_export.chunk_membership_artifact_id()
    );
}

#[test]
fn cdc_touched_scope_is_admitted() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let plan = admitted_plan(
        &store,
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::CdcTouched(CdcTouchedAspectScope::new(
                "cdc-run-42",
                vec!["entity-a".to_string(), "entity-b".to_string()],
            )),
            AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
        ),
    );

    assert_eq!(
        plan.performance().complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(plan.slice_ids().len(), 2);
}

#[test]
fn generalized_scope_returns_explicit_fallback() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let decision = store
        .plan_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::Generalized {
                descriptor: "wildcard-join".to_string(),
            },
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ))
        .unwrap();

    match decision {
        AspectLayoutReadPlanDecision::Fallback(plan) => {
            assert_eq!(
                plan.performance().strategy,
                AspectReadRegime::ExplicitBroadFallback
            );
        }
        other => panic!("expected fallback plan, got {other:?}"),
    }
}

#[test]
fn over_budget_scope_never_admits() {
    let (store, branch_id, commit_id) = store_with_root_commit();
    let decision = store
        .plan_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(
                (0..40).map(|index| format!("entity-{index}")).collect(),
            )),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ))
        .unwrap();

    assert!(matches!(
        decision,
        AspectLayoutReadPlanDecision::Fallback(_)
    ));
}

#[test]
fn chunk_determinism_and_cross_milestone_references_flow_only_from_admitted_plan() {
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
    let block_reuse = store.admit_structural_block_reuse(plan.clone()).unwrap();
    let frozen = store.freeze_chunk_model(plan.clone()).unwrap();
    let milestone_7 = store
        .admit_milestone_7_independent_layout_reference(plan)
        .unwrap();
    let milestone_9 = store
        .admit_milestone_9_physical_chunk_reference(frozen.clone())
        .unwrap();

    assert_eq!(block_reuse.slice_ids().len(), 2);
    assert_eq!(milestone_7.branch_id().0, "main");
    assert_eq!(
        milestone_9.determinism_digest(),
        frozen.witness().determinism_digest()
    );
}

#[test]
fn structural_block_lookup_reads_published_block_family() {
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
        .materialize_milestone_6_layout_support(request)
        .unwrap();

    let lookup = store
        .structural_block_lookup(crate::StructuralBlockLookup::new(
            materialized.block_reuse().structural_block_id().clone(),
        ))
        .unwrap();

    assert_eq!(
        lookup.structural_block_id(),
        materialized.block_reuse().structural_block_id()
    );
    assert_eq!(lookup.slice_ids(), materialized.block_reuse().slice_ids());
    assert_eq!(
        lookup.equivalence_contract_version(),
        materialized.block_reuse().equivalence_contract_version()
    );
}
