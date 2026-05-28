use crate::certification::TopologyCertificationError;

use super::super::milestone_three_required_scenarios;
use super::super::report::{
    MilestoneThreeEditFalloutBreadthRow, MilestoneThreeEditFalloutClass,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
};
use super::derived_work_breadth_rows::{
    MilestoneThreeDerivedWorkBreadthClass, MilestoneThreeDerivedWorkBreadthRow,
};

pub(in crate::certification::topology_operator_closeout) fn build_derived_work_breadth_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
    fallout_rows: &[MilestoneThreeEditFalloutBreadthRow],
) -> Vec<MilestoneThreeDerivedWorkBreadthRow> {
    reports
        .iter()
        .map(|report| build_derived_work_breadth_row(report, fallout_rows))
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_derived_work_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        let row = report
            .derived_work_breadth_rows
            .iter()
            .find(|row| row.scenario == *scenario)
            .ok_or_else(|| {
                closeout_requirement_error(&format!(
                    "missing derived work breadth row for {}",
                    scenario.as_str()
                ))
            })?;
        if row.declared_derived_region_count == 0 || row.declared_changed_scope_count == 0 {
            return Err(closeout_requirement_error(&format!(
                "derived work breadth row lacks declared scope for {}",
                scenario.as_str()
            )));
        }
        if row.locality_claim_mismatch && row.fallback_count == 0 {
            return Err(closeout_requirement_error(&format!(
                "derived work breadth row mismatched locality without fallback for {}",
                scenario.as_str()
            )));
        }
        if !row
            .row_digest
            .starts_with(&format!("scenario={};", scenario.as_str()))
        {
            return Err(closeout_requirement_error(&format!(
                "derived work breadth row digest is malformed for {}",
                scenario.as_str()
            )));
        }
    }
    Ok(())
}

fn build_derived_work_breadth_row(
    report: &MilestoneThreeHostileScenarioReport,
    fallout_rows: &[MilestoneThreeEditFalloutBreadthRow],
) -> MilestoneThreeDerivedWorkBreadthRow {
    let fallout = fallout_row_for_scenario(report, fallout_rows);
    let breadth_claim = classify_derived_work_breadth_claim(fallout);

    MilestoneThreeDerivedWorkBreadthRow {
        scenario: report.scenario,
        invalidation_breadth_class: breadth_claim.invalidation_breadth_class,
        rebuild_breadth_class: breadth_claim.rebuild_breadth_class,
        declared_changed_scope_count: report.topology_edit_digest.changed_scope_count,
        declared_derived_region_count: fallout.declared_derived_region_count,
        actual_derived_validation_row_count: fallout.derived_validation_row_count,
        fallback_count: fallout.fallback_count,
        locality_claimed: breadth_claim.locality_claimed,
        locality_claim_mismatch: breadth_claim.locality_claim_mismatch,
        row_digest: derived_work_breadth_row_digest(report, fallout, breadth_claim),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DerivedWorkBreadthClaim {
    invalidation_breadth_class: MilestoneThreeDerivedWorkBreadthClass,
    rebuild_breadth_class: MilestoneThreeDerivedWorkBreadthClass,
    locality_claimed: bool,
    locality_claim_mismatch: bool,
}

fn classify_derived_work_breadth_claim(
    fallout: &MilestoneThreeEditFalloutBreadthRow,
) -> DerivedWorkBreadthClaim {
    let invalidation_breadth_class = invalidation_breadth_class(fallout);
    let rebuild_breadth_class = rebuild_breadth_class(fallout);
    let locality_claimed =
        invalidation_breadth_class == MilestoneThreeDerivedWorkBreadthClass::DeclaredRegions;
    let locality_claim_mismatch =
        locality_claimed && rebuild_breadth_class != invalidation_breadth_class;

    DerivedWorkBreadthClaim {
        invalidation_breadth_class,
        rebuild_breadth_class,
        locality_claimed,
        locality_claim_mismatch,
    }
}

fn fallout_row_for_scenario<'a>(
    report: &MilestoneThreeHostileScenarioReport,
    fallout_rows: &'a [MilestoneThreeEditFalloutBreadthRow],
) -> &'a MilestoneThreeEditFalloutBreadthRow {
    fallout_rows
        .iter()
        .find(|row| row.scenario == report.scenario)
        .expect("fallout row should exist for every hostile scenario")
}

fn derived_work_breadth_row_digest(
    report: &MilestoneThreeHostileScenarioReport,
    fallout: &MilestoneThreeEditFalloutBreadthRow,
    breadth_claim: DerivedWorkBreadthClaim,
) -> String {
    format!(
        "scenario={};invalidation={:?};rebuild={:?};declared_changed_scopes={};declared_regions={};actual_validation_rows={};fallback_count={};locality_claim_mismatch={}",
        report.scenario.as_str(),
        breadth_claim.invalidation_breadth_class,
        breadth_claim.rebuild_breadth_class,
        report.topology_edit_digest.changed_scope_count,
        fallout.declared_derived_region_count,
        fallout.derived_validation_row_count,
        fallout.fallback_count,
        breadth_claim.locality_claim_mismatch
    )
}

fn invalidation_breadth_class(
    fallout: &MilestoneThreeEditFalloutBreadthRow,
) -> MilestoneThreeDerivedWorkBreadthClass {
    match fallout.fallout_class {
        MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork => {
            MilestoneThreeDerivedWorkBreadthClass::RejectedBeforeDerivedWork
        }
        _ => MilestoneThreeDerivedWorkBreadthClass::DeclaredRegions,
    }
}

fn rebuild_breadth_class(
    fallout: &MilestoneThreeEditFalloutBreadthRow,
) -> MilestoneThreeDerivedWorkBreadthClass {
    match fallout.fallout_class {
        MilestoneThreeEditFalloutClass::Localized | MilestoneThreeEditFalloutClass::Widened => {
            MilestoneThreeDerivedWorkBreadthClass::DeclaredRegions
        }
        MilestoneThreeEditFalloutClass::WholeViewFallback => {
            MilestoneThreeDerivedWorkBreadthClass::WholeViewFallback
        }
        MilestoneThreeEditFalloutClass::WholeHistoryFallback => {
            MilestoneThreeDerivedWorkBreadthClass::WholeHistoryFallback
        }
        MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork => {
            MilestoneThreeDerivedWorkBreadthClass::RejectedBeforeDerivedWork
        }
    }
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}




