use super::*;

pub(super) fn counter_value(
    accounting: &crate::certification::PerformanceAccounting,
    name: &str,
) -> Option<u64> {
    accounting
        .counters
        .iter()
        .find(|counter| counter.name == name)
        .map(|counter| counter.value)
}

pub(super) fn assert_milestone_one_closeout_surface_integrity(report: &MilestoneOneCloseoutReport) {
    assert!(report.seeded_bootstrap.named_truth_validated);
    assert!(report.seeded_bootstrap.topology_validated);
    assert_eq!(report.topology_truth_digest.algorithm, "fnv1a64");
    assert_eq!(report.naming_truth_digest.algorithm, "fnv1a64");
    assert_eq!(report.topology_validation_digest.algorithm, "fnv1a64");
    assert!(report.topology_truth_digest.row_count >= 1);
    assert!(report.naming_truth_digest.row_count >= 1);
    assert!(report.topology_validation_digest.row_count >= 1);
    assert!(report.topology_validation_report.rows.len() >= 5);
    assert!(report.topology_localization_report.topology_entities.len() >= 1);
    assert!(report.topology_localization_report.topology_relations.len() >= 1);
    assert!(report.naming_attachment_report.fully_named);
    assert!(report.naming_attachment_report.attachments.len() >= 1);
    assert!(report
        .primitive_family_coverage_matrix
        .entries
        .iter()
        .all(|entry| entry.role_closure_complete));
    assert!(report
        .primitive_corpus_parity_report
        .entries
        .iter()
        .all(|entry| entry.parity_closure_complete));
    assert!(report
        .primitive_corpus_parity_report
        .entries
        .iter()
        .all(|entry| entry.branch_ids.iter().any(|branch| branch == "main")));
    assert!(report
        .primitive_corpus_parity_report
        .entries
        .iter()
        .all(|entry| entry.branch_ids.iter().any(|branch| branch == "feature")));
    assert!(report
        .admitted_range_sweep_report
        .rows
        .iter()
        .all(|row| row.sweep_closure_complete
            && row.out_of_class_case_count >= 1
            && row.out_of_class_rejection_count >= 1));
    assert!(report
        .topology_validation_report
        .rows
        .iter()
        .any(|row| row.source == "seeded_bootstrap"
            && row.family == "SeededBootstrap"
            && row.validator == "ownership"));
    assert!(report
        .validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "SeededBootstrap"
            && row.validator == "ownership"
            && row.passed_count >= 1));
    assert!(report
        .validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "WireBranch(k)"
            && row.validator == "vertex_disks"
            && row.passed_count >= 1));
    assert!(report
        .validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "SolidShell(f)"
            && row.validator == "shell_closure"
            && row.passed_count >= 1));
    assert!(report
        .validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "NmtEdgeFan(k)"
            && row.validator == "radial_rings"
            && row.passed_count >= 1));
    assert!(report.branch_local_topology_report.mainline_case_count >= 1);
    assert!(report.branch_local_topology_report.branch_local_case_count >= 1);
    assert!(report
        .branch_local_topology_report
        .branch_ids
        .iter()
        .any(|branch| branch == "main"));
    assert!(report
        .branch_local_topology_report
        .branch_ids
        .iter()
        .any(|branch| branch == "feature"));
    assert!(
        report
            .branch_local_topology_report
            .branch_local_closure_complete
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .replay_checked_case_count
            >= 1
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .replay_verified_case_count
            >= 1
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .branch_local_replay_checked_case_count
            >= 1
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .branch_local_replay_verified_case_count
            >= 1
    );
    assert_eq!(
        report
            .milestone_1_replay_parity_report
            .replay_mismatch_case_count,
        0
    );
    assert!(
        report
            .milestone_1_replay_parity_report
            .replay_closure_complete
    );
    assert!(report
        .rejection_class_report
        .rows
        .iter()
        .any(|row| row.family == "WireClosed(n)" && row.case_count >= 1));
    assert!(report
        .rejection_class_report
        .rows
        .iter()
        .any(|row| row.family == "WireBranch(k)"
            && row.rejection_class == "IllegalAdmittedTopology"
            && row.case_count >= 1));
    assert!(report
        .failure_locality_report
        .rows
        .iter()
        .any(|row| row.family == "WireClosed(n)"
            && row.role == "OutOfClass"
            && row.rejection_class == "OutOfClass"));
    assert!(report
        .failure_locality_report
        .rows
        .iter()
        .any(|row| row.family == "NmtEdgeFan(k)"
            && row.validator_family.as_deref() == Some("radial_rings")
            && row.rejection_class == "IllegalAdmittedTopology"));
}

