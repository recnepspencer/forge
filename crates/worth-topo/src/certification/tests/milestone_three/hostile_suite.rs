use crate::facade::{
    certify_milestone_three_hostile_suite, WorthMilestoneThreeHostileOutcomeClass,
    WorthMilestoneThreeHostileScenario, WorthReplayParityStatus, WorthTopologyEditNamingOutcome,
    WorthTopologyEditRejectionClass,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn milestone_three_hostile_suite_reports_implemented_coverage_and_missing_named_gap_honestly() {
    let report = certify_milestone_three_hostile_suite(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.hostile_suite",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.scenario_reports.len(), 4);
    assert_eq!(report.coverage_rows.len(), 4);
    assert_eq!(report.implemented_scenario_count, 4);
    assert_eq!(report.required_scenario_count, 5);
    assert_eq!(
        report.missing_required_scenarios,
        vec!["SplitCollapseChurn".to_string()]
    );
    assert!(!report.coverage_complete);
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == WorthMilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.outcome_class == WorthMilestoneThreeHostileOutcomeClass::Rejected
            && row.replay_checked
            && row.replay_parity_status == WorthReplayParityStatus::Match
    }));
    assert!(report.coverage_rows.iter().any(|row| {
        row.scenario == WorthMilestoneThreeHostileScenario::CancellationChainParity
            && row.outcome_class == WorthMilestoneThreeHostileOutcomeClass::Accepted
    }));
}

#[test]
fn milestone_three_hostile_suite_reports_rejection_and_naming_distributions() {
    let report = certify_milestone_three_hostile_suite(
        || build_worth_milestone_one_runtime().expect("worth milestone one runtime builder"),
        "m3.hostile_suite.distribution",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.rejection_distribution_rows.len(), 1);
    assert_eq!(
        report.rejection_distribution_rows[0].rejection_class,
        WorthTopologyEditRejectionClass::InvariantBlocked
    );
    assert_eq!(report.rejection_distribution_rows[0].case_count, 2);
    assert!(report.rejection_distribution_rows[0]
        .scenarios
        .contains(&WorthMilestoneThreeHostileScenario::BowtieAdjacentRewire));
    assert!(report.rejection_distribution_rows[0]
        .scenarios
        .contains(&WorthMilestoneThreeHostileScenario::BrokenRadialLocalization));

    assert_eq!(report.naming_distribution_rows.len(), 2);
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class == WorthTopologyEditNamingOutcome::Ambiguous
            && row.case_count == 3
            && row
                .scenarios
                .contains(&WorthMilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity)
    }));
    assert!(report.naming_distribution_rows.iter().any(|row| {
        row.continuity_outcome_class == WorthTopologyEditNamingOutcome::Rejected
            && row.case_count == 1
            && row
                .scenarios
                .contains(&WorthMilestoneThreeHostileScenario::CancellationChainParity)
    }));
}
