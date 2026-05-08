use super::support::{
    assert_milestone_one_closeout_bridge_and_corpus,
    assert_milestone_one_closeout_surface_integrity,
};
use super::*;

#[test]
fn milestone_one_closeout_emits_bootstrap_and_corpus_proof_surfaces() {
    let report = certify_milestone_one_closeout(
        || {
            crate::facade::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-one-closeout",
    )
    .expect("milestone one closeout should succeed");

    assert_milestone_one_closeout_surface_integrity(&report);
    assert_milestone_one_closeout_bridge_and_corpus(&report);
}

#[test]
fn milestone_two_closeout_emits_direct_derived_proof_surfaces() {
    let report = certify_milestone_two_closeout(
        || {
            crate::facade::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-two-closeout",
    )
    .expect("milestone two closeout should succeed");

    assert!(report.materialized_topology_digest.row_count > 0);
    assert!(report.interpreted_topology_digest.row_count > 0);
    assert!(report.derived_validation_digest.row_count > 0);
    assert!(report.derived_truth_basis_digest.row_count > 0);
    assert!(report.bridge_routing_digest.row_count > 0);
    assert!(report.bridge_historical_evaluation_digest.row_count > 0);
    assert!(!report.derived_family_coverage_matrix.rows.is_empty());
    assert!(!report.derived_family_parity_matrix.rows.is_empty());
    assert!(!report.derived_validator_coverage_report.rows.is_empty());
    assert!(report
        .derived_validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "WireBranch(k)" && row.validator == "vertex_branching"));
    assert!(report
        .derived_validator_coverage_report
        .rows
        .iter()
        .any(|row| row.family == "SolidShell(f)" && row.validator == "shell_closure"));
    assert!(!report.derived_invalidation_report.rows.is_empty());
    assert!(!report.derived_rebuild_report.rows.is_empty());
    assert!(!report.derived_equivalence_contract_report.rows.is_empty());
    assert!(!report.derived_fallback_report.rows.is_empty());
    assert!(!report.derived_failure_locality_report.rows.is_empty());
    assert!(!report
        .derived_branch_local_parity_report
        .branch_ids
        .is_empty());
    assert!(
        report
            .derived_replay_parity_report
            .replay_checked_case_count
            > 0
    );
    assert!(!report.derived_bridge_family_coverage_report.rows.is_empty());
    assert!(report.milestone_2_counter_report.derived_read_count > 0);
}

#[test]
fn milestone_one_closeout_requirements_registry_matches_canonical_closeout_shape() {
    let requirements = milestone_one_closeout_requirements();
    let suite = milestone_one_closeout_suite_definition();

    assert_eq!(requirements.suite_name, ".milestone_1.closeout");
    assert_eq!(requirements.required_family_rows.len(), 7);
    assert_eq!(requirements.required_rejection_rows.len(), 7);
    assert_eq!(requirements.required_parity_rows.len(), 7);
    assert_eq!(requirements.required_bridge_rows.len(), 7);
    assert_eq!(suite.suite_name, requirements.suite_name);
    assert_eq!(suite.canonical_rows.len(), 21);
    assert_eq!(suite.rejection_rows.len(), 7);
    assert_eq!(suite.parity_rows.len(), 14);
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::BridgeFamilyCoverageReport));
    assert!(requirements
        .validator_expectations
        .iter()
        .any(|expectation| expectation.family == "WireBranch(k)"
            && expectation
                .validators
                .iter()
                .any(|validator| validator == "vertex_branching")));
    assert!(requirements
        .validator_expectations
        .iter()
        .any(|expectation| expectation.family == "SolidShell(f)"
            && expectation
                .validators
                .iter()
                .any(|validator| validator == "shell_closure")));
}

#[test]
fn milestone_two_closeout_requirements_registry_matches_direct_derived_outputs() {
    let requirements = milestone_two_closeout_requirements();
    let suite = milestone_two_closeout_suite_definition();

    assert_eq!(requirements.suite_name, ".milestone_2.closeout");
    assert_eq!(requirements.required_family_rows.len(), 7);
    assert_eq!(requirements.required_rejection_rows.len(), 7);
    assert_eq!(requirements.required_parity_rows.len(), 7);
    assert_eq!(requirements.required_bridge_rows.len(), 7);
    assert_eq!(suite.suite_name, requirements.suite_name);
    assert_eq!(suite.canonical_rows.len(), 21);
    assert_eq!(suite.rejection_rows.len(), 7);
    assert_eq!(suite.parity_rows.len(), 14);
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MaterializedTopologyDigest));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::DerivedEquivalenceContractReport));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::DerivedTruthBasisDigest));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::DerivedValidatorCoverageReport));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::DerivedBridgeFamilyCoverageReport));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneTwoCounterReport));
    assert!(requirements
        .validator_expectations
        .iter()
        .any(|expectation| expectation.family == "WireBranch(k)"
            && expectation
                .validators
                .iter()
                .any(|validator| validator == "vertex_branching")));
    assert!(requirements
        .validator_expectations
        .iter()
        .any(|expectation| expectation.family == "SolidShell(f)"
            && expectation
                .validators
                .iter()
                .any(|validator| validator == "shell_closure")));
}

