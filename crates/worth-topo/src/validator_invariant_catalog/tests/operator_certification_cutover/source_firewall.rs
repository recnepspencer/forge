use super::super::relational_invariant_catalog::execution_inputs::relational_invariant_query_execution_input;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyOperatorCertificationCutoverDenialKind,
    WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
};

#[test]
fn source_firewall_flags_old_expectation_array_authority() {
    let report =
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::from_source_pairs([
            (
                "dirty.rs",
                "fn old() { let _ = milestone_three_validator_expectations(); }",
            ),
            (
                "also_dirty.rs",
                "let rows = CertificationValidatorExpectation::new();",
            ),
        ]);

    assert_eq!(report.scanned_file_count(), 2);
    assert!(!report.is_clean());
    assert_eq!(report.violations().len(), 2);
    assert!(report
        .violations()
        .iter()
        .any(|violation| violation == "dirty.rs::milestone_three_validator_expectations"));
    assert!(report
        .violations()
        .iter()
        .any(|violation| violation == "also_dirty.rs::CertificationValidatorExpectation"));
}

#[test]
fn current_source_scan_allows_only_declared_capped_old_expectation_residue() {
    let residue =
        WorthTopologyOperatorCertificationOldExpectationResidueReport::current_capped_migration_residue();
    let report =
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::current_with_capped_residue(
            &residue,
        );
    let expected_paths =
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::CURRENT_SCAN_SOURCE_PATHS
            .iter()
            .map(|path| (*path).to_string())
            .collect::<Vec<_>>();

    assert_eq!(report.scanned_source_paths(), expected_paths);
    assert_eq!(
        report.scanned_file_count(),
        report.scanned_source_paths().len()
    );
    let capped_paths = residue
        .rows()
        .iter()
        .map(|row| row.source_path().to_string())
        .collect::<Vec<_>>();
    assert_eq!(report.allowed_capped_residue_paths(), capped_paths);
    assert!(
        report.is_clean(),
        "known old closeout authority may survive only through capped residue rows"
    );
}

#[test]
fn every_forbidden_authority_pattern_is_hostile_in_covered_source_regions() {
    let covered_path =
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::CURRENT_SCAN_SOURCE_PATHS[0];

    for pattern in
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::forbidden_authority_patterns(
        )
    {
        let report =
            WorthTopologyOperatorCertificationCutoverSourceFirewallReport::from_source_pairs([(
                covered_path,
                format!("fn resurrect_old_operator_authority() {{ let _ = {pattern}; }}"),
            )]);
        assert_eq!(report.scanned_source_paths(), &[covered_path.to_string()]);
        assert_eq!(
            report.violations(),
            &[format!("{covered_path}::{pattern}")],
            "covered source regions must reject `{pattern}` as old operator authority"
        );
    }
}

#[test]
fn uncapped_current_source_old_authority_is_rejected_by_the_real_scan_path() {
    let report =
        WorthTopologyOperatorCertificationCutoverSourceFirewallReport::current_with_capped_residue(
            &WorthTopologyOperatorCertificationOldExpectationResidueReport::empty(),
        );

    assert!(!report.is_clean());
    assert!(report.violations().iter().any(|violation| {
        violation
            == "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs::CertificationValidatorExpectation"
    }));
}

#[test]
fn closeout_rejects_source_firewall_violations_before_projection() {
    let (relational_closeout, execution_input) = relational_invariant_query_execution_input();
    let enforcement =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )
        .expect("Phase 6 enforcement should close");
    let firewall = WorthTopologyOperatorCertificationCutoverSourceFirewallReport::from_source_pairs(
        [("dirty.rs", "let _ = validator_family_count;")],
    );

    let error =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement_with_reports(
            &enforcement,
            firewall,
            WorthTopologyOperatorCertificationOldExpectationResidueReport::empty(),
        )
        .unwrap_err();

    let WorthTopologyLegalityCatalogError::OperatorCertificationCutover(denial) = error else {
        panic!("expected operator certification cutover denial");
    };
    assert_eq!(
        denial.kind(),
        WorthTopologyOperatorCertificationCutoverDenialKind::SourceFirewallViolation
    );
}
