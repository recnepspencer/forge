use std::collections::BTreeMap;

use forge_relational::facade::runtime::RelationalRuntime;

use super::ambiguous_local_rewire::certify_milestone_three_ambiguous_local_rewire_continuity_impl;
use super::bowtie_adjacent::certify_milestone_three_bowtie_adjacent_rewire_impl;
use super::branch_local_acceptance::ensure_branch_local_edit_parity_rows;
use super::branch_local_parity::certify_milestone_three_branch_local_edit_parity_impl;
use super::broken_radial_localization::certify_milestone_three_broken_radial_localization_impl;
use super::cancellation_chain::certify_milestone_three_cancellation_chain_parity_impl;
use super::direct_acceptance::{build_direct_acceptance_rows, ensure_direct_acceptance_proof_rows};
use super::edited_query_traversal::{
    certify_milestone_three_edited_query_traversal_impl, ensure_edited_query_traversal_rows,
};
use super::hostile_category_posture::{
    build_hostile_certification_category_rows, ensure_hostile_certification_category_rows,
};
use super::primitive_family_closure::{
    certify_milestone_three_primitive_family_closure_impl, ensure_primitive_family_closure_rows,
};
use super::report::{
    MilestoneThreeHostileCoverageRow, MilestoneThreeHostileFamilyCoverageRow,
    MilestoneThreeHostileNamingDistributionRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileRejectionDistributionRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
};
use super::side_quest_closeout::certify_milestone_three_side_quest_closeout_impl;
use super::side_quest_types::MilestoneThreeReturnGateBlockerRow;
use super::split_collapse_churn::certify_milestone_three_split_collapse_churn_impl;
use super::validator_family_coverage::{
    build_validator_family_coverage_rows, ensure_validator_family_coverage_rows,
};
use super::{
    milestone_three_rejected_scenarios, milestone_three_replay_scenarios,
    milestone_three_required_scenarios,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::support::reporting::ReplayParityStatus;
use crate::topology_operators::{
    TopologyEditFamily, TopologyEditNamingOutcome, TopologyEditRejectionClass,
};

pub(crate) fn certify_milestone_three_hostile_suite_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError>
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
        certify_milestone_three_split_collapse_churn_impl(
            &mut runtime_factory,
            &format!("{stem}.split_collapse"),
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
    let direct_acceptance_rows = build_direct_acceptance_rows(&scenario_reports);
    let validator_family_coverage_rows = build_validator_family_coverage_rows(&scenario_reports);
    let edit_branch_local_parity_rows = certify_milestone_three_branch_local_edit_parity_impl(
        &mut runtime_factory,
        stem,
        &scenario_reports,
    )?;
    let primitive_family_closure_rows =
        certify_milestone_three_primitive_family_closure_impl(&mut runtime_factory, stem)?;
    let edited_query_traversal_rows =
        certify_milestone_three_edited_query_traversal_impl(&mut runtime_factory, stem)?;
    let side_quest_closeout_report =
        certify_milestone_three_side_quest_closeout_impl(&mut runtime_factory, stem)?;
    let implemented_scenarios = scenario_reports
        .iter()
        .map(|report| report.scenario)
        .collect::<Vec<_>>();
    let missing_required_scenarios = milestone_three_required_scenarios()
        .iter()
        .filter(|scenario| !implemented_scenarios.contains(scenario))
        .map(|scenario| scenario.as_str().to_string())
        .collect::<Vec<_>>();
    let side_quest_gate_ready = side_quest_closeout_report.phase_three_ready;
    let milestone_three_return_gate_blocker_rows = build_milestone_three_return_gate_blocker_rows(
        &missing_required_scenarios,
        side_quest_gate_ready,
    );
    let coverage_complete = missing_required_scenarios.is_empty();
    let milestone_three_return_gate_ready = milestone_three_return_gate_blocker_rows.is_empty();

    let mut report = MilestoneThreeHostileSuiteReport {
        scenario_reports,
        coverage_rows,
        family_coverage_rows,
        rejection_distribution_rows,
        naming_distribution_rows,
        hostile_certification_category_rows: Vec::new(),
        primitive_family_closure_rows,
        topology_edit_digest_rows: direct_acceptance_rows.topology_edit_digest_rows,
        naming_edit_continuity_matrix_rows: direct_acceptance_rows
            .naming_edit_continuity_matrix_rows,
        rejected_edit_scope_report_rows: direct_acceptance_rows.rejected_edit_scope_report_rows,
        edit_replay_parity_rows: direct_acceptance_rows.edit_replay_parity_rows,
        edit_branch_local_parity_rows,
        edited_query_traversal_rows,
        validator_family_coverage_rows,
        changed_scope_coverage_rows: direct_acceptance_rows.changed_scope_coverage_rows,
        derived_region_coverage_rows: direct_acceptance_rows.derived_region_coverage_rows,
        determinism_rule_rows: direct_acceptance_rows.determinism_rule_rows,
        edit_breadth_counter_rows: direct_acceptance_rows.edit_breadth_counter_rows,
        edit_fallout_breadth_rows: direct_acceptance_rows.edit_fallout_breadth_rows,
        failure_locality_rows: direct_acceptance_rows.failure_locality_rows,
        side_quest_closeout_report,
        side_quest_gate_ready,
        missing_required_scenarios: missing_required_scenarios.clone(),
        milestone_three_return_gate_blocker_rows,
        implemented_scenario_count: implemented_scenarios.len(),
        required_scenario_count: milestone_three_required_scenarios().len(),
        coverage_complete,
        milestone_three_return_gate_ready,
    };
    report.hostile_certification_category_rows = build_hostile_certification_category_rows(&report);
    Ok(report)
}

pub(crate) fn certify_milestone_three_closeout_impl<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let report = certify_milestone_three_hostile_suite_impl(runtime_factory, stem)?;
    ensure_milestone_three_closeout_requirements(&report)?;
    Ok(report)
}

