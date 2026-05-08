use crate::facade::{
    certify_milestone_three_hostile_suite, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, ReplayParityStatus, TopologyEditNamingOutcome,
    TopologyEditRejectionClass,
};
use crate::runtime_invariants::build_milestone_one_runtime;

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
    assert_eq!(report.rejected_edit_scope_report_rows.len(), 2);
    assert_eq!(report.edit_breadth_counter_rows.len(), 5);
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
    assert!(report.edit_breadth_counter_rows.iter().all(|row| {
        row.contract_count > 0
            && row.replay_checked
            && row
                .row_digest
                .starts_with(&format!("scenario={};", row.scenario.as_str()))
    }));
    assert!(report
        .changed_scope_coverage_rows
        .iter()
        .any(|row| row.changed_scope == crate::edit::TopologyEditChangedScope::LocalNeighborhood));
    assert!(report
        .derived_region_coverage_rows
        .iter()
        .any(|row| row.derived_region
            == crate::edit::TopologyDerivedRegion::EditLocalNeighborhoodRegion));
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
