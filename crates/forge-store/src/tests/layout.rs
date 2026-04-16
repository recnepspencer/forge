use crate::{
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet,
    AspectReadRegime, AspectScopeClass, CdcTouchedAspectScope, ComplexityStatus,
    EntitySetUniformAspectScope, ForgeStoreBuilder, SingleEntityAspectScope,
};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{build_store_for_lane, unique_test_sqlite_path, unique_test_store_path, StoreLane},
};
use super::harness::corruption::local_file::{
    force_milestone_6_layout_materialization_chunk_member_count_drift,
    force_milestone_6_layout_materialization_key_mismatch,
};

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
