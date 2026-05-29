use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::TopologyEditNamingOutcome;

use super::super::derived_fallout::MilestoneThreeDerivedFallbackPolicyDenialRow;
use super::super::derived_fallout::MilestoneThreeDerivedReuseLegalityRow;
use super::super::derived_fallout::MilestoneThreeDerivedWorkBreadthRow;
use super::super::derived_fallout::{
    build_derived_reuse_legality_rows, ensure_derived_reuse_legality_rows,
};
use super::super::derived_fallout::{
    build_derived_work_breadth_rows, ensure_derived_work_breadth_rows,
};
use super::super::derived_fallout::{
    build_fallback_policy_denial_rows, ensure_fallback_policy_denial_rows,
};
use super::super::naming_continuity_breadth_row::MilestoneThreeNamingContinuityBreadthRow;
use super::super::report::{
    MilestoneThreeChangedScopeCoverageRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeDeterminismRuleKind, MilestoneThreeDeterminismRuleRow,
    MilestoneThreeEditBreadthCounterRow, MilestoneThreeEditFalloutBreadthRow,
    MilestoneThreeEditFalloutClass, MilestoneThreeEditReplayParityRow,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeTopologyEditDigestRow,
};
use super::super::{milestone_three_rejected_scenarios, milestone_three_required_scenarios};
use super::aggregate_acceptance::build_aggregate_acceptance_rows;
use super::naming_continuity_breadth::{
    build_naming_continuity_breadth_rows, ensure_naming_continuity_breadth_rows,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::certification::topology_operator_closeout) struct MilestoneThreeDirectAcceptanceRows {
    pub topology_edit_digest_rows: Vec<MilestoneThreeTopologyEditDigestRow>,
    pub naming_edit_continuity_matrix_rows: Vec<MilestoneThreeNamingContinuityMatrixRow>,
    pub naming_continuity_breadth_rows: Vec<MilestoneThreeNamingContinuityBreadthRow>,
    pub rejected_edit_scope_report_rows: Vec<MilestoneThreeRejectedEditScopeReportRow>,
    pub edit_replay_parity_rows: Vec<MilestoneThreeEditReplayParityRow>,
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub determinism_rule_rows: Vec<MilestoneThreeDeterminismRuleRow>,
    pub edit_breadth_counter_rows: Vec<MilestoneThreeEditBreadthCounterRow>,
    pub edit_fallout_breadth_rows: Vec<MilestoneThreeEditFalloutBreadthRow>,
    pub derived_fallback_policy_denial_rows: Vec<MilestoneThreeDerivedFallbackPolicyDenialRow>,
    pub derived_reuse_legality_rows: Vec<MilestoneThreeDerivedReuseLegalityRow>,
    pub derived_work_breadth_rows: Vec<MilestoneThreeDerivedWorkBreadthRow>,
    pub failure_locality_rows: Vec<MilestoneThreeFailureLocalityRow>,
}

pub(in crate::certification::topology_operator_closeout) fn build_direct_acceptance_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> MilestoneThreeDirectAcceptanceRows {
    let aggregate_rows = build_aggregate_acceptance_rows(reports);
    let derived_reuse_legality_rows =
        build_derived_reuse_legality_rows(reports, &aggregate_rows.edit_fallout_breadth_rows);
    let derived_work_breadth_rows =
        build_derived_work_breadth_rows(reports, &aggregate_rows.edit_fallout_breadth_rows);
    let derived_fallback_policy_denial_rows =
        build_fallback_policy_denial_rows(&aggregate_rows.edit_fallout_breadth_rows);
    MilestoneThreeDirectAcceptanceRows {
        topology_edit_digest_rows: build_topology_edit_digest_rows(reports),
        naming_edit_continuity_matrix_rows: build_naming_edit_continuity_matrix_rows(reports),
        naming_continuity_breadth_rows: build_naming_continuity_breadth_rows(reports),
        rejected_edit_scope_report_rows: build_rejected_edit_scope_report_rows(reports),
        edit_replay_parity_rows: build_edit_replay_parity_rows(reports),
        changed_scope_coverage_rows: aggregate_rows.changed_scope_coverage_rows,
        derived_region_coverage_rows: aggregate_rows.derived_region_coverage_rows,
        determinism_rule_rows: aggregate_rows.determinism_rule_rows,
        edit_breadth_counter_rows: aggregate_rows.edit_breadth_counter_rows,
        edit_fallout_breadth_rows: aggregate_rows.edit_fallout_breadth_rows,
        derived_fallback_policy_denial_rows,
        derived_reuse_legality_rows,
        derived_work_breadth_rows,
        failure_locality_rows: aggregate_rows.failure_locality_rows,
    }
}

