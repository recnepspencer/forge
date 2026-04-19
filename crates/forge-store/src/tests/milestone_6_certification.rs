use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    EntitySetUniformAspectScope, ForgeStore, ForgeStoreBuilder, Milestone6LayoutSupportLane,
    Milestone6LayoutSupportPolicy, Milestone6ResolvedLayoutSupportLane, SingleEntityAspectScope,
    StoreErrorKind,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::replay::CanonicalCommitEnvelope;

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST},
    },
    corruption::local_file::force_clear_milestone_6_derived_access_structures,
    corruption::local_file::force_clear_milestone_6_materializations_and_derived_access_structures,
    corruption::local_file::force_milestone_6_chunk_membership_boundary_drift,
    corruption::local_file::force_milestone_6_commit_support_summary_seed_gap,
    corruption::local_file::force_milestone_6_layout_materialization_chunk_member_count_drift,
    corruption::sqlite::simulate_legacy_milestone_6_commit_coupled_layout_seed_storage,
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{
            build_store_for_lane, unique_test_sqlite_path, unique_test_store_path, StoreLane,
        },
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

fn rebuild_identity_surface(bundle: &crate::Milestone6CertificationBundle) -> serde_json::Value {
    serde_json::json!({
        "truth_digest": bundle.truth_digest,
        "artifact_digest": bundle.artifact_digest,
        "structural_block_id": bundle.physical_layout_report.structural_block_id,
        "physical_chunk_id": bundle.physical_layout_report.physical_chunk_id,
        "determinism_digest": bundle.physical_layout_report.determinism_digest,
    })
}

fn chunk_export_surface(export: &crate::Milestone6ChunkModelExport) -> serde_json::Value {
    serde_json::json!({
        "physical_chunk_id": export.physical_chunk_id().as_str(),
        "chunk_membership_artifact_id": export.chunk_membership_artifact_id(),
        "determinism_digest": export.determinism_digest(),
        "chunk_member_count": export.chunk_member_count(),
        "layout_materialization_artifact_id": export.layout_materialization_artifact_id(),
    })
}

fn execution_surface(
    read: &crate::AspectLayoutReadExecutionResult,
    dedup: &crate::DedupBackedReadResult,
) -> serde_json::Value {
    serde_json::json!({
        "request_scope_class": read.plan().request().scope_class().label(),
        "layout_materialization_artifact_id": read.layout_materialization_artifact_id(),
        "scope_membership_artifact_id": read.scope_membership_artifact_id(),
        "chunk_membership_artifact_id": read.chunk_membership_artifact_id(),
        "semantic_truth_digest": read.semantic_truth_digest(),
        "authoritative_commit_count": read.authoritative_commit_count(),
        "structural_block_id": dedup.structural_block_lookup().structural_block_id().as_str(),
        "structural_block_slice_ids": dedup.structural_block_lookup().slice_ids(),
    })
}

fn overlap_branch_parity_surface(
    read: &crate::AspectLayoutReadExecutionResult,
    dedup: &crate::DedupBackedReadResult,
    control: &crate::AspectLayoutControlTruth,
) -> serde_json::Value {
    serde_json::json!({
        "execution_branch_id": read.plan().request().target().branch_id().0,
        "execution_frontier_commit_id": read.plan().request().target().frontier_commit_id().0,
        "execution_scope_class": read.plan().request().scope_class().label(),
        "execution_slice_ids": read.plan().slice_ids(),
        "execution_semantic_truth_digest": read.semantic_truth_digest(),
        "execution_authoritative_commit_count": read.authoritative_commit_count(),
        "dedup_semantic_truth_digest": dedup.read().semantic_truth_digest(),
        "dedup_authoritative_commit_count": dedup.read().authoritative_commit_count(),
        "dedup_structural_block_id": dedup.structural_block_lookup().structural_block_id().as_str(),
        "dedup_slice_ids": dedup.structural_block_lookup().slice_ids(),
        "control_branch_id": control.branch_id().0,
        "control_frontier_commit_id": control.frontier_commit_id().0,
        "control_scope_class": control.scope_class(),
        "control_authoritative_truth_digest": control.authoritative_truth_digest(),
        "control_authoritative_commit_count": control.authoritative_commit_count(),
    })
}

fn canonical_row_by_name<'a, T: Eq + serde::Serialize, E: Eq + serde::Serialize>(
    suite: &'a CertificationSuite<T, E>,
    name: &str,
) -> &'a CanonicalRow<T> {
    suite
        .canonical_rows()
        .iter()
        .find(|row| row.name() == name)
        .unwrap_or_else(|| panic!("missing canonical row `{name}`"))
}

