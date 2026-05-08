use crate::certification::error::TopologyCertificationError;
use crate::edit::TopologyEditNamingOutcome;

use super::aggregate_acceptance::build_aggregate_acceptance_rows;
use super::report::{
    MilestoneThreeChangedScopeCoverageRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeEditBreadthCounterRow, MilestoneThreeEditReplayParityRow,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeTopologyEditDigestRow,
};
use super::{milestone_three_rejected_scenarios, milestone_three_required_scenarios};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MilestoneThreeDirectAcceptanceRows {
    pub topology_edit_digest_rows: Vec<MilestoneThreeTopologyEditDigestRow>,
    pub naming_edit_continuity_matrix_rows: Vec<MilestoneThreeNamingContinuityMatrixRow>,
    pub rejected_edit_scope_report_rows: Vec<MilestoneThreeRejectedEditScopeReportRow>,
    pub edit_replay_parity_rows: Vec<MilestoneThreeEditReplayParityRow>,
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub edit_breadth_counter_rows: Vec<MilestoneThreeEditBreadthCounterRow>,
    pub failure_locality_rows: Vec<MilestoneThreeFailureLocalityRow>,
}

pub(super) fn build_direct_acceptance_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> MilestoneThreeDirectAcceptanceRows {
    let aggregate_rows = build_aggregate_acceptance_rows(reports);
    MilestoneThreeDirectAcceptanceRows {
        topology_edit_digest_rows: build_topology_edit_digest_rows(reports),
        naming_edit_continuity_matrix_rows: build_naming_edit_continuity_matrix_rows(reports),
        rejected_edit_scope_report_rows: build_rejected_edit_scope_report_rows(reports),
        edit_replay_parity_rows: build_edit_replay_parity_rows(reports),
        changed_scope_coverage_rows: aggregate_rows.changed_scope_coverage_rows,
        derived_region_coverage_rows: aggregate_rows.derived_region_coverage_rows,
        edit_breadth_counter_rows: aggregate_rows.edit_breadth_counter_rows,
        failure_locality_rows: aggregate_rows.failure_locality_rows,
    }
}

pub(super) fn ensure_direct_acceptance_proof_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        ensure_required_scenario_rows(report, *scenario)?;
    }
    for scenario in milestone_three_rejected_scenarios() {
        ensure_rejected_scenario_rows(report, *scenario)?;
    }
    if report.changed_scope_coverage_rows.is_empty() {
        return Err(closeout_requirement_error(
            "missing changed-scope vocabulary coverage rows",
        ));
    }
    if report.derived_region_coverage_rows.is_empty() {
        return Err(closeout_requirement_error(
            "missing derived-region vocabulary coverage rows",
        ));
    }
    Ok(())
}

fn ensure_required_scenario_rows(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    if !report
        .topology_edit_digest_rows
        .iter()
        .any(|row| row.scenario == scenario && row.topology_edit_digest.contract_count > 0)
    {
        return Err(closeout_requirement_error(&format!(
            "missing topology edit digest row for {}",
            scenario.as_str()
        )));
    }
    if !report
        .naming_edit_continuity_matrix_rows
        .iter()
        .any(|row| row.scenario == scenario && !row.naming_edit_continuity_matrix.rows.is_empty())
    {
        return Err(closeout_requirement_error(&format!(
            "missing naming edit continuity matrix row for {}",
            scenario.as_str()
        )));
    }
    if !report
        .edit_replay_parity_rows
        .iter()
        .any(|row| row.scenario == scenario)
    {
        return Err(closeout_requirement_error(&format!(
            "missing edit replay parity row for {}",
            scenario.as_str()
        )));
    }
    if !report
        .edit_breadth_counter_rows
        .iter()
        .any(|row| row.scenario == scenario && row.contract_count > 0)
    {
        return Err(closeout_requirement_error(&format!(
            "missing edit breadth counter row for {}",
            scenario.as_str()
        )));
    }
    Ok(())
}

fn ensure_rejected_scenario_rows(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    if !report
        .rejected_edit_scope_report_rows
        .iter()
        .any(|row| row.scenario == scenario && !row.rejected_edit_scope_report.rows.is_empty())
    {
        return Err(closeout_requirement_error(&format!(
            "missing rejected edit scope report row for {}",
            scenario.as_str()
        )));
    }
    if !report.failure_locality_rows.iter().any(|row| {
        row.scenario == scenario && row.scope_row_count > 0 && !row.changed_scopes.is_empty()
    }) {
        return Err(closeout_requirement_error(&format!(
            "missing failure locality row for {}",
            scenario.as_str()
        )));
    }
    Ok(())
}

fn build_topology_edit_digest_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeTopologyEditDigestRow> {
    reports
        .iter()
        .map(|report| MilestoneThreeTopologyEditDigestRow {
            scenario: report.scenario,
            topology_edit_digest: report.topology_edit_digest.clone(),
            row_digest: format!(
                "scenario={};topology_edit_digest={}",
                report.scenario.as_str(),
                report.topology_edit_digest.digest.digest_hex
            ),
        })
        .collect()
}

fn build_naming_edit_continuity_matrix_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeNamingContinuityMatrixRow> {
    reports
        .iter()
        .map(|report| MilestoneThreeNamingContinuityMatrixRow {
            scenario: report.scenario,
            naming_edit_continuity_matrix: report.naming_edit_continuity_matrix.clone(),
            continuity_outcome_class: report.continuity_outcome_class,
            continuity_rejection_class: report.continuity_rejection_class,
            row_digest: format!(
                "scenario={};naming_outcome={};rows={}",
                report.scenario.as_str(),
                naming_outcome_label(report.continuity_outcome_class),
                report.naming_edit_continuity_matrix.rows.len()
            ),
        })
        .collect()
}

fn build_rejected_edit_scope_report_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeRejectedEditScopeReportRow> {
    reports
        .iter()
        .filter_map(|report| {
            let rejection_class = report.rejection_class?;
            let rejected_edit_scope_report = report.rejected_edit_scope_report.clone()?;
            Some(MilestoneThreeRejectedEditScopeReportRow {
                scenario: report.scenario,
                rejection_class,
                row_digest: format!(
                    "scenario={};rejection_class={:?};scope_rows={}",
                    report.scenario.as_str(),
                    rejection_class,
                    rejected_edit_scope_report.rows.len()
                ),
                rejected_edit_scope_report,
            })
        })
        .collect()
}

fn build_edit_replay_parity_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeEditReplayParityRow> {
    reports
        .iter()
        .map(|report| {
            let replay = &report.edit_replay_parity_report;
            MilestoneThreeEditReplayParityRow {
                scenario: report.scenario,
                replay_checked: replay.replay_checked,
                parity_status: replay.parity_status,
                mismatch_count: replay.mismatch_count,
                step_count: replay.step_rows.len(),
                replay_step_count: replay.replay_step_rows.len(),
                row_digest: format!(
                    "scenario={};replay_checked={};parity_status={:?};mismatch_count={}",
                    report.scenario.as_str(),
                    replay.replay_checked,
                    replay.parity_status,
                    replay.mismatch_count
                ),
            }
        })
        .collect()
}

fn naming_outcome_label(outcome: TopologyEditNamingOutcome) -> &'static str {
    match outcome {
        TopologyEditNamingOutcome::Preserved => "preserved",
        TopologyEditNamingOutcome::Ambiguous => "ambiguous",
        TopologyEditNamingOutcome::Rejected => "rejected",
    }
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}