#[test]
fn milestone_three_closeout_requirements_registry_matches_hostile_return_gate_shape() {
    let requirements = milestone_three_closeout_requirements();
    let suite = milestone_three_closeout_suite_definition();

    assert_eq!(requirements.suite_name, ".milestone_3.closeout");
    assert_eq!(requirements.required_family_rows.len(), 5);
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
        vec![
            "CancellationChainParity".to_string(),
            "SplitCollapseChurn".to_string(),
            "AmbiguousLocalRewireContinuity".to_string(),
            "BrokenRadialLocalization".to_string(),
        ]
    );
    assert!(requirements.required_bridge_rows.is_empty());
    assert!(requirements.validator_expectations.is_empty());
    assert_eq!(suite.suite_name, requirements.suite_name);
    assert_eq!(suite.canonical_rows.len(), 5);
    assert_eq!(suite.rejection_rows.len(), 2);
    assert_eq!(suite.parity_rows.len(), 4);
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneThreeHostileSuiteReport));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneThreeSideQuestCloseoutReport));
    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneThreeReturnGateReport));
}

#[test]
fn milestone_three_closeout_enforces_declared_closeout_requirements() {
    let requirements = milestone_three_closeout_requirements();
    let report = certify_milestone_three_closeout(
        || {
            crate::facade::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-closeout-requirements",
    )
    .expect("milestone three closeout should certify");
    let coverage_scenarios = report
        .coverage_rows
        .iter()
        .map(|row| row.scenario.as_str().to_string())
        .collect::<Vec<_>>();
    let rejected_scenarios = report
        .coverage_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.rejection_class.is_some()
        })
        .map(|row| row.scenario.as_str().to_string())
        .collect::<Vec<_>>();
    let replay_scenarios = report
        .coverage_rows
        .iter()
        .filter(|row| row.replay_checked && row.replay_parity_status == ReplayParityStatus::Match)
        .map(|row| row.scenario.as_str().to_string())
        .collect::<Vec<_>>();

    assert_eq!(coverage_scenarios, requirements.required_family_rows);
    assert_eq!(rejected_scenarios, requirements.required_rejection_rows);
    assert_eq!(replay_scenarios, requirements.required_parity_rows);
    assert!(!report.coverage_rows.is_empty());
    assert!(!report.family_coverage_rows.is_empty());
    assert!(!report.rejection_distribution_rows.is_empty());
    assert!(!report.naming_distribution_rows.is_empty());
    assert!(report.side_quest_closeout_report.phase_three_ready);
    assert!(report.side_quest_closeout_report.domain_read_request_count > 0);
    assert!(report.side_quest_closeout_report.domain_read_parity_count > 0);
    assert!(report.milestone_three_return_gate_ready);
    assert!(report.milestone_three_return_gate_blocker_rows.is_empty());
}

#[test]
fn local_certification_core_expresses_suite_rows_without_specific_branching() {
    let suite = CertificationSuiteDefinition {
        suite_name: ".test.shape".to_string(),
        canonical_rows: vec![CertificationCanonicalRow {
            family: "WireOpen(n)".to_string(),
            role: "Generic".to_string(),
        }],
        rejection_rows: vec![CertificationRejectionRow {
            family: "WireClosed(n)".to_string(),
            role: "OutOfClass".to_string(),
            rejection_class: "OutOfClass".to_string(),
        }],
        parity_rows: vec![CertificationParityRow {
            family: "WireBranch(k)".to_string(),
            parity_kind: "branch".to_string(),
        }],
        required_outputs: vec![
            CertificationRequiredOutput::TopologyTruthDigest,
            CertificationRequiredOutput::FailureLocalityReport,
        ],
    };

    assert_eq!(suite.canonical_rows.len(), 1);
    assert_eq!(suite.rejection_rows.len(), 1);
    assert_eq!(suite.parity_rows.len(), 1);
    assert_eq!(suite.required_outputs.len(), 2);
}

#[test]
fn fixtures_provide_named_phase_inputs_for_milestone_one_closeout() {
    let authored = milestone_one_default_corpus_scenarios();
    let branch_local = milestone_one_default_branch_local_admitted_scenarios();

    assert!(!authored.is_empty());
    assert!(authored.iter().any(|scenario| {
        scenario.expected_outcome == MilestoneOnePrimitiveExpectedOutcome::Reject
    }));
    assert!(!branch_local.is_empty());
    assert!(branch_local.iter().all(|scenario| {
        scenario.expected_outcome == MilestoneOnePrimitiveExpectedOutcome::Admit
    }));
}
