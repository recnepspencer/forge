use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario, ReplayParityStatus,
};

use super::certify_hostile_suite_report;

#[test]
fn hostile_suite_report_gate_shape_is_complete() {
    let report = certify_hostile_suite_report("m3.hostile_suite.gate_shape");

    assert_eq!(report.scenario_reports.len(), 5);
    assert_eq!(report.coverage_rows.len(), 5);
    assert_eq!(report.topology_edit_digest_rows.len(), 5);
    assert_eq!(report.naming_edit_continuity_matrix_rows.len(), 5);
    assert_eq!(report.naming_continuity_breadth_rows.len(), 5);
    assert_eq!(report.edit_replay_parity_rows.len(), 5);
    assert_eq!(report.edit_branch_local_parity_rows.len(), 5);
    assert_eq!(report.replay_branch_breadth_rows.len(), 1);
    assert_eq!(report.edited_query_traversal_rows.len(), 2);
    assert_eq!(report.primitive_family_closure_rows.len(), 5);
    assert_eq!(report.scale_pressure_rows.len(), 6);
    assert_eq!(report.operator_family_closure_rows.len(), 10);
    assert_eq!(report.hostile_certification_category_rows.len(), 9);
    assert_eq!(report.validator_family_coverage_rows.len(), 15);
    assert_eq!(report.validation_breadth_rows.len(), 5);
    assert_eq!(report.rejected_edit_scope_report_rows.len(), 2);
    assert_eq!(report.determinism_rule_rows.len(), 13);
    assert_eq!(report.edit_breadth_counter_rows.len(), 5);
    assert_eq!(report.edit_fallout_breadth_rows.len(), 5);
    assert_eq!(report.failure_locality_rows.len(), 2);
    assert!(!report.changed_scope_coverage_rows.is_empty());
    assert!(!report.derived_region_coverage_rows.is_empty());
    assert_eq!(report.implemented_scenario_count, 5);
    assert_eq!(report.required_scenario_count, 5);
    assert!(report.missing_required_scenarios.is_empty());
    assert!(report.coverage_complete);
    assert!(report.side_quest_gate_ready);
    assert!(report.milestone_three_return_gate_ready);
    assert!(report.milestone_three_return_gate_blocker_rows.is_empty());
    assert!(report.side_quest_closeout_report.phase_three_ready);
    assert_eq!(
        report.side_quest_closeout_report.domain_read_request_count,
        4
    );
    assert_eq!(
        report.side_quest_closeout_report.domain_read_parity_count,
        2
    );
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
            && row.replay_checked
            && row.replay_parity_status == ReplayParityStatus::Match
    }));
}




