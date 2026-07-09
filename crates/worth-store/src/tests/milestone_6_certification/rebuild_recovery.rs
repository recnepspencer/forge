use super::*;

#[test]
fn milestone_6_materialization_and_fetch_use_counted_admission_surfaces() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    assert_eq!(store.counters().aspect_layout_plan_count, 1);
    assert_eq!(store.counters().aspect_layout_admitted_count, 1);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);
    assert_eq!(
        store
            .counters()
            .milestone_7_layout_reference_admission_count,
        1
    );
    assert_eq!(
        store
            .counters()
            .milestone_9_physical_chunk_reference_admission_count,
        1
    );

    store.fetch_milestone_6_layout_support(request).unwrap();
    assert_eq!(store.counters().aspect_layout_plan_count, 2);
    assert_eq!(store.counters().aspect_layout_admitted_count, 2);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);
}

#[test]
fn milestone_6_rebuild_restores_derived_access_structures_from_materializations() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("worth-store-m6-derived-rebuild");

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let baseline_bundle = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    drop(store);

    force_clear_milestone_6_derived_access_structures(&path);

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let degraded_bundle = reopened
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    assert_eq!(degraded_bundle.truth_digest, baseline_bundle.truth_digest);
    assert_eq!(
        degraded_bundle.artifact_digest,
        baseline_bundle.artifact_digest
    );
    assert_eq!(
        degraded_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(
        degraded_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(
        degraded_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Debt
    );

    let rebuild = reopened
        .rebuild_milestone_6_derived_artifacts_from_materializations()
        .unwrap();
    assert_eq!(rebuild.layout_materialization_count(), 1);
    assert_eq!(rebuild.scope_membership_count(), 1);
    assert_eq!(rebuild.structural_block_count(), 1);
    assert_eq!(rebuild.chunk_membership_count(), 1);

    let rebuilt_bundle = reopened.milestone_6_certification_bundle(request).unwrap();
    assert_eq!(rebuilt_bundle.truth_digest, baseline_bundle.truth_digest);
    assert_eq!(
        rebuilt_bundle.artifact_digest,
        baseline_bundle.artifact_digest
    );
    assert_eq!(
        rebuilt_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        rebuilt_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        rebuilt_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_ne!(
        rebuilt_bundle.diagnostics_digest,
        degraded_bundle.diagnostics_digest
    );
}

#[test]
fn milestone_6_authority_rebuild_restores_materializations_and_derived_access_structures_from_publication_seed(
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("worth-store-m6-authority-rebuild");

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let baseline_bundle = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let degraded_bundle = reopened
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    assert_eq!(degraded_bundle.truth_digest, baseline_bundle.truth_digest);
    assert_ne!(
        degraded_bundle.artifact_digest,
        baseline_bundle.artifact_digest
    );
    assert_eq!(
        degraded_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(
        degraded_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(
        degraded_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Debt
    );

    let rebuild = reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    assert_eq!(rebuild.layout_materialization_count(), 1);
    assert_eq!(rebuild.scope_membership_count(), 1);
    assert_eq!(rebuild.structural_block_count(), 1);
    assert_eq!(rebuild.chunk_membership_count(), 1);

    let rebuilt_bundle = reopened.milestone_6_certification_bundle(request).unwrap();
    assert_eq!(rebuilt_bundle.truth_digest, baseline_bundle.truth_digest);
    assert_eq!(
        rebuilt_bundle.artifact_digest,
        baseline_bundle.artifact_digest
    );
    assert_eq!(
        rebuilt_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        rebuilt_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        rebuilt_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_ne!(
        rebuilt_bundle.diagnostics_digest,
        degraded_bundle.diagnostics_digest
    );
}
