use super::super::*;

#[test]
fn milestone_11_certification_bundle_publishes_acceptance_artifacts() {
    let path = unique_test_store_path("worth-store-m11-certification-bundle");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(initial).unwrap();
    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let update = update_entity_on_branch_with_commit(&mut runtime, entity_id, "foreground-write");
    drop(store);

    force_local_file_recovered(&path, &declaration_id);
    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let _ = reopened.append_canonical_commit(update).unwrap();
    let control_export = reopened.export_authoritative_records();
    let bundle = reopened.milestone_11_certification_bundle(&control_export, &[]);

    assert!(!bundle.truth_digest.is_empty());
    assert!(!bundle.diagnostics_digest.is_empty());
    assert!(!bundle.failure_digest.is_empty());
    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.scheduler_topology_declared);
    assert_eq!(
        bundle.scheduler_topology_report.queue_family_count,
        bundle.maintenance_report.work_class_counts.len() as u64
    );
    assert!(bundle.scheduler_topology_report.queue_family_count > 0);
    assert!(
        bundle
            .scheduler_topology_report
            .has_foreground_reservation_pool
    );
    assert!(
        bundle
            .scheduler_topology_report
            .has_background_reservation_pool
    );
    assert_eq!(
        bundle.resource_budget_report.io_budget_units_reserved,
        bundle.counter_contract.maintenance_io_budget_units_reserved
    );
    let matrix_lane_names = bundle
        .maintenance_interference_matrix
        .iter()
        .map(|entry| entry.lane_name.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(bundle.maintenance_interference_matrix.len(), 9);
    for expected_lane in [
        "isolated",
        "hostile_backlog",
        "deferred",
        "escalated",
        "recovered",
        "coalesced",
        "freshness_rejected",
        "tier_pressure",
        "explicit_cross_locality_debt",
    ] {
        assert!(matrix_lane_names.contains(expected_lane));
    }
    assert!(bundle
        .maintenance_interference_matrix
        .iter()
        .all(|entry| entry.truth_visible_equal));
    assert!(bundle
        .maintenance_interference_matrix
        .iter()
        .any(|entry| entry.foreground_interference_count >= 1));
    assert!(
        bundle
            .certification_summary
            .cold_warm_scheduler_equivalence_reported
    );
    assert!(bundle.certification_summary.tier_pressure_contained);
    assert!(
        bundle
            .certification_summary
            .cross_locality_escalation_explicit
    );
    assert!(bundle.certification_summary.queue_timing_truth_parity);
    assert_eq!(bundle.debt_escalation_report.escalated_declaration_count, 1);
    assert_eq!(bundle.maintenance_report.recovered_declaration_count, 1);
}
