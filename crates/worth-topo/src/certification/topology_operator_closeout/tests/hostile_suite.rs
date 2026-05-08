use crate::facade::{
    certify_milestone_three_hostile_suite, MilestoneThreeHostileCertificationStatus,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
    TopologyEditNamingOutcome, TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn milestone_three_hostile_suite_reports_implemented_coverage_and_missing_named_gap_honestly() {
    let report = certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.hostile_suite",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.scenario_reports.len(), 5);
    assert_eq!(report.coverage_rows.len(), 5);
    assert_eq!(report.topology_edit_digest_rows.len(), 5);
    assert_eq!(report.naming_edit_continuity_matrix_rows.len(), 5);
    assert_eq!(report.edit_replay_parity_rows.len(), 5);
    assert_eq!(report.edit_branch_local_parity_rows.len(), 3);
    assert_eq!(report.edited_query_traversal_rows.len(), 2);
    assert_eq!(report.primitive_family_closure_rows.len(), 3);
    assert_eq!(report.hostile_certification_category_rows.len(), 9);
    assert_eq!(report.validator_family_coverage_rows.len(), 15);
    assert_eq!(report.rejected_edit_scope_report_rows.len(), 2);
    assert_eq!(report.determinism_rule_rows.len(), 13);
    assert_eq!(report.edit_breadth_counter_rows.len(), 5);
    assert_eq!(report.edit_fallout_breadth_rows.len(), 5);
    assert_eq!(report.failure_locality_rows.len(), 2);
    assert!(!report.changed_scope_coverage_rows.is_empty());
    assert!(!report.derived_region_coverage_rows.is_empty());
    assert_eq!(report.implemented_scenario_count, 5);
    assert_eq!(report.required_scenario_count, 5);
    assert!(report.side_quest_closeout_report.phase_three_ready);
    assert_eq!(
        report.side_quest_closeout_report.domain_read_request_count,
        4
    );
    assert_eq!(
        report.side_quest_closeout_report.domain_read_parity_count,
        2
    );
    assert!(report.missing_required_scenarios.is_empty());
    assert!(report.side_quest_gate_ready);
    assert!(report.coverage_complete);
    assert!(report.milestone_three_return_gate_ready);
    assert!(report.milestone_three_return_gate_blocker_rows.is_empty());
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.replay_checked
            && row.replay_parity_status == ReplayParityStatus::Match
    }));
    assert!(report.topology_edit_digest_rows.iter().all(|row| {
        row.topology_edit_digest.contract_count > 0
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.naming_edit_continuity_matrix_rows.iter().all(|row| {
        !row.naming_edit_continuity_matrix.rows.is_empty()
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.edit_replay_parity_rows.iter().all(|row| row
        .row_digest
        .starts_with(&format!("scenario={};", row.scenario.as_str()))));
    assert!(report.edit_branch_local_parity_rows.iter().all(|row| {
        row.mutation_origin == "branch_local_application"
            && row.topology_edit_digest.contract_count > 0
            && row
                .row_digest
                .starts_with(&format!("branch={};", row.branch_label))
    }));
    assert!(report.edit_branch_local_parity_rows.iter().any(|row| {
        row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.branch_head_diverged_from_main
            && row.branch_truth_digest.is_some()
    }));
    assert!(report.edited_query_traversal_rows.iter().all(|row| {
        row.scenario == MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
            && row.parity_verified
            && row.left_view_digest == row.replay_view_digest
            && row.request_count > 0
            && row.relationship_proof_admission_count > 0
            && row.traversal_count > 0
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.primitive_family_closure_rows.iter().all(|row| {
        row.replay_verified
            && row.topology_edit_digest.contract_count > 0
            && row.final_materialized_topology_digest
                == row.replay_final_materialized_topology_digest
            && row.derived_validation_row_count > 0
            && row
                .row_digest
                .starts_with(&format!("primitive_family={};", row.primitive_family))
    }));
    assert_eq!(
        report
            .edit_branch_local_parity_rows
            .iter()
            .filter(
                |row| row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                    && row.branch_head_unchanged_after_rejection
                    && row.branch_truth_digest.is_none()
                    && row.rejection_class == Some(TopologyEditRejectionClass::InvariantBlocked)
            )
            .count(),
        2
    );
    assert!(report.edit_breadth_counter_rows.iter().all(|row| {
        row.contract_count > 0
            && row.replay_checked
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.edit_fallout_breadth_rows.iter().all(|row| {
        row.declared_derived_region_count > 0
            && !row.locality_claim_mismatch
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.edit_fallout_breadth_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.fallout_class == crate::facade::MilestoneThreeEditFalloutClass::WholeViewFallback
            && row.derived_validation_row_count > 0
            && row.fallback_count == 1
    }));
    assert!(report.edit_fallout_breadth_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.fallout_class
                == crate::facade::MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork
            && row.derived_validation_row_count == 0
            && row.fallback_count == 0
    }));
    assert!(report.validator_family_coverage_rows.iter().all(|row| {
        !row.validator_names.is_empty()
            && row.edit_family_count > 0
            && row.changed_scope_count > 0
            && row.naming_scope_count > 0
            && row.derived_region_count > 0
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.validator_family_coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.validator_family.as_str() == "derived_validation_inspection"
            && row.derived_validation_row_count > 0
            && row.validator_names.iter().any(|name| name == "ownership")
    }));
    assert!(report.validator_family_coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.validator_family.as_str() == "rejection_locality"
            && row.localized_rejection_boundary
    }));
    assert!(report
        .changed_scope_coverage_rows
        .iter()
        .any(|row| row.changed_scope
            == crate::topology_operators::TopologyEditChangedScope::LocalNeighborhood));
    assert!(report
        .derived_region_coverage_rows
        .iter()
        .any(|row| row.derived_region
            == crate::topology_operators::TopologyDerivedRegion::EditLocalNeighborhoodRegion));
    assert_eq!(
        report
            .hostile_certification_category_rows
            .iter()
            .map(|row| row.category.as_str())
            .collect::<Vec<_>>(),
        vec![
            "mutation_pipeline_integrity",
            "primitive_topology_family_closure",
            "operator_brutality",
            "query_traversal_brutality",
            "non_manifold_radial_brutality",
            "degeneracy_corruption_localization",
            "determinism_order_assault",
            "diagnostics_failure_taxonomy",
            "scale_depth_sustained_pressure",
        ]
    );
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .all(|row| {
            row.evidence_count > 0
                && row.evidence_count == row.evidence_labels.len()
                && row.scenario_count > 0
                && row.replay_verified_count > 0
                && row
                    .row_digest
                    .starts_with(&format!("category={};", row.category.as_str()))
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "primitive_topology_family_closure"
                && row.status == MilestoneThreeHostileCertificationStatus::Certified
                && row.gap_labels.is_empty()
                && row
                    .evidence_labels
                    .iter()
                    .any(|evidence| evidence == "primitive_family_edit_closure=WireClosed(n)")
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "query_traversal_brutality"
                && row.status == MilestoneThreeHostileCertificationStatus::Certified
                && row.gap_labels.is_empty()
                && row.evidence_labels.iter().any(|evidence| {
                    evidence == "edited_topology_query_traversal=post_edit_loop_cycle_view"
                })
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "scale_depth_sustained_pressure"
                && row.status == MilestoneThreeHostileCertificationStatus::Partial
                && row
                    .gap_labels
                    .iter()
                    .any(|gap| gap == "missing_scale_sweep=large_branch_local_histories")
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .all(|row| {
            matches!(row.category.as_str(), "scale_depth_sustained_pressure")
                || row.status == MilestoneThreeHostileCertificationStatus::Certified
        }));
    assert!(report.determinism_rule_rows.iter().all(|row| {
        row.evidence_count > 0
            && row.replay_verified
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert_eq!(
        report
            .determinism_rule_rows
            .iter()
            .filter(|row| row.rule_kind
                == crate::facade::MilestoneThreeDeterminismRuleKind::StableEditOrder
                && row.row_digest.contains("order_policy=sequence_preserving"))
            .count(),
        report.required_scenario_count
    );
    assert!(report.determinism_rule_rows.iter().all(|row| {
        row.rule_kind != crate::facade::MilestoneThreeDeterminismRuleKind::StableEditDigest
            || (!row.diagnostic_classification_stable && !row.tie_break_evidence_stable)
    }));
    assert!(report.determinism_rule_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
            && row.rule_kind
                == crate::facade::MilestoneThreeDeterminismRuleKind::AmbiguousTieBreakEvidence
            && row.diagnostic_classification_stable
            && row.tie_break_evidence_stable
    }));
    assert!(report.determinism_rule_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.rule_kind
                == crate::facade::MilestoneThreeDeterminismRuleKind::StableRejectionClassification
            && row.diagnostic_classification_stable
    }));
    let split_collapse_report = report
        .scenario_reports
        .iter()
        .find(|scenario| scenario.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn)
        .expect("split-collapse churn scenario should be certified");
    let split_collapse_witness = split_collapse_report
        .split_collapse_churn_witness
        .as_ref()
        .expect("split-collapse churn should expose its wire churn witness");
    assert_eq!(split_collapse_witness.split_step_wire_count, 2);
    assert_eq!(split_collapse_witness.final_wire_count, 2);
    assert_eq!(split_collapse_witness.moved_half_edge_identities.len(), 2);
    assert_eq!(
        split_collapse_witness.retained_half_edge_identities.len(),
        2
    );
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
            && row.replay_checked
            && row.replay_parity_status == ReplayParityStatus::Match
    }));
    assert!(report.rejected_edit_scope_report_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && !row.rejected_edit_scope_report.rows.is_empty()
    }));
    assert!(report.rejected_edit_scope_report_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BowtieAdjacentRewire
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && !row.rejected_edit_scope_report.rows.is_empty()
    }));
    assert!(report.failure_locality_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && row.scope_row_count > 0
            && !row.changed_scopes.is_empty()
            && !row.derived_regions.is_empty()
    }));
    assert!(report.failure_locality_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::BowtieAdjacentRewire
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && row.scope_row_count > 0
            && !row.changed_scopes.is_empty()
            && !row.derived_regions.is_empty()
    }));
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::CancellationChainParity
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
    }));
}

