use crate::certification::support::parity::digest_derived_validation_report;
use crate::certification::{ReplayParityStatus, TopologyCertificationError};

use super::super::milestone_three_required_scenarios;
use super::super::report::{
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeMutationFalloutBreadthRow,
};
use super::derived_reuse_rows::MilestoneThreeDerivedReuseLegalityRow;

pub(in crate::certification::topology_operator_closeout) fn build_derived_reuse_legality_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
    fallout_rows: &[MilestoneThreeMutationFalloutBreadthRow],
) -> Vec<MilestoneThreeDerivedReuseLegalityRow> {
    reports
        .iter()
        .map(|report| {
            let fallout = fallout_rows
                .iter()
                .find(|row| row.scenario == report.scenario)
                .expect("fallout row should exist for every hostile scenario");
            let replay = &report.mutation_replay_parity_report;
            let replay_materialized_topology_equivalent = replay.parity_status
                == ReplayParityStatus::Match
                && replay.final_materialized_topology_digest
                    == replay.replay_final_materialized_topology_digest;
            let derived_validation_digest = report
                .derived_validation_report
                .as_ref()
                .map(digest_derived_validation_report);

            MilestoneThreeDerivedReuseLegalityRow {
                scenario: report.scenario,
                recompute_suppression_claimed: false,
                equivalence_contract_required: false,
                replay_materialized_topology_equivalent,
                fallback_count: fallout.fallback_count,
                fallout_class: fallout.fallout_class,
                derived_validation_digest,
                row_digest: format!(
                    "scenario={};suppression_claimed=false;equivalence_required=false;fallout_class={:?};fallback_count={};replay_equivalent={}",
                    report.scenario.as_str(),
                    fallout.fallout_class,
                    fallout.fallback_count,
                    replay_materialized_topology_equivalent
                ),
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_derived_reuse_legality_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        let row = report
            .derived_reuse_legality_rows
            .iter()
            .find(|row| row.scenario == *scenario)
            .ok_or_else(|| {
                closeout_requirement_error(&format!(
                    "missing derived reuse legality row for {}",
                    scenario.as_str()
                ))
            })?;
        if !row.replay_materialized_topology_equivalent {
            return Err(closeout_requirement_error(&format!(
                "derived reuse legality row lacks replay equivalence for {}",
                scenario.as_str()
            )));
        }
        if row.recompute_suppression_claimed
            && (!row.equivalence_contract_required || row.fallback_count > 0)
        {
            return Err(closeout_requirement_error(&format!(
                "derived reuse legality row has an unsupported suppression claim for {}",
                scenario.as_str()
            )));
        }
        if !row.row_digest.contains("suppression_claimed=false") {
            return Err(closeout_requirement_error(&format!(
                "derived reuse legality row is not suppression-honest for {}",
                scenario.as_str()
            )));
        }
    }
    Ok(())
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}
