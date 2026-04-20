use super::*;

fn run_mixed_pressure_matrix(lane: DurableLaneCase) {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = lane.build();
    store.append_canonical_commit(initial.clone()).unwrap();

    let main_head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2", None);
    store.append_canonical_commit(main_head.clone()).unwrap();

    let feature = BranchId("feature".to_string());
    runtime
        .history_authority()
        .create_branch(feature.clone(), &main_head.branch_context)
        .unwrap();
    store
        .create_branch(feature.clone(), Some(&main_head.branch_context))
        .unwrap();

    let feature_head = update_entity_on_branch_with_commit(
        &mut runtime,
        entity_id,
        "feature-v1",
        Some(feature.clone()),
    );
    store.append_canonical_commit(feature_head.clone()).unwrap();

    let main_reclaimable_head =
        update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v3", None);
    store
        .append_canonical_commit(main_reclaimable_head.clone())
        .unwrap();

    let protected_main_request =
        layout_request(main_head.branch_context.clone(), main_head.commit.commit_id);
    let reclaimable_main_request = layout_request(
        main_reclaimable_head.branch_context.clone(),
        main_reclaimable_head.commit.commit_id,
    );
    let protected_main_materialization = store
        .materialize_milestone_6_layout_support(protected_main_request.clone())
        .unwrap();
    let reclaimable_main_materialization = store
        .materialize_milestone_6_layout_support(reclaimable_main_request.clone())
        .unwrap();

    let stable_basis = store
        .read_stable_basis(stable_basis_request(
            &store,
            main_head.branch_context.clone(),
            main_head.commit.commit_id,
        ))
        .unwrap();
    let stable_basis_id = stable_basis.stable_basis_id().clone();

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            main_head.branch_context.clone(),
            main_head.commit.commit_id,
        ))
        .unwrap();

    let control = store.export_authoritative_records();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    assert!(planning.compaction_plans().iter().any(|plan| plan
        .family_labels()
        .iter()
        .any(|label| label == "snapshot_family")));
    assert!(planning.compaction_plans().iter().any(|plan| {
        plan.family_labels()
            .iter()
            .any(|label| label == "milestone_6_layout_materialization")
            && plan
                .superseded_families()
                .iter()
                .any(|family| family.artifact_id() == protected_main_materialization.artifact_id())
    }));
    assert!(planning.compaction_plans().iter().any(|plan| {
        plan.family_labels()
            .iter()
            .any(|label| label == "milestone_6_layout_materialization")
            && plan.superseded_families().iter().any(|family| {
                family.artifact_id() == reclaimable_main_materialization.artifact_id()
            })
    }));
    let reclaim_candidates = planning
        .reclaim_candidates()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let reclaim_candidate_ids = reclaim_candidates
        .iter()
        .map(|witness| witness.artifact_id().to_string())
        .collect::<Vec<_>>();
    assert!(
        reclaim_candidate_ids.contains(&reclaimable_main_materialization.artifact_id().to_string())
    );
    assert!(
        !reclaim_candidate_ids.contains(&protected_main_materialization.artifact_id().to_string())
    );

    for plan in planning.compaction_plans().iter().cloned() {
        let publication = store.publish_compaction_product(plan).unwrap();
        store
            .verify_compaction_product(publication.product().clone())
            .unwrap();
        store
            .cutover_compaction_product(publication.product().clone())
            .unwrap();
    }

    let protected_error = store
        .execute_derived_reclaim(crate::ReclaimEligibilityWitness::new(
            "milestone_6_layout_materialization",
            protected_main_materialization.artifact_id().to_string(),
            format!(
                "branch:{}@{}",
                main_head.branch_context.0, main_head.commit.commit_id.0
            ),
        ))
        .unwrap_err();
    assert_eq!(
        protected_error.kind(),
        &StoreErrorKind::ReclaimLiveBasisConflict
    );

    let reclaim = store
        .execute_derived_reclaim(
            reclaim_candidates
                .iter()
                .find(|witness| {
                    witness.artifact_id() == reclaimable_main_materialization.artifact_id()
                })
                .cloned()
                .expect("reclaimable main layout reclaim witness"),
        )
        .unwrap();
    assert!(store
        .fetch_milestone_6_layout_support(protected_main_request.clone())
        .is_ok());
    assert!(store
        .fetch_milestone_6_layout_support(reclaimable_main_request.clone())
        .is_err());
    assert!(store.fetch_stable_basis(&stable_basis_id).is_ok());
    assert!(store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            main_head.commit.commit_id,
        ))
        .is_ok());
    assert!(store.counters().reclaim_rejected_live_basis_count >= 1);

    drop(store);

    let mut reopened = lane.build();
    assert!(reopened.fetch_stable_basis(&stable_basis_id).is_ok());
    assert!(reopened
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            main_head.commit.commit_id,
        ))
        .is_ok());
    assert!(reopened
        .fetch_milestone_6_layout_support(protected_main_request.clone())
        .is_ok());
    assert!(reopened
        .fetch_milestone_6_layout_support(reclaimable_main_request.clone())
        .is_err());

    reopened
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();
    assert!(reopened
        .fetch_milestone_6_layout_support(protected_main_request)
        .is_ok());
    assert!(reopened
        .fetch_milestone_6_layout_support(reclaimable_main_request)
        .is_ok());

    let bundle = reopened
        .milestone_10_certification_bundle(&control)
        .unwrap();
    assert!(
        bundle.certification_summary.truth_matches_control_lane,
        "{} truth drifted from control lane",
        lane.label()
    );
    assert!(
        bundle.certification_summary.restore_truth_parity,
        "{} restore parity drifted",
        lane.label()
    );
    assert!(
        bundle
            .certification_summary
            .no_unverified_compaction_products,
        "{} still had unverified compaction products",
        lane.label()
    );
    assert!(
        bundle.certification_summary.no_uncleared_rebuild_debt,
        "{} still had rebuild debt after rebuild",
        lane.label()
    );
}

#[test]
fn mixed_pressure_retention_matrix_survives_local_file_restart() {
    run_mixed_pressure_matrix(DurableLaneCase::LocalFile(unique_test_store_path(
        "forge-store-m10-stress-local",
    )));
}

#[test]
fn mixed_pressure_retention_matrix_survives_sqlite_restart() {
    run_mixed_pressure_matrix(DurableLaneCase::Sqlite(unique_test_sqlite_path(
        "forge-store-m10-stress-sqlite",
    )));
}