fn rejection_row_by_name<'a, T: Eq + serde::Serialize, E: Eq + serde::Serialize>(
    suite: &'a CertificationSuite<T, E>,
    name: &str,
) -> &'a RejectionRow<E> {
    suite
        .rejection_rows()
        .iter()
        .find(|row| row.name() == name)
        .unwrap_or_else(|| panic!("missing rejection row `{name}`"))
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

    let admitted_artifact_parity = [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let bundle = entity_set_bundle_for_lane(lane);
            LaneResult::new(lane.label(), bundle.artifact_digest.clone())
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

    let authority_rebuild_parity = {
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
        let path = unique_test_store_path("forge-store-m6-suite-authority-rebuild");
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request.clone())
            .unwrap();
        let before = store
            .milestone_6_certification_bundle(request.clone())
            .unwrap();
        drop(store);

        force_clear_milestone_6_materializations_and_derived_access_structures(&path);

        let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
        reopened
            .rebuild_milestone_6_derived_artifacts_from_authority()
            .unwrap();
        let after = reopened.milestone_6_certification_bundle(request).unwrap();
        vec![
            LaneResult::new(
                "before_rebuild",
                serde_json::to_string(&rebuild_identity_surface(&before)).unwrap(),
            ),
            LaneResult::new(
                "after_rebuild",
                serde_json::to_string(&rebuild_identity_surface(&after)).unwrap(),
            ),
        ]
    };

    let chunk_export_rebuild_parity = {
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
        let path = unique_test_store_path("forge-store-m6-suite-chunk-export");
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request.clone())
            .unwrap();
        let before = store
            .export_milestone_6_chunk_model(request.clone())
            .unwrap();
        drop(store);

        force_clear_milestone_6_materializations_and_derived_access_structures(&path);

        let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
        reopened
            .rebuild_milestone_6_derived_artifacts_from_authority()
            .unwrap();
        let after = reopened.export_milestone_6_chunk_model(request).unwrap();
        vec![
            LaneResult::new(
                "before_rebuild",
                serde_json::to_string(&chunk_export_surface(&before)).unwrap(),
            ),
            LaneResult::new(
                "after_rebuild",
                serde_json::to_string(&chunk_export_surface(&after)).unwrap(),
            ),
        ]
    };

    let authority_rebuild_execution_parity = {
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
        let path = unique_test_store_path("forge-store-m6-suite-rebuild-execution");
        let mut store = ForgeStoreBuilder::new()
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
        drop(store);

        force_clear_milestone_6_materializations_and_derived_access_structures(&path);

        let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
        let after_dedup = reopened.execute_dedup_backed_read(request).unwrap();
        vec![
            LaneResult::new(
                "before_rebuild",
                serde_json::to_string(&execution_surface(&before_read, &before_dedup)).unwrap(),
            ),
            LaneResult::new(
                "after_rebuild",
                serde_json::to_string(&execution_surface(&after_read, &after_dedup)).unwrap(),
            ),
        ]
    };

    let sqlite_legacy_seed_migration_parity = {
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
        let path = unique_test_sqlite_path("forge-store-m6-suite-legacy-seed");
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request.clone())
            .unwrap();
        let before = store
            .milestone_6_certification_bundle(request.clone())
            .unwrap();
        drop(store);

        simulate_legacy_milestone_6_commit_coupled_layout_seed_storage(&path);

        let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
        let after = reopened.milestone_6_certification_bundle(request).unwrap();
        vec![
            LaneResult::new(
                "before_migration",
                serde_json::to_string(&rebuild_identity_surface(&before)).unwrap(),
            ),
            LaneResult::new(
                "after_migration",
                serde_json::to_string(&rebuild_identity_surface(&after)).unwrap(),
            ),
        ]
    };

    let dedup_control_overlap_branch_parity =
        [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
            .into_iter()
            .map(|lane| {
                let mut runtime = runtime_with_demo_schema();
                let entity_id = create_entity(&mut runtime, "alpha");
                let root = latest_envelope(&runtime);
                update_entity_on_branch(&mut runtime, entity_id, "beta", None);
                let main_head = latest_envelope(&runtime);
                let main_branch = main_head.branch_context.clone();
                let feature_branch = BranchId("m6-suite-dedup-feature".to_string());

                let mut store = match lane {
                    StoreLane::InMemory => ForgeStoreBuilder::new().in_memory().build().unwrap(),
                    _ => build_store_for_lane(
                        lane,
                        &format!("milestone-6-dedup-overlap-{}", lane.label()),
                    ),
                };
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
                let request = request_for_scope(
                    &feature_head,
                    AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                        "entity-a".to_string(),
                        "entity-b".to_string(),
                    ])),
                    &["profile", "status"],
                );
                store.append_canonical_commit(feature_head).unwrap();
                store
                    .materialize_milestone_6_layout_support(request.clone())
                    .unwrap();

                let aspect_read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
                    crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
                    other => panic!("expected admitted overlap execution result, got {other:?}"),
                };
                let dedup_read = store.execute_dedup_backed_read(request.clone()).unwrap();
                let control = store.read_aspect_layout_control_truth(request).unwrap();
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
                LaneResult::new(
                    lane.label(),
                    serde_json::to_string(&overlap_branch_parity_surface(
                        &aspect_read,
                        &dedup_read,
                        &control,
                    ))
                    .unwrap(),
                )
            })
            .collect::<Vec<_>>();

    let commit_coupled_seed_corruption = {
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
        let path = unique_test_store_path("forge-store-m6-suite-seed-corruption");
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request)
            .unwrap();
        drop(store);

        force_milestone_6_commit_support_summary_seed_gap(&path);
        let error = ForgeStoreBuilder::new()
            .local_file(path)
            .build()
            .unwrap_err();
        vec![LaneResult::new(
            "commit_coupled_seed_gap",
            serde_json::to_string(&serde_json::json!({
                "error_kind": format!("{:?}", error.kind()),
                "message": error.message(),
            }))
            .unwrap(),
        )]
    };

    let chunk_export_corruption = {
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
        let path = unique_test_store_path("forge-store-m6-suite-chunk-corruption");
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request)
            .unwrap();
        drop(store);

        force_milestone_6_layout_materialization_chunk_member_count_drift(&path);
        let error = ForgeStoreBuilder::new()
            .local_file(path)
            .build()
            .unwrap_err();
        vec![LaneResult::new(
            "chunk_member_drift",
            serde_json::to_string(&serde_json::json!({
                "error_kind": format!("{:?}", error.kind()),
                "message": error.message(),
            }))
            .unwrap(),
        )]
    };

    let chunk_export_boundary_mismatch = {
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
        let path = unique_test_store_path("forge-store-m6-suite-chunk-boundary");
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store
            .materialize_milestone_6_layout_support(request)
            .unwrap();
        drop(store);

        force_milestone_6_chunk_membership_boundary_drift(&path);
        let error = ForgeStoreBuilder::new()
            .local_file(path)
            .build()
            .unwrap_err();
        vec![LaneResult::new(
            "chunk_boundary_drift",
            serde_json::to_string(&serde_json::json!({
                "error_kind": format!("{:?}", error.kind()),
                "message": error.message(),
            }))
            .unwrap(),
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
            "admitted_layout_artifact_parity",
            admitted_artifact_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "authority_rebuild_preserves_layout_identity",
            authority_rebuild_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "authority_rebuild_preserves_execution_surfaces",
            authority_rebuild_execution_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "dedup_control_overlap_branch_parity",
            dedup_control_overlap_branch_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "chunk_export_rebuild_parity",
            chunk_export_rebuild_parity,
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "sqlite_legacy_seed_migration_parity",
            sqlite_legacy_seed_migration_parity,
            &[AssertionClass::Equality],
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
        .with_rejection_row(RejectionRow::new(
            "commit_coupled_seed_corruption_requires_typed_failure",
            commit_coupled_seed_corruption,
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "chunk_export_corruption_requires_typed_failure",
            chunk_export_corruption,
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "chunk_export_boundary_mismatch_requires_typed_failure",
            chunk_export_boundary_mismatch,
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_6_certification_harness_scaffolds_layout_suite() {
    let suite = milestone_6_suite();
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_truth_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_counter_contract_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_artifact_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "authority_rebuild_preserves_layout_identity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "authority_rebuild_preserves_execution_surfaces",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "dedup_control_overlap_branch_parity",
    ));
    assert_all_equal(canonical_row_by_name(&suite, "chunk_export_rebuild_parity"));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "sqlite_legacy_seed_migration_parity",
    ));
    assert_any_not_equal(canonical_row_by_name(
        &suite,
        "scope_shape_changes_physical_truth",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "generalized_scope_requires_explicit_fallback",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "commit_coupled_seed_corruption_requires_typed_failure",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "chunk_export_corruption_requires_typed_failure",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "chunk_export_boundary_mismatch_requires_typed_failure",
    ));
    let completeness = evaluate_completeness(&suite, &ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_6_certification_bundle_is_backend_stable() {
    let mut truth_digests = Vec::new();
    let mut artifact_digests = Vec::new();
    let mut diagnostics_digests = Vec::new();
    let mut chunk_digests = Vec::new();

    for lane in [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite] {
        let bundle = entity_set_bundle_for_lane(lane);
        assert_eq!(
            bundle.requested_layout_support_lane,
            Milestone6LayoutSupportLane::ProofOnly
        );
        assert_eq!(
            bundle.resolved_layout_support_lane,
            Milestone6ResolvedLayoutSupportLane::ProofOnly
        );
        truth_digests.push((lane.label(), bundle.truth_digest.clone()));
        artifact_digests.push((lane.label(), bundle.artifact_digest.clone()));
        diagnostics_digests.push((lane.label(), bundle.diagnostics_digest.clone()));
        chunk_digests.push((
            lane.label(),
            bundle.physical_layout_report.determinism_digest.clone(),
        ));
        assert_eq!(bundle.layout_read_report.scope_class, "entity_set_uniform");
        assert_eq!(
            bundle.physical_layout_report.branch_id,
            BranchId("main".to_string())
        );
        assert_eq!(
            bundle.physical_layout_report.milestone_9_chunk_member_count,
            2
        );
        assert!(bundle
            .access_structure_contract
            .aspect_layout_read
            .access_structure
            .contains("scope-to-slice membership records"));
        assert!(
            !bundle
                .access_structure_verification
                .aspect_layout_read
                .verified_at_open
        );
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
        assert_eq!(
            bundle
                .counter_contract
                .structural_block_reuse_admission_count,
            1
        );
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
    let first_artifact = &artifact_digests[0].1;
    assert!(artifact_digests
        .iter()
        .all(|(_, digest)| digest == first_artifact));
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
        direct_bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        direct_bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        direct_bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
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
    let fetched = reopened
        .fetch_milestone_6_layout_support(request.clone())
        .unwrap();
    let reopened_bundle = reopened.milestone_6_certification_bundle(request).unwrap();
    assert_eq!(
        reopened_bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        reopened_bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        reopened_bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
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
    assert_eq!(
        direct_bundle.artifact_digest,
        reopened_bundle.artifact_digest
    );
    assert_ne!(
        direct_bundle.diagnostics_digest,
        reopened_bundle.diagnostics_digest
    );
    assert_eq!(
        direct_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle
            .complexity_status
            .structural_block_reuse
            .status,
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
        reopened_bundle
            .complexity_status
            .structural_block_reuse
            .status,
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
    assert_ne!(single.artifact_digest, entity_set.artifact_digest);
    assert_eq!(single.layout_read_report.scope_class, "single_entity");
    assert_eq!(
        entity_set.layout_read_report.scope_class,
        "entity_set_uniform"
    );
    assert_eq!(
        single.physical_layout_report.milestone_9_chunk_member_count,
        1
    );
    assert_eq!(
        entity_set
            .physical_layout_report
            .milestone_9_chunk_member_count,
        2
    );
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
    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
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
fn milestone_6_explicit_proof_only_certification_stays_debt_even_when_materialized_support_exists()
{
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

    let bundle = store
        .milestone_6_certification_bundle_in_lane(request, Milestone6LayoutSupportLane::ProofOnly)
        .unwrap();

    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(
        bundle.certification_origin,
        crate::Milestone6CertificationOrigin::ReconstructedWitness
    );
    assert_eq!(
        bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
}

#[test]
fn milestone_6_explicit_on_demand_certification_materializes_support_when_missing() {
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

    let bundle = store
        .milestone_6_certification_bundle_in_lane(
            request.clone(),
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap();
    let fetched = store.fetch_milestone_6_layout_support(request).unwrap();

    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(
        bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        bundle
            .layout_materialization_report
            .as_ref()
            .map(|report| report.artifact_id.as_str()),
        Some(fetched.artifact_id())
    );
    assert_eq!(
        bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 0);
}

#[test]
fn milestone_6_policy_eager_certification_resolving_to_proof_only_stays_debt() {
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

    let bundle = store
        .milestone_6_certification_bundle_in_lane_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap();

    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::PolicyEagerMaterialized
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
    assert_eq!(
        bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Debt
    );
}

#[test]
fn milestone_6_policy_eager_certification_resolving_to_materialized_is_verified() {
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

    let proof_only_bundle = store
        .milestone_6_certification_bundle_in_lane_with_policy(
            request.clone(),
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap();
    let materialized_bundle = store
        .milestone_6_certification_bundle_in_lane_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap();

    assert_eq!(
        materialized_bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::PolicyEagerMaterialized
    );
    assert_eq!(
        materialized_bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
    );
    assert_eq!(
        materialized_bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(
        materialized_bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        materialized_bundle
            .complexity_status
            .aspect_layout_read
            .status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(materialized_bundle.certification_summary.debt_path_count, 0);
    assert_eq!(
        proof_only_bundle.truth_digest,
        materialized_bundle.truth_digest
    );
    assert_ne!(
        proof_only_bundle.diagnostics_digest,
        materialized_bundle.diagnostics_digest
    );
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
    let path = unique_test_store_path("forge-store-m6-derived-rebuild");

    let mut store = ForgeStoreBuilder::new()
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

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
    let path = unique_test_store_path("forge-store-m6-authority-rebuild");

    let mut store = ForgeStoreBuilder::new()
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

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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
