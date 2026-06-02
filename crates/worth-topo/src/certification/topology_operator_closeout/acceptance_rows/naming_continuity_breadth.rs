use crate::certification::error::TopologyCertificationError;

use super::super::milestone_three_required_scenarios;
use super::super::naming_continuity_breadth_row::MilestoneThreeNamingContinuityBreadthRow;
use super::super::report::{MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport};

pub(in crate::certification::topology_operator_closeout) fn build_naming_continuity_breadth_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeNamingContinuityBreadthRow> {
    reports
        .iter()
        .map(|report| {
            let matrix = report.naming_mutation_continuity_matrix();
            MilestoneThreeNamingContinuityBreadthRow {
                scenario: report.scenario,
                continuity_row_count: matrix.rows.len(),
                preserved_count: matrix.preserved_count,
                ambiguous_count: matrix.ambiguous_count,
                rejected_count: matrix.rejected_count,
                naming_scope_count: report.topology_mutation_digest().naming_scope_count,
                replay_step_count: report.mutation_replay_parity_report.step_rows.len(),
                replay_checked: report.mutation_replay_parity_report.replay_checked,
                outcome_class: report.continuity_outcome_class(),
                row_digest: format!(
                    "scenario={};continuity_rows={};preserved={};ambiguous={};rejected={};naming_scopes={};replay_steps={};replay_checked={}",
                    report.scenario.as_str(),
                    matrix.rows.len(),
                    matrix.preserved_count,
                    matrix.ambiguous_count,
                    matrix.rejected_count,
                    report.topology_mutation_digest().naming_scope_count,
                    report.mutation_replay_parity_report.step_rows.len(),
                    report.mutation_replay_parity_report.replay_checked
                ),
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_naming_continuity_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        let row = report
            .naming_continuity_breadth_rows
            .iter()
            .find(|row| row.scenario == *scenario)
            .ok_or_else(|| {
                naming_breadth_error(&format!(
                    "missing naming continuity breadth row for {}",
                    scenario.as_str()
                ))
            })?;
        if row.continuity_row_count == 0
            || row.naming_scope_count == 0
            || !row.replay_checked
            || row.continuity_row_count
                != row.preserved_count + row.ambiguous_count + row.rejected_count
            || !row.row_digest.contains("continuity_rows=")
            || !row.row_digest.contains("naming_scopes=")
        {
            return Err(naming_breadth_error(&format!(
                "naming continuity breadth row is not proof-bearing for {}",
                scenario.as_str()
            )));
        }
    }
    Ok(())
}

fn naming_breadth_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three naming continuity breadth failed: {reason}"
    ))
}