#[test]
fn milestone_three_hostile_suite_reports_rejection_and_naming_distributions() {
    let report = certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.hostile_suite.distribution",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.rejection_distribution_rows.len(), 1);
    assert_eq!(
        report.rejection_distribution_rows[0].rejection_class,
        TopologyEditRejectionClass::InvariantBlocked
    );
    assert_eq!(report.rejection_distribution_rows[0].case_count, 2);
    assert!(report.rejection_distribution_rows[0]
        .scenarios
        .contains(&MilestoneThreeHostileScenario::BowtieAdjacentRewire));
    assert!(report.rejection_distribution_rows[0]
        .scenarios
        .contains(&MilestoneThreeHostileScenario::BrokenRadialLocalization));

    assert_eq!(report.naming_distribution_rows.len(), 2);
    assert_eq!(report.side_quest_closeout_report.contract_rows.len(), 4);
    assert_eq!(
        report
            .side_quest_closeout_report
            .contract_rows
            .iter()
            .map(|row| row.contract_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "topology_read_lowering_breadth",
            "topology_read_fallback_posture",
            "topology_read_view_parity",
            "topology_read_relationship_proof_posture",
        ]
    );
    assert!(report
        .side_quest_closeout_report
        .contract_rows
        .iter()
        .all(|row| row.status == "satisfied"
            && row
                .row_digest
                .starts_with(&format!("contract={};", row.contract_name))));
    assert!(report
        .side_quest_closeout_report
        .blocker_rows
        .iter()
        .all(|row| row.status == "clear"));
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class == TopologyEditNamingOutcome::Ambiguous
            && row.case_count == 3
            && row
                .scenarios
                .contains(&MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity)
    }));
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class == TopologyEditNamingOutcome::Rejected
            && row.case_count == 2
            && row
                .scenarios
                .contains(&MilestoneThreeHostileScenario::CancellationChainParity)
            && row
                .scenarios
                .contains(&MilestoneThreeHostileScenario::SplitCollapseChurn)
    }));
}
