use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    EntitySetUniformAspectScope, ForgeStore, ForgeStoreBuilder, SingleEntityAspectScope,
    StoreErrorKind,
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;
use forge_relational::facade::history::BranchId;

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST},
    },
    fixtures::{
        runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
        stores::{build_store_for_lane, unique_test_store_path, StoreLane},
    },
};

fn store_for_lane_with_root(
    lane: StoreLane,
    suffix: &str,
) -> (ForgeStore, CanonicalCommitEnvelope) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let mut store = match lane {
        StoreLane::InMemory => ForgeStoreBuilder::new().in_memory().build().unwrap(),
        _ => build_store_for_lane(lane, &format!("milestone-6-{suffix}-{}", lane.label())),
    };
    store.append_canonical_commit(root.clone()).unwrap();
    (store, root)
}

fn request_for_scope(
    root: &CanonicalCommitEnvelope,
    scope_class: AspectScopeClass,
    projection_names: &[&str],
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(root.branch_context.clone(), root.commit.commit_id),
        scope_class,
        AspectProjectionSet::new(
            projection_names
                .iter()
                .map(|name| (*name).to_string())
                .collect(),
        ),
    )
}

fn admitted_request_for_lane(lane: StoreLane) -> (ForgeStore, AspectLayoutReadRequest) {
    let (store, root) = store_for_lane_with_root(lane, "admitted");
    (
        store,
        request_for_scope(
            &root,
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-a".to_string(),
                "entity-b".to_string(),
            ])),
            &["profile", "status"],
        ),
    )
}

fn single_entity_bundle_for_lane(lane: StoreLane) -> crate::Milestone6CertificationBundle {
    let (store, root) = store_for_lane_with_root(lane, "single-entity");
    store
        .milestone_6_certification_bundle(request_for_scope(
            &root,
            AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
            &["profile", "status"],
        ))
        .unwrap()
}

fn entity_set_bundle_for_lane(lane: StoreLane) -> crate::Milestone6CertificationBundle {
    let (store, request) = admitted_request_for_lane(lane);
    store.milestone_6_certification_bundle(request).unwrap()
}

fn canonical_truth(bundle: &crate::Milestone6CertificationBundle) -> serde_json::Value {
    serde_json::json!({
        "truth_digest": bundle.truth_digest,
        "layout_read_report": bundle.layout_read_report,
        "physical_layout_report": bundle.physical_layout_report,
    })
}

fn fallback_surface(
    error: &crate::StoreError,
    counters: crate::StoreCounterSnapshot,
) -> serde_json::Value {
    serde_json::json!({
        "error_kind": format!("{:?}", error.kind()),
        "aspect_layout_plan_count": counters.aspect_layout_plan_count,
        "aspect_layout_admitted_count": counters.aspect_layout_admitted_count,
        "aspect_layout_fallback_count": counters.aspect_layout_fallback_count,
        "aspect_layout_rejected_count": counters.aspect_layout_rejected_count,
    })
}

fn assert_complexity_debt(
    path: &crate::Milestone6ComplexityPathStatus,
    verification: &crate::Milestone6AccessStructureVerificationPath,
    expected_fragment: &str,
) {
    assert!(!verification.verified_at_open);
    assert!(verification
        .verification_gap
        .as_deref()
        .unwrap_or_default()
        .contains(expected_fragment));
    assert_eq!(path.status, crate::ComplexityStatus::Debt);
    assert!(path.proof_basis.is_none());
    assert!(path
        .debt_reason
        .as_deref()
        .unwrap_or_default()
        .contains(expected_fragment));
}

