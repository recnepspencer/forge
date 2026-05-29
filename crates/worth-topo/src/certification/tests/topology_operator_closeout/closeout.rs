use super::super::*;
use super::expectations::*;
use crate::facade::{
    CertificationSuiteRequirements, MilestoneThreeHostileScenario, TopologyEditNamingOutcome,
    TopologyEditRejectionClass,
};

#[test]
fn milestone_three_closeout_requirements_registry_matches_hostile_return_gate_shape() {
    let requirements = milestone_three_closeout_requirements();
    let suite = milestone_three_closeout_suite_definition();

    assert_eq!(requirements.suite_name, ".milestone_3.closeout");
    assert_eq!(
        requirements.required_family_rows,
        vec![
            "BowtieAdjacentRewire".to_string(),
            "CancellationChainParity".to_string(),
            "SplitCollapseChurn".to_string(),
            "AmbiguousLocalRewireContinuity".to_string(),
            "BrokenRadialLocalization".to_string(),
        ]
    );
    assert_eq!(
        requirements.required_rejection_rows,
        vec![
            "BowtieAdjacentRewire".to_string(),
            "BrokenRadialLocalization".to_string(),
        ]
    );
    assert_eq!(
        requirements.required_parity_rows,
        requirements.required_family_rows
    );
    assert!(requirements.required_bridge_rows.is_empty());
    assert_eq!(
        validator_expectation_pairs(&requirements.validator_expectations),
        expected_milestone_three_validator_expectations()
    );
    assert_eq!(suite.suite_name, requirements.suite_name);
    assert_eq!(suite.canonical_rows.len(), 5);
    assert_eq!(suite.rejection_rows.len(), 2);
    assert_eq!(suite.parity_rows.len(), 5);
    assert_milestone_three_required_outputs(&requirements.required_outputs);
}

#[test]
fn milestone_three_closeout_enforces_declared_closeout_requirements() {
    let requirements = milestone_three_closeout_requirements();
    let report = certify_milestone_three_closeout(
        || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-closeout-requirements",
    )
    .expect("milestone three closeout should certify");

    assert_eq!(
        report
            .coverage_rows
            .iter()
            .map(|row| row.scenario.as_str().to_string())
            .collect::<Vec<_>>(),
        requirements.required_family_rows
    );
    assert_eq!(
        rejected_scenarios_from_report(&report),
        requirements.required_rejection_rows
    );
    assert_eq!(
        replay_scenarios_from_report(&report),
        requirements.required_parity_rows
    );
    assert_eq!(
        report.topology_edit_digest_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        report.naming_edit_continuity_matrix_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        report.naming_continuity_breadth_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        report.edit_replay_parity_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        accepted_branch_local_row_count(&report),
        requirements.required_family_rows.len() - requirements.required_rejection_rows.len(),
        "closeout should prove accepted branch-local edit parity for each accepted scenario"
    );
    assert_eq!(
        accepted_branch_local_scenarios_from_report(&report),
        vec![
            "CancellationChainParity".to_string(),
            "SplitCollapseChurn".to_string(),
            "AmbiguousLocalRewireContinuity".to_string(),
        ],
        "accepted branch-local evidence must be scenario-specific"
    );
    assert_eq!(
        rejected_branch_local_scenarios_from_report(&report),
        requirements.required_rejection_rows,
        "closeout should prove rejected branch-local diagnostic parity"
    );
    assert_eq!(
        stable_edit_digest_scenarios_from_report(&report),
        requirements.required_family_rows
    );
    assert_eq!(
        stable_edit_order_scenarios_from_report(&report),
        requirements.required_family_rows
    );
    assert_eq!(
        report.rejected_edit_scope_report_rows.len(),
        requirements.required_rejection_rows.len()
    );
    assert_eq!(
        report.edit_breadth_counter_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        report.edit_fallout_breadth_rows.len(),
        requirements.required_family_rows.len()
    );
    assert_eq!(
        report.failure_locality_rows.len(),
        requirements.required_rejection_rows.len()
    );
    assert_declared_validator_expectations_have_rows(&report, &requirements);
    assert_milestone_three_direct_rows_are_nonempty(&report);
}