pub(in crate::certification::topology_operator_closeout) fn ensure_direct_acceptance_proof_rows(
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
    ensure_edit_fallout_breadth_rows(report)?;
    ensure_naming_continuity_breadth_rows(report)?;
    ensure_fallback_policy_denial_rows(report)?;
    ensure_derived_reuse_legality_rows(report)?;
    ensure_derived_work_breadth_rows(report)?;
    ensure_determinism_rule_rows(report)?;
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

fn ensure_edit_fallout_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        let row = report
            .edit_fallout_breadth_rows
            .iter()
            .find(|row| row.scenario == *scenario)
            .ok_or_else(|| {
                closeout_requirement_error(&format!(
                    "missing edit fallout breadth row for {}",
                    scenario.as_str()
                ))
            })?;
        let has_honest_fallout_class = match row.fallout_class {
            MilestoneThreeEditFalloutClass::Localized | MilestoneThreeEditFalloutClass::Widened => {
                row.fallback_count == 0
            }
            MilestoneThreeEditFalloutClass::WholeViewFallback
            | MilestoneThreeEditFalloutClass::WholeHistoryFallback => row.fallback_count > 0,
            MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork => {
                row.derived_validation_row_count == 0 && row.fallback_count == 0
            }
        };
        if !has_honest_fallout_class || row.locality_claim_mismatch {
            return Err(closeout_requirement_error(&format!(
                "edit fallout breadth row is not basis-honest for {}",
                scenario.as_str()
            )));
        }
        let fallback_rejection_matches_policy = row.fallback_rejection_class
            == row.fallback_policy_exceeded.then_some(
                crate::topology_operators::TopologyEditRejectionClass::DerivedFallbackExceeded,
            );
        if row.fallback_policy_exceeded || !fallback_rejection_matches_policy {
            return Err(closeout_requirement_error(&format!(
                "edit fallout breadth row exceeded fallback policy for {}",
                scenario.as_str()
            )));
        }
    }
    Ok(())
}

fn ensure_determinism_rule_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        let stable_order = report.determinism_rule_rows.iter().any(|row| {
            row.scenario == *scenario
                && row.rule_kind == MilestoneThreeDeterminismRuleKind::StableEditOrder
                && row.replay_verified
                && row.evidence_count > 0
                && row.row_digest.contains("order_policy=sequence_preserving")
        });
        if !stable_order {
            return Err(closeout_requirement_error(&format!(
                "missing stable edit order determinism row for {}",
                scenario.as_str()
            )));
        }
        let stable_digest = report.determinism_rule_rows.iter().any(|row| {
            row.scenario == *scenario
                && row.rule_kind == MilestoneThreeDeterminismRuleKind::StableEditDigest
                && row.replay_verified
                && row.evidence_count > 0
        });
        if !stable_digest {
            return Err(closeout_requirement_error(&format!(
                "missing stable edit digest determinism row for {}",
                scenario.as_str()
            )));
        }
    }
    for scenario in milestone_three_rejected_scenarios() {
        let stable_rejection = report.determinism_rule_rows.iter().any(|row| {
            row.scenario == *scenario
                && row.rule_kind == MilestoneThreeDeterminismRuleKind::StableRejectionClassification
                && row.replay_verified
                && row.diagnostic_classification_stable
        });
        if !stable_rejection {
            return Err(closeout_requirement_error(&format!(
                "missing stable rejection classification determinism row for {}",
                scenario.as_str()
            )));
        }
    }
    if !report.determinism_rule_rows.iter().any(|row| {
        row.rule_kind == MilestoneThreeDeterminismRuleKind::AmbiguousTieBreakEvidence
            && row.replay_verified
            && row.diagnostic_classification_stable
            && row.tie_break_evidence_stable
    }) {
        return Err(closeout_requirement_error(
            "missing ambiguous local rewire tie-break determinism evidence row",
        ));
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