fn milestone_6_suite() -> CertificationSuite<String, String> {
    let admitted_truth_parity = [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(
                lane.label(),
                serde_json::to_string(&canonical_truth(&bundle)).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let admitted_counter_parity = [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(
                lane.label(),
                serde_json::to_string(&bundle.counter_contract).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let scope_shape_divergence = vec![
        LaneResult::new(
            "single_entity",
            serde_json::to_string(&canonical_truth(&single_entity_bundle_for_lane(
                StoreLane::InMemory,
            )))
            .unwrap(),
        ),
        LaneResult::new(
            "entity_set_uniform",
            serde_json::to_string(&canonical_truth(&entity_set_bundle_for_lane(
                StoreLane::InMemory,
            )))
            .unwrap(),
        ),
    ];

    let generalized_scope_rejection = {
        let (store, root) = store_for_lane_with_root(StoreLane::InMemory, "fallback");
        let error = store
            .milestone_6_certification_bundle(request_for_scope(
                &root,
                AspectScopeClass::Generalized {
                    descriptor: "wildcard-join".to_string(),
                },
                &["profile"],
            ))
            .unwrap_err();
        vec![LaneResult::new(
            "generalized_scope",
            serde_json::to_string(&fallback_surface(&error, store.counters())).unwrap(),
        )]
    };

    CertificationSuite::new(ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "admitted_layout_truth_parity",
            admitted_truth_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "admitted_layout_counter_contract_parity",
            admitted_counter_parity,
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "scope_shape_changes_physical_truth",
            scope_shape_divergence,
            &[AssertionClass::Inequality],
        ))
        .with_rejection_row(RejectionRow::new(
            "generalized_scope_requires_explicit_fallback",
            generalized_scope_rejection,
            &[AssertionClass::TypedFailure, AssertionClass::ExactCounter],
        ))
}

#[test]
fn milestone_6_certification_harness_scaffolds_layout_suite() {
    let suite = milestone_6_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    assert_any_not_equal(&suite.canonical_rows()[2]);
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness = evaluate_completeness(&suite, &ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_6_certification_bundle_is_backend_stable() {
    let mut truth_digests = Vec::new();
    let mut diagnostics_digests = Vec::new();
    let mut chunk_digests = Vec::new();

    for lane in [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite] {
        let bundle = entity_set_bundle_for_lane(lane);
        truth_digests.push((lane.label(), bundle.truth_digest.clone()));
        diagnostics_digests.push((lane.label(), bundle.diagnostics_digest.clone()));
        chunk_digests.push((
            lane.label(),
            bundle.physical_layout_report.determinism_digest.clone(),
        ));
        assert_eq!(bundle.layout_read_report.scope_class, "entity_set_uniform");
        assert_eq!(bundle.physical_layout_report.branch_id, BranchId("main".to_string()));
        assert_eq!(bundle.physical_layout_report.milestone_9_chunk_member_count, 2);
        assert!(bundle
            .access_structure_contract
            .aspect_layout_read
            .access_structure
            .contains("scope-to-slice membership records"));
        assert!(!bundle
            .access_structure_verification
            .aspect_layout_read
            .verified_at_open);
        assert_eq!(
            bundle.complexity_status.aspect_layout_read.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.structural_block_reuse.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.chunk_model_freeze.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.milestone_7_layout_reference.status,
            crate::ComplexityStatus::Verified
        );
        assert_eq!(
            bundle
                .complexity_status
                .milestone_9_physical_chunk_reference
                .status,
            crate::ComplexityStatus::Verified
        );
        assert!(bundle
            .complexity_status
            .aspect_layout_read
            .debt_reason
            .as_deref()
            .unwrap_or_default()
            .contains("proof-only"));
        assert_eq!(bundle.counter_contract.aspect_layout_plan_count, 1);
        assert_eq!(bundle.counter_contract.aspect_layout_admitted_count, 1);
        assert_eq!(bundle.counter_contract.aspect_layout_fallback_count, 0);
        assert_eq!(bundle.counter_contract.structural_block_reuse_admission_count, 1);
        assert_eq!(bundle.counter_contract.chunk_model_freeze_count, 1);
        assert_eq!(
            bundle
                .counter_contract
                .milestone_7_layout_reference_admission_count,
            1
        );
        assert_eq!(
            bundle
                .counter_contract
                .milestone_9_physical_chunk_reference_admission_count,
            1
        );
        assert_eq!(bundle.certification_summary.verified_path_count, 2);
        assert_eq!(bundle.certification_summary.debt_path_count, 3);
        assert!(bundle.certification_summary.fallback_free_admission);
        assert!(bundle.certification_summary.deterministic_chunk_freeze);
        assert!(bundle.certification_summary.milestone_7_boundary_isolated);
        assert!(bundle.certification_summary.milestone_9_boundary_isolated);
    }

    let first_truth = &truth_digests[0].1;
    assert!(truth_digests
        .iter()
        .all(|(_, digest)| digest == first_truth));
    let first_diagnostics = &diagnostics_digests[0].1;
    assert!(diagnostics_digests
        .iter()
        .any(|(_, digest)| digest != first_diagnostics));
    let first_chunk = &chunk_digests[0].1;
    assert!(chunk_digests
        .iter()
        .all(|(_, digest)| digest == first_chunk));
}

#[test]
fn milestone_6_certification_bundle_prefers_persisted_layout_materialization_when_present() {
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
    let path = unique_test_store_path("forge-store-m6-persisted-certification");

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let direct_bundle = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    assert_eq!(
        direct_bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        direct_bundle
            .layout_materialization_report
            .as_ref()
            .map(|report| report.artifact_id.as_str()),
        Some(materialized.artifact_id())
    );
    drop(store);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let fetched = reopened.fetch_milestone_6_layout_support(request.clone()).unwrap();
    let reopened_bundle = reopened.milestone_6_certification_bundle(request).unwrap();
    assert_eq!(
        reopened_bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        reopened_bundle
            .layout_materialization_report
            .as_ref()
            .map(|report| report.artifact_id.as_str()),
        Some(fetched.artifact_id())
    );

    assert_eq!(materialized, fetched);
    assert_eq!(direct_bundle.truth_digest, reopened_bundle.truth_digest);
    assert_ne!(direct_bundle.diagnostics_digest, reopened_bundle.diagnostics_digest);
    assert_eq!(
        direct_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle.complexity_status.structural_block_reuse.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .structural_block_reuse
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .aspect_layout_read
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .chunk_model_freeze
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .milestone_9_physical_chunk_reference
            .verified_at_open,
        true
    );
    assert_eq!(direct_bundle.certification_summary.verified_path_count, 5);
    assert_eq!(direct_bundle.certification_summary.debt_path_count, 0);
    assert_eq!(
        reopened_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        reopened_bundle.complexity_status.structural_block_reuse.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        reopened_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(reopened_bundle.certification_summary.verified_path_count, 5);
    assert_eq!(reopened_bundle.certification_summary.debt_path_count, 0);
    assert_eq!(
        reopened_bundle.physical_layout_report.structural_block_id,
        fetched
            .block_reuse()
            .structural_block_id()
            .as_str()
            .to_string()
    );
    assert_eq!(
        reopened_bundle.physical_layout_report.physical_chunk_id,
        fetched
            .milestone_9_reference()
            .physical_chunk_id()
            .as_str()
            .to_string()
    );
}

#[test]
fn milestone_6_certification_bundle_proves_scope_shape_changes_layout_truth() {
    let single = single_entity_bundle_for_lane(StoreLane::InMemory);
    let entity_set = entity_set_bundle_for_lane(StoreLane::InMemory);

    assert_ne!(single.truth_digest, entity_set.truth_digest);
    assert_eq!(single.layout_read_report.scope_class, "single_entity");
    assert_eq!(entity_set.layout_read_report.scope_class, "entity_set_uniform");
    assert_eq!(single.physical_layout_report.milestone_9_chunk_member_count, 1);
    assert_eq!(entity_set.physical_layout_report.milestone_9_chunk_member_count, 2);
    assert_ne!(
        single.physical_layout_report.structural_block_id,
        entity_set.physical_layout_report.structural_block_id
    );
}

#[test]
fn milestone_6_certification_rejects_non_admitted_generalized_scope() {
    let (store, root) = store_for_lane_with_root(StoreLane::InMemory, "rejection");
    let error = store
        .milestone_6_certification_bundle(request_for_scope(
            &root,
            AspectScopeClass::Generalized {
                descriptor: "wildcard-join".to_string(),
            },
            &["profile"],
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::AspectLayoutFallbackRequired);
    assert_eq!(store.counters().aspect_layout_plan_count, 1);
    assert_eq!(store.counters().aspect_layout_admitted_count, 0);
    assert_eq!(store.counters().aspect_layout_fallback_count, 1);
    assert_eq!(store.counters().aspect_layout_rejected_count, 0);
}

#[test]
fn milestone_6_live_certification_marks_unpublished_layout_paths_as_debt() {
    let bundle = entity_set_bundle_for_lane(StoreLane::InMemory);
    assert_complexity_debt(
        &bundle.complexity_status.aspect_layout_read,
        &bundle.access_structure_verification.aspect_layout_read,
        "proof-only",
    );
    assert_complexity_debt(
        &bundle.complexity_status.chunk_model_freeze,
        &bundle.access_structure_verification.chunk_model_freeze,
        "proof-only",
    );
    assert_complexity_debt(
        &bundle.complexity_status.structural_block_reuse,
        &bundle.access_structure_verification.structural_block_reuse,
        "proof-only",
    );
    assert_eq!(bundle.certification_summary.verified_path_count, 2);
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
}

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
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    assert_eq!(store.counters().aspect_layout_plan_count, 1);
    assert_eq!(store.counters().aspect_layout_admitted_count, 1);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);
    assert_eq!(store.counters().milestone_7_layout_reference_admission_count, 1);
    assert_eq!(
        store.counters().milestone_9_physical_chunk_reference_admission_count,
        1
    );

    store.fetch_milestone_6_layout_support(request).unwrap();
    assert_eq!(store.counters().aspect_layout_plan_count, 2);
    assert_eq!(store.counters().aspect_layout_admitted_count, 2);
    assert_eq!(store.counters().structural_block_reuse_admission_count, 1);
    assert_eq!(store.counters().chunk_model_freeze_count, 1);
}