fn ensure_milestone_three_closeout_requirements(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    if !report.coverage_complete || !report.missing_required_scenarios.is_empty() {
        return Err(closeout_requirement_error(
            "required hostile scenario coverage is incomplete",
        ));
    }
    for scenario in milestone_three_required_scenarios() {
        if !report
            .coverage_rows
            .iter()
            .any(|row| row.scenario == *scenario)
        {
            return Err(closeout_requirement_error(&format!(
                "missing hostile coverage row for {}",
                scenario.as_str()
            )));
        }
    }
    for scenario in milestone_three_replay_scenarios() {
        let replay_verified = report.coverage_rows.iter().any(|row| {
            row.scenario == *scenario
                && row.replay_checked
                && row.replay_parity_status == ReplayParityStatus::Match
        });
        if !replay_verified {
            return Err(closeout_requirement_error(&format!(
                "missing replay parity match for {}",
                scenario.as_str()
            )));
        }
    }
    for scenario in milestone_three_rejected_scenarios() {
        let rejection_verified = report.coverage_rows.iter().any(|row| {
            row.scenario == *scenario
                && row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.rejection_class.is_some()
        });
        if !rejection_verified {
            return Err(closeout_requirement_error(&format!(
                "missing hostile rejection proof for {}",
                scenario.as_str()
            )));
        }
    }
    if report.family_coverage_rows.is_empty()
        || report.rejection_distribution_rows.is_empty()
        || report.naming_distribution_rows.is_empty()
    {
        return Err(closeout_requirement_error(
            "hostile aggregate coverage rows are incomplete",
        ));
    }
    ensure_direct_acceptance_proof_rows(report)?;
    ensure_branch_local_edit_parity_rows(report)?;
    ensure_primitive_family_closure_rows(report)?;
    ensure_edited_query_traversal_rows(report)?;
    ensure_validator_family_coverage_rows(report)?;
    ensure_hostile_certification_category_rows(report)?;
    let side_quest = &report.side_quest_closeout_report;
    if !side_quest.phase_three_ready
        || side_quest.domain_read_request_count == 0
        || side_quest.domain_read_parity_count == 0
    {
        return Err(closeout_requirement_error(
            "side-quest closeout report is not phase-three ready",
        ));
    }
    if !report.milestone_three_return_gate_ready
        || !report.milestone_three_return_gate_blocker_rows.is_empty()
    {
        return Err(closeout_requirement_error(
            "milestone three return gate is not ready",
        ));
    }
    Ok(())
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}

