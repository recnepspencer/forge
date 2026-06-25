mod aggregate_distributions;
mod ambiguous_local_rewire_continuity;
mod bowtie_adjacent_rewire;
mod branch_local_parity;
mod broken_radial_localization;
mod cancellation_chain_parity;
mod direct_acceptance;
mod hostile_suite;
mod split_collapse_churn;

fn cached_scenario_report(
    scenario: crate::facade::MilestoneThreeHostileScenario,
) -> crate::certification::topology_operator_closeout::MilestoneThreeHostileScenarioReport {
    crate::certification::test_support::cached_milestone_three_closeout_report()
        .scenario_reports
        .into_iter()
        .find(|report| report.scenario == scenario)
        .expect("cached milestone three closeout should include scenario report")
}
