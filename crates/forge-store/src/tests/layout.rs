use crate::{
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet,
    AspectReadRegime, AspectScopeClass, CdcTouchedAspectScope, ComplexityStatus,
    EntitySetUniformAspectScope, ForgeStoreBuilder, SingleEntityAspectScope,
};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch},
    stores::{build_store_for_lane, unique_test_sqlite_path, unique_test_store_path, StoreLane},
};
use super::harness::corruption::local_file::{
    force_clear_milestone_6_materializations_and_derived_access_structures,
    force_milestone_6_commit_support_summary_seed_gap,
    force_milestone_6_commit_coupled_layout_seed_authority_digest_drift,
    force_milestone_6_commit_coupled_layout_seed_payload_drift,
    force_milestone_6_commit_coupled_layout_seed_payload_gap,
    force_milestone_6_chunk_membership_boundary_drift,
    force_milestone_6_layout_materialization_chunk_member_count_drift,
    force_milestone_6_layout_materialization_key_mismatch,
};
use super::harness::corruption::sqlite::simulate_legacy_milestone_6_commit_coupled_layout_seed_storage;

fn store_with_root_commit() -> (
    crate::ForgeStore,
    forge_relational::facade::history::BranchId,
    forge_relational::facade::history::CommitId,
) {
    store_with_root_commit_for_lane(StoreLane::InMemory)
}

fn store_with_root_commit_for_lane(
    lane: StoreLane,
) -> (
    crate::ForgeStore,
    forge_relational::facade::history::BranchId,
    forge_relational::facade::history::CommitId,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;

    let mut store = match lane {
        StoreLane::InMemory => ForgeStoreBuilder::new().in_memory().build().unwrap(),
        _ => build_store_for_lane(lane, &format!("layout-{}", lane.label())),
    };
    store.append_canonical_commit(root).unwrap();
    (store, branch_id, commit_id)
}

fn admitted_plan(
    store: &crate::ForgeStore,
    request: AspectLayoutReadRequest,
) -> crate::AdmittedAspectLayoutReadPlan {
    match store.plan_aspect_layout_read(request).unwrap() {
        AspectLayoutReadPlanDecision::Admitted(plan) => plan,
        other => panic!("expected admitted plan, got {other:?}"),
    }
}

fn entity_set_request(
    branch_id: forge_relational::facade::history::BranchId,
    commit_id: forge_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    )
}

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

    assert_eq!(plan.performance().strategy, AspectReadRegime::DirectLayoutSlice);
    assert_eq!(plan.performance().complexity_status, ComplexityStatus::Verified);
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

    assert_eq!(plan.performance().strategy, AspectReadRegime::BlockReuseBacked);
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
fn milestone_6_layout_materialization_fails_reopen_when_persisted_key_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-materialization-corrupt");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_layout_materialization_key_mismatch(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::BackendIntegrityViolation);
    assert!(error
        .message()
        .contains("milestone 6 layout materialization map key"));
}

#[test]
fn milestone_6_layout_materialization_fails_reopen_when_payload_witness_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-materialization-payload-corrupt");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_layout_materialization_chunk_member_count_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::BackendIntegrityViolation);
    assert!(error
        .message()
        .contains("canonical Milestone 9 physical chunk reference"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_authority_basis_digest_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-authority-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_authority_digest_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::BackendIntegrityViolation);
    assert!(error
        .message()
        .contains("authority basis digest"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_commit_support_summary_loses_seed_link() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-summary-gap");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_support_summary_seed_gap(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::CommitSupportPublicationGap);
    assert!(error
        .message()
        .contains("commit-coupled layout seed set"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_payload_support_artifact_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-payload-gap");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_payload_gap(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::CommitSupportPublicationGap);
    assert!(error
        .message()
        .contains("commit-coupled layout seed"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_payload_body_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-payload-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_payload_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::CommitSupportPublicationGap);
    assert!(error
        .message()
        .contains("non-canonical milestone 6 commit-coupled layout seed"));
}

#[test]
fn milestone_6_chunk_membership_fails_reopen_when_boundary_reference_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-chunk-membership-boundary-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_chunk_membership_boundary_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &crate::StoreErrorKind::BackendIntegrityViolation);
    assert!(error
        .message()
        .contains("chunk membership"));
}

#[test]
fn sqlite_legacy_commit_coupled_layout_seed_table_migrates_forward_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_sqlite_path("layout-legacy-seed-migration");

    let mut store = crate::ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    drop(store);

    simulate_legacy_milestone_6_commit_coupled_layout_seed_storage(&path);

    let reopened = crate::ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let fetched = reopened.fetch_milestone_6_layout_support(request).unwrap();
    assert_eq!(fetched, materialized);

    let connection = rusqlite::Connection::open(path).unwrap();
    let migrated_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM milestone_6_commit_coupled_layout_seed_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let legacy_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM milestone_6_published_layout_request_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(migrated_count, 1);
    assert_eq!(legacy_count, 1);
}

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
    let before_export = store.export_milestone_6_chunk_model(request.clone()).unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap();
    reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    let after_read = match reopened.execute_aspect_layout_read(request.clone()).unwrap() {
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
    assert_eq!(before_export.physical_chunk_id(), after_export.physical_chunk_id());
    assert_eq!(before_export.determinism_digest(), after_export.determinism_digest());
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

    assert_eq!(plan.performance().complexity_status, ComplexityStatus::Verified);
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
            assert_eq!(plan.performance().strategy, AspectReadRegime::ExplicitBroadFallback);
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

    assert!(matches!(decision, AspectLayoutReadPlanDecision::Fallback(_)));
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

    assert_eq!(read.plan(), materialized.admitted_plan());
    assert_eq!(
        read.layout_materialization_artifact_id(),
        materialized.artifact_id()
    );
    assert_eq!(
        read.scope_membership_artifact_id(),
        crate::layout::layout_scope_membership_artifact_id(materialized.admitted_plan().request())
            .unwrap()
    );
    assert_eq!(
        read.structural_block_artifact_id(),
        crate::layout::structural_block_artifact_id(
            materialized.block_reuse().structural_block_id()
        )
    );
    assert_eq!(
        read.chunk_membership_artifact_id(),
        crate::layout::chunk_membership_artifact_id(materialized.frozen_layout())
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
fn aspect_layout_control_truth_matches_admitted_execution_target() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch =
        forge_relational::facade::history::BranchId("layout-control-feature".to_string());

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
    assert_eq!(control.scope_class(), read.plan().request().scope_class().label());
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
        materialized.artifact_id()
    );
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
    let read = match store.execute_aspect_layout_read(AspectLayoutReadRequest::new(
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
            main_materialized.block_reuse().structural_block_id().clone(),
        ))
        .unwrap();
    assert_eq!(
        lookup.supporting_layout_materialization_artifact_ids().len(),
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
            assert_eq!(plan.performance().strategy, AspectReadRegime::ExplicitBroadFallback);
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

        assert_eq!(plan.performance().complexity_status, ComplexityStatus::Verified);
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