fn build_milestone_three_return_gate_blocker_rows(
    missing_required_scenarios: &[String],
    side_quest_gate_ready: bool,
) -> Vec<MilestoneThreeReturnGateBlockerRow> {
    let mut blockers = missing_required_scenarios
        .iter()
        .map(|scenario| {
            return_gate_blocker_row(
                &format!("missing_required_scenario:{scenario}"),
                "required hostile scenario has not certified yet",
            )
        })
        .collect::<Vec<_>>();
    if !side_quest_gate_ready {
        blockers.push(return_gate_blocker_row(
            "side_quest_closeout_not_ready",
            "Phase 3 side-quest closeout is not ready",
        ));
    }
    blockers
}

fn return_gate_blocker_row(blocker_name: &str, reason: &str) -> MilestoneThreeReturnGateBlockerRow {
    MilestoneThreeReturnGateBlockerRow {
        blocker_name: blocker_name.to_string(),
        reason: reason.to_string(),
        row_digest: format!("return_gate_blocker={blocker_name};reason={reason}"),
    }
}

fn build_coverage_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileCoverageRow> {
    reports
        .iter()
        .map(|report| MilestoneThreeHostileCoverageRow {
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
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileFamilyCoverageRow> {
    let mut rows = BTreeMap::<TopologyEditFamily, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        for family in &report.edit_families {
            rows.entry(*family).or_default().push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(family, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeHostileFamilyCoverageRow {
                family,
                scenario_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}

fn build_rejection_distribution_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileRejectionDistributionRow> {
    let mut rows =
        BTreeMap::<TopologyEditRejectionClass, Vec<MilestoneThreeHostileScenario>>::new();
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
            MilestoneThreeHostileRejectionDistributionRow {
                rejection_class,
                case_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}

fn build_naming_distribution_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileNamingDistributionRow> {
    let mut rows = BTreeMap::<TopologyEditNamingOutcome, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        rows.entry(report.continuity_outcome_class)
            .or_default()
            .push(report.scenario);
    }
    rows.into_iter()
        .map(|(continuity_outcome_class, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeHostileNamingDistributionRow {
                continuity_outcome_class,
                case_count: scenarios.len(),
                scenarios,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::build_milestone_three_return_gate_blocker_rows;

    #[test]
    fn return_gate_blocker_rows_include_missing_scenarios_and_side_quest_failure() {
        let blockers = build_milestone_three_return_gate_blocker_rows(
            &["SplitCollapseChurn".to_string()],
            false,
        );

        assert_eq!(
            blockers
                .iter()
                .map(|row| row.blocker_name.as_str())
                .collect::<Vec<_>>(),
            vec![
                "missing_required_scenario:SplitCollapseChurn",
                "side_quest_closeout_not_ready",
            ]
        );
        assert!(blockers.iter().all(|row| row
            .row_digest
            .starts_with(&format!("return_gate_blocker={};", row.blocker_name))));
    }

    #[test]
    fn return_gate_blocker_rows_are_empty_when_coverage_and_side_quest_are_ready() {
        let blockers = build_milestone_three_return_gate_blocker_rows(&[], true);

        assert!(blockers.is_empty());
    }
}