fn assert_declared_validator_expectations_have_rows(
    report: &crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport,
    requirements: &CertificationSuiteRequirements,
) {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let row = report
                .validator_family_coverage_rows
                .iter()
                .find(|row| {
                    row.scenario.as_str() == expectation.family
                        && row.validator_family.as_str() == validator
                })
                .expect("milestone three validator expectation should have a proof row");
            assert!(!row.validator_names.is_empty());
            if validator == "derived_validation_inspection" {
                assert!(row.derived_validation_row_count > 0);
                assert!(row.validator_names.iter().any(|name| name == "ownership"));
            }
            if validator == "rejection_locality" {
                assert!(row.localized_rejection_boundary);
            }
        }
    }
}

fn assert_milestone_three_direct_rows_are_nonempty(
    report: &crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport,
) {
    assert!(!report.coverage_rows.is_empty());
    assert!(!report.family_coverage_rows.is_empty());
    assert!(!report.rejection_distribution_rows.is_empty());
    assert!(!report.naming_distribution_rows.is_empty());
    assert!(!report.changed_scope_coverage_rows.is_empty());
    assert!(!report.derived_region_coverage_rows.is_empty());
    assert!(report
        .topology_edit_digest_rows
        .iter()
        .all(|row| row.topology_edit_digest.contract_count > 0));
    assert!(report
        .naming_edit_continuity_matrix_rows
        .iter()
        .all(|row| !row.naming_edit_continuity_matrix.rows.is_empty()));
    assert!(report
        .naming_continuity_breadth_rows
        .iter()
        .all(|row| row.continuity_row_count() > 0
            && row.naming_scope_count() > 0
            && row.replay_checked()));
    assert!(report
        .edit_breadth_counter_rows
        .iter()
        .all(|row| row.contract_count > 0 && row.replay_checked));
    assert!(report
        .edit_fallout_breadth_rows
        .iter()
        .all(|row| row.declared_derived_region_count > 0
            && !row.locality_claim_mismatch
            && !row.fallback_policy_exceeded
            && row.fallback_rejection_class.is_none()));
    assert_eq!(report.edited_query_traversal_rows.len(), 2);
    assert!(report.edited_query_traversal_rows.iter().all(|row| {
        row.parity_verified
            && row.request_count > 0
            && row.relationship_proof_admission_count > 0
            && row.traversal_count > 0
            && row.left_view_digest == row.replay_view_digest
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert_eq!(report.primitive_family_closure_rows.len(), 5);
    assert_eq!(
        primitive_closure_families_from_report(report),
        vec![
            "SheetDisk(n)".to_string(),
            "SheetPatch(f)".to_string(),
            "SolidShell(f)".to_string(),
            "WireClosed(n)".to_string(),
            "WireOpen(n)".to_string(),
        ]
    );
    assert!(report.primitive_family_closure_rows.iter().all(|row| {
        row.replay_verified
            && row.topology_edit_digest.contract_count > 0
            && row.final_materialized_topology_digest
                == row.replay_final_materialized_topology_digest
            && row.derived_validation_row_count > 0
    }));
    assert_eq!(report.scale_pressure_rows.len(), 6);
    assert!(report.scale_pressure_rows.iter().all(|row| {
        row.replay_verified()
            && row.topology_edit_digest().contract_count > 0
            && row.workload_size() > 0
            && row.edit_step_count() > 0
            && row.final_state_digest() == row.replay_final_state_digest()
    }));
    assert!(report
        .scale_pressure_rows
        .iter()
        .any(|row| row.sweep_label() == "large_branch_local_histories" && row.branch_local()));
    assert!(report.determinism_rule_rows.iter().all(|row| {
        row.evidence_count > 0
            && row.replay_verified
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report.determinism_rule_rows.iter().any(|row| {
        row.rule_kind == MilestoneThreeDeterminismRuleKind::AmbiguousTieBreakEvidence
            && row.diagnostic_classification_stable
            && row.tie_break_evidence_stable
    }));
    assert!(report.edit_fallout_breadth_rows.iter().any(|row| {
        row.fallout_class == MilestoneThreeEditFalloutClass::WholeViewFallback
            && row.fallback_count > 0
            && row.derived_validation_row_count > 0
    }));
    assert!(report
        .failure_locality_rows
        .iter()
        .all(|row| row.scope_row_count > 0 && !row.changed_scopes.is_empty()));
    assert!(report.family_coverage_rows.iter().all(|row| {
        row.scenario_count() == row.scenarios().len()
            && row.scenario_count() > 0
            && row
                .row_digest()
                .starts_with(&format!("family={:?};", row.family()))
            && row.row_digest().contains("scenarios=")
    }));
    assert!(report.rejection_distribution_rows.iter().any(|row| {
        row.rejection_class() == TopologyEditRejectionClass::InvariantBlocked
            && row.case_count() == 2
            && row.scenarios().len() == 2
            && row
                .row_digest()
                .starts_with("rejection_class=InvariantBlocked;")
            && row.row_digest().contains("BowtieAdjacentRewire")
            && row.row_digest().contains("BrokenRadialLocalization")
    }));
    assert_eq!(
        report.rejection_distribution_rows.len(),
        TopologyEditRejectionClass::ALL.len()
    );
    for rejection_class in TopologyEditRejectionClass::ALL {
        assert!(
            report
                .rejection_distribution_rows
                .iter()
                .any(|row| row.rejection_class() == rejection_class
                    && row.case_count() == row.scenarios().len()),
            "milestone three rejection distribution must retain closed taxonomy row for {rejection_class:?}"
        );
    }
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class() == TopologyEditNamingOutcome::Ambiguous
            && row.case_count() > 0
            && row
                .scenarios()
                .contains(&MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity)
            && row.row_digest().starts_with("naming_outcome=Ambiguous;")
            && row.row_digest().contains("AmbiguousLocalRewireContinuity")
    }));
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class() == TopologyEditNamingOutcome::Rejected
            && row.case_count() > 0
            && row.row_digest().starts_with("naming_outcome=Rejected;")
            && row.row_digest().contains("scenarios=")
    }));
    assert_eq!(report.hostile_certification_category_rows.len(), 9);
    assert_eq!(report.operator_family_closure_rows.len(), 10);
    assert!(report.operator_family_closure_rows.iter().all(|row| {
        !row.admitted_lane_labels().is_empty()
            && !row.legal_evidence_labels().is_empty()
            && !row.hostile_evidence_labels().is_empty()
            && !row.replay_evidence_labels().is_empty()
            && !row.rejection_evidence_labels().is_empty()
            && row.legal_execution_count() > 0
            && row.hostile_workload_count() > 0
            && row.replay_evidence_count() > 0
            && row.rejection_evidence_count() > 0
            && row.derived_breadth_evidence_count() > 0
            && row.row_digest().contains("hostile_workloads=")
    }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .all(|row| row.evidence_count > 0
            && row.evidence_count == row.evidence_labels.len()
            && row.scenario_count > 0
            && row.replay_verified_count > 0
            && row
                .row_digest
                .starts_with(&format!("category={};", row.category.as_str()))));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "primitive_topology_family_closure"
                && row.status.as_str() == "certified"
                && row.gap_labels.is_empty()
                && row
                    .evidence_labels
                    .iter()
                    .any(|evidence| evidence == "primitive_family_edit_closure=SolidShell(f)")
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "query_traversal_brutality"
                && row.status.as_str() == "certified"
                && row.gap_labels.is_empty()
                && row.evidence_labels.iter().any(|evidence| {
                    evidence == "edited_topology_query_traversal=post_edit_local_rewire_view"
                })
        }));
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| {
            row.category.as_str() == "scale_depth_sustained_pressure"
                && row.status.as_str() == "certified"
                && row.gap_labels.is_empty()
                && row
                    .evidence_labels
                    .iter()
                    .any(|evidence| evidence == "scale_pressure=high_cardinality_loops")
        }));
    assert!(report.side_quest_closeout_report.phase_three_ready);
    assert!(report.side_quest_closeout_report.domain_read_request_count > 0);
    assert!(report.side_quest_closeout_report.domain_read_parity_count > 0);
    assert!(report.milestone_three_return_gate_ready);
    assert!(report.milestone_three_return_gate_blocker_rows.is_empty());
}

fn primitive_closure_families_from_report(
    report: &crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    let mut families = report
        .primitive_family_closure_rows
        .iter()
        .map(|row| row.primitive_family.clone())
        .collect::<Vec<_>>();
    families.sort();
    families
}




