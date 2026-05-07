use std::collections::BTreeMap;

use forge_relational::facade::runtime::RelationalRuntime;

use super::ambiguous_local_rewire::certify_milestone_three_ambiguous_local_rewire_continuity_impl;
use super::bowtie_adjacent::certify_milestone_three_bowtie_adjacent_rewire_impl;
use super::broken_radial_localization::certify_milestone_three_broken_radial_localization_impl;
use super::cancellation_chain::certify_milestone_three_cancellation_chain_parity_impl;
use super::report::{
    WorthMilestoneThreeHostileCoverageRow, WorthMilestoneThreeHostileFamilyCoverageRow,
    WorthMilestoneThreeHostileNamingDistributionRow,
    WorthMilestoneThreeHostileRejectionDistributionRow, WorthMilestoneThreeHostileScenario,
    WorthMilestoneThreeHostileScenarioReport, WorthMilestoneThreeHostileSuiteReport,
};
use crate::certification::error::WorthTopologyCertificationError;
use crate::edit::{
    WorthTopologyEditFamily, WorthTopologyEditNamingOutcome, WorthTopologyEditRejectionClass,
};

const REQUIRED_SCENARIOS: &[&str] = &[
    "BowtieAdjacentRewire",
    "CancellationChainParity",
    "SplitCollapseChurn",
    "AmbiguousLocalRewireContinuity",
    "BrokenRadialLocalization",
];

pub(crate) fn certify_milestone_three_hostile_suite_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneThreeHostileSuiteReport, WorthTopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let scenario_reports = vec![
        certify_milestone_three_bowtie_adjacent_rewire_impl(
            &mut runtime_factory,
            &format!("{stem}.bowtie"),
        )?,
        certify_milestone_three_cancellation_chain_parity_impl(
            &mut runtime_factory,
            &format!("{stem}.cancellation"),
        )?,
        certify_milestone_three_ambiguous_local_rewire_continuity_impl(
            &mut runtime_factory,
            &format!("{stem}.ambiguous"),
        )?,
        certify_milestone_three_broken_radial_localization_impl(
            &mut runtime_factory,
            &format!("{stem}.broken_radial"),
        )?,
    ];
    let coverage_rows = build_coverage_rows(&scenario_reports);
    let family_coverage_rows = build_family_coverage_rows(&scenario_reports);
    let rejection_distribution_rows = build_rejection_distribution_rows(&scenario_reports);
    let naming_distribution_rows = build_naming_distribution_rows(&scenario_reports);
    let implemented_names = scenario_reports
        .iter()
        .map(|report| format!("{:?}", report.scenario))
        .collect::<Vec<_>>();
    let missing_required_scenarios = REQUIRED_SCENARIOS
        .iter()
        .filter(|name| {
            !implemented_names
                .iter()
                .any(|implemented| implemented == *name)
        })
        .map(|name| (*name).to_string())
        .collect::<Vec<_>>();

    Ok(WorthMilestoneThreeHostileSuiteReport {
        scenario_reports,
        coverage_rows,
        family_coverage_rows,
        rejection_distribution_rows,
        naming_distribution_rows,
        missing_required_scenarios: missing_required_scenarios.clone(),
        implemented_scenario_count: implemented_names.len(),
        required_scenario_count: REQUIRED_SCENARIOS.len(),
        coverage_complete: missing_required_scenarios.is_empty(),
    })
}

fn build_coverage_rows(
    reports: &[WorthMilestoneThreeHostileScenarioReport],
) -> Vec<WorthMilestoneThreeHostileCoverageRow> {
    reports
        .iter()
        .map(|report| WorthMilestoneThreeHostileCoverageRow {
            scenario: report.scenario,
            outcome_class: report.outcome_class,
            rejection_class: report.rejection_class,
            continuity_outcome_class: report.continuity_outcome_class,
            continuity_rejection_class: report.continuity_rejection_class,
            replay_checked: report.edit_replay_parity_report.replay_checked,
            replay_parity_status: report.edit_replay_parity_report.parity_status,
        })
        .collect()
}

fn build_family_coverage_rows(
    reports: &[WorthMilestoneThreeHostileScenarioReport],
) -> Vec<WorthMilestoneThreeHostileFamilyCoverageRow> {
    let mut rows =
        BTreeMap::<WorthTopologyEditFamily, Vec<WorthMilestoneThreeHostileScenario>>::new();
    for report in reports {
        for family in &report.edit_families {
            rows.entry(*family).or_default().push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(family, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            WorthMilestoneThreeHostileFamilyCoverageRow {
                family,
                scenario_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}

fn build_rejection_distribution_rows(
    reports: &[WorthMilestoneThreeHostileScenarioReport],
) -> Vec<WorthMilestoneThreeHostileRejectionDistributionRow> {
    let mut rows =
        BTreeMap::<WorthTopologyEditRejectionClass, Vec<WorthMilestoneThreeHostileScenario>>::new();
    for report in reports {
        if let Some(rejection_class) = report.rejection_class {
            rows.entry(rejection_class)
                .or_default()
                .push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(rejection_class, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            WorthMilestoneThreeHostileRejectionDistributionRow {
                rejection_class,
                case_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}

fn build_naming_distribution_rows(
    reports: &[WorthMilestoneThreeHostileScenarioReport],
) -> Vec<WorthMilestoneThreeHostileNamingDistributionRow> {
    let mut rows =
        BTreeMap::<WorthTopologyEditNamingOutcome, Vec<WorthMilestoneThreeHostileScenario>>::new();
    for report in reports {
        rows.entry(report.continuity_outcome_class)
            .or_default()
            .push(report.scenario);
    }
    rows.into_iter()
        .map(|(continuity_outcome_class, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            WorthMilestoneThreeHostileNamingDistributionRow {
                continuity_outcome_class,
                case_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}