pub(super) fn assert_milestone_one_closeout_bridge_and_corpus(report: &MilestoneOneCloseoutReport) {
    assert_eq!(
        report
            .seeded_bootstrap
            .topology_validation_report
            .rows
            .len(),
        5
    );
    assert!(
        !report
            .seeded_bootstrap
            .branch_local_topology_report
            .branch_local
    );
    assert!(
        !report
            .seeded_bootstrap
            .milestone_1_replay_parity_report
            .relational_replay_checked
    );
    assert_eq!(
        report
            .seeded_bootstrap
            .milestone_1_replay_parity_report
            .parity_status,
        ReplayParityStatus::NotChecked
    );
    assert_eq!(
        report
            .seeded_bootstrap
            .counters
            .topology_entity_upsert_count,
        11
    );
    assert_eq!(
        report
            .seeded_bootstrap
            .counters
            .topology_relation_upsert_count,
        14
    );
    assert_eq!(report.bridge_proof_report.proof_case_count, 7);
    assert!(report
        .bridge_proof_report
        .family_coverage_report
        .rows
        .iter()
        .all(|row| row.proof_complete
            && row.routed_case_count >= 1
            && row.historical_evaluation_count >= 1));
    assert!(report
        .bridge_family_coverage_report
        .rows
        .iter()
        .all(|row| row.proof_complete
            && row.routed_case_count >= 1
            && row.historical_evaluation_count >= 1));
    assert_eq!(
        report.bridge_family_coverage_report.rows,
        report.bridge_proof_report.family_coverage_report.rows
    );
    for family in [
        "WireOpen(n)",
        "WireClosed(n)",
        "WireBranch(k)",
        "SheetDisk(n)",
        "SheetPatch(f)",
        "SolidShell(f)",
        "NmtEdgeFan(k)",
    ] {
        assert!(report
            .bridge_proof_report
            .proved_families
            .iter()
            .any(|proved| proved == family));
    }
    assert!(report.bridge_proof_report.route_record_count >= 1);
    assert!(
        report
            .bridge_proof_report
            .historical_evaluation_record_count
            >= 1
    );
    assert_eq!(
        report
            .bridge_proof_report
            .bridge_trace_anchor
            .route_identities
            .len(),
        report.bridge_proof_report.route_record_count
    );
    assert_eq!(
        report
            .bridge_proof_report
            .bridge_trace_anchor
            .invalidation_identities
            .len(),
        report.bridge_proof_report.route_record_count
    );
    assert!(!report
        .bridge_proof_report
        .bridge_trace_anchor
        .snapshot_identities
        .is_empty());
    assert_eq!(
        report
            .bridge_proof_report
            .bridge_trace_anchor
            .historical_record_identities
            .len(),
        report
            .bridge_proof_report
            .historical_evaluation_record_count
    );
    assert!(report.bridge_proof_report.bridge_routing_digest.row_count >= 1);
    assert!(
        report
            .bridge_proof_report
            .bridge_historical_evaluation_digest
            .row_count
            >= 1
    );
    assert!(!report.primitive_corpus.cases.is_empty());
    assert!(!report.primitive_corpus.rejected_cases.is_empty());
    assert_eq!(report.illegal_topology_rejection_report.case_count, 7);
    assert_eq!(report.illegal_topology_rejection_report.cases.len(), 7);
    assert_eq!(
        report
            .milestone_1_counter_report
            .commit_boundary_rejection_count,
        7
    );
    assert!(
        report
            .milestone_1_counter_report
            .topology_entity_upsert_count
            >= 11
    );
    assert!(
        report
            .milestone_1_counter_report
            .topology_relation_upsert_count
            >= 14
    );
    assert!(
        report
            .milestone_1_counter_report
            .commit_boundary_validator_count
            >= 6
    );
    assert!(report
        .illegal_topology_rejection_report
        .cases
        .iter()
        .all(
            |case| case.rejection.rejection_class == "IllegalAdmittedTopology"
                || case.rejection.rejection_class == "InvariantFailure"
        ));
    for case_name in [
        "non_manifold_closed_shell",
        "illegal_wire_branch",
        "broken_loop_wiring",
        "broken_radial_ring",
        "open_boundary_solid_shell",
    ] {
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == case_name));
    }
    assert!(
        report
            .illegal_topology_rejection_report
            .rejection_digest
            .row_count
            >= 7
    );
    assert!(report
        .primitive_corpus
        .coverage_matrix
        .entries
        .iter()
        .all(|entry| entry.role_closure_complete));
    assert!(report
        .primitive_corpus
        .parity_report
        .entries
        .iter()
        .all(|entry| entry.parity_closure_complete));

    let requirements = milestone_one_closeout_requirements();
    for family in &requirements.required_family_rows {
        assert!(report
            .primitive_family_coverage_matrix
            .entries
            .iter()
            .any(|entry| &entry.family == family));
        assert!(report
            .primitive_corpus_parity_report
            .entries
            .iter()
            .any(|entry| &entry.family == family));
        assert!(report
            .admitted_range_sweep_report
            .rows
            .iter()
            .any(|row| &row.family == family));
    }
}
