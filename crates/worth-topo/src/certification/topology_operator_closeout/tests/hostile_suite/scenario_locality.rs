use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
    TopologyEditRejectionClass,
};

use super::certify_hostile_suite_report;

#[test]
fn hostile_suite_validator_and_traversal_rows_localize_scenarios() {
    let report = certify_hostile_suite_report("m3.hostile_suite.locality");

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
}

#[test]
fn hostile_suite_scenario_witnesses_and_rejections_are_precise() {
    let report = certify_hostile_suite_report("m3.hostile_suite.scenarios");

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
    assert_rejected_scope_row_exists(
        &report,
        MilestoneThreeHostileScenario::BrokenRadialLocalization,
    );
    assert_rejected_scope_row_exists(&report, MilestoneThreeHostileScenario::BowtieAdjacentRewire);
    assert_failure_locality_row_exists(
        &report,
        MilestoneThreeHostileScenario::BrokenRadialLocalization,
    );
    assert_failure_locality_row_exists(
        &report,
        MilestoneThreeHostileScenario::BowtieAdjacentRewire,
    );
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::CancellationChainParity
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
    }));
}

#[test]
fn hostile_suite_determinism_rows_cover_order_digest_and_tie_breaks() {
    let report = certify_hostile_suite_report("m3.hostile_suite.determinism");

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
}

fn assert_rejected_scope_row_exists(
    report: &crate::facade::MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) {
    assert!(report.rejected_edit_scope_report_rows.iter().any(|row| {
        row.scenario == scenario
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && !row.rejected_edit_scope_report.rows.is_empty()
    }));
}

fn assert_failure_locality_row_exists(
    report: &crate::facade::MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) {
    assert!(report.failure_locality_rows.iter().any(|row| {
        row.scenario == scenario
            && row.rejection_class == TopologyEditRejectionClass::InvariantBlocked
            && row.scope_row_count > 0
            && !row.changed_scopes.is_empty()
            && !row.derived_regions.is_empty()
    }));
}




