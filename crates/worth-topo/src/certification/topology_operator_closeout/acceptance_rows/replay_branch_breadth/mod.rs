use crate::certification::error::TopologyCertificationError;
use crate::certification::ReplayParityStatus;

use std::collections::BTreeSet;

use super::super::replay_branch_breadth_row::MilestoneThreeReplayBranchBreadthRow;
use super::super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport};
use super::super::{milestone_three_rejected_scenarios, milestone_three_required_scenarios};

pub(in crate::certification::topology_operator_closeout) fn build_replay_branch_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<MilestoneThreeReplayBranchBreadthRow> {
    vec![replay_branch_breadth_row(report)]
}

pub(in crate::certification::topology_operator_closeout) fn ensure_replay_branch_breadth_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    if report.replay_branch_breadth_rows.len() != 1 {
        return Err(replay_branch_breadth_error(&format!(
            "expected one replay/branch breadth row, found {}",
            report.replay_branch_breadth_rows.len()
        )));
    }
    let row = report
        .replay_branch_breadth_rows
        .first()
        .expect("length checked");
    let expected_row = replay_branch_breadth_row(report);
    if row != &expected_row {
        return Err(replay_branch_breadth_error(
            "replay/branch breadth row drifted from source evidence",
        ));
    }
    ensure_replay_breadth_is_complete(row)?;
    ensure_branch_local_breadth_is_complete(row)
}

fn replay_branch_breadth_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeReplayBranchBreadthRow {
    let replay_checked_scenario_count = report
        .edit_replay_parity_rows
        .iter()
        .filter(|row| row.replay_checked && row.parity_status == ReplayParityStatus::Match)
        .count();
    let replay_step_count = report
        .edit_replay_parity_rows
        .iter()
        .map(|row| row.step_count)
        .sum();
    let replay_comparison_step_count = report
        .edit_replay_parity_rows
        .iter()
        .map(|row| row.replay_step_count)
        .sum();
    let replay_mismatch_count = report
        .edit_replay_parity_rows
        .iter()
        .map(|row| row.mismatch_count)
        .sum();
    let accepted_branch_local_scenarios = report
        .edit_branch_local_parity_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
                && row.rejection_class.is_none()
                && row.branch_head_diverged_from_main
                && row.branch_truth_digest.is_some()
                && row.scenario.is_some()
        })
        .filter_map(|row| row.scenario)
        .collect::<BTreeSet<_>>();
    let accepted_branch_local_row_count = accepted_branch_local_scenarios.len();
    let rejected_branch_local_scenarios = report
        .edit_branch_local_parity_rows
        .iter()
        .filter_map(|row| {
            (row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.rejection_class.is_some()
                && row.branch_head_unchanged_after_rejection
                && row.branch_truth_digest.is_none()
                && row.scenario.is_some())
            .then_some(row.scenario.expect("scenario checked"))
        })
        .collect::<BTreeSet<_>>();
    let rejected_branch_local_row_count = rejected_branch_local_scenarios.len();
    let branch_truth_digest_count = report
        .edit_branch_local_parity_rows
        .iter()
        .filter(|row| row.branch_truth_digest.is_some())
        .count();
    let unchanged_rejected_branch_count = report
        .edit_branch_local_parity_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.branch_head_unchanged_after_rejection
        })
        .count();
    let required_scenario_count = milestone_three_required_scenarios().len();
    let required_rejected_branch_local_count = milestone_three_rejected_scenarios().len();
    let required_accepted_branch_local_count =
        required_scenario_count - required_rejected_branch_local_count;
    let branch_local_row_count = report.edit_branch_local_parity_rows.len();

    MilestoneThreeReplayBranchBreadthRow {
        required_scenario_count,
        replay_checked_scenario_count,
        replay_step_count,
        replay_comparison_step_count,
        replay_mismatch_count,
        branch_local_row_count,
        accepted_branch_local_row_count,
        required_accepted_branch_local_count,
        rejected_branch_local_row_count,
        required_rejected_branch_local_count,
        branch_truth_digest_count,
        unchanged_rejected_branch_count,
        row_digest: format!(
            "required_scenarios={required_scenario_count};replay_checked={replay_checked_scenario_count};replay_steps={replay_step_count};replay_comparison_steps={replay_comparison_step_count};replay_mismatches={replay_mismatch_count};branch_rows={branch_local_row_count};accepted_branch_rows={accepted_branch_local_row_count};required_accepted_branch_rows={required_accepted_branch_local_count};rejected_branch_rows={rejected_branch_local_row_count};required_rejected_branch_rows={required_rejected_branch_local_count};branch_truth_digests={branch_truth_digest_count};unchanged_rejected_branches={unchanged_rejected_branch_count}",
        ),
    }
}

fn ensure_replay_breadth_is_complete(
    row: &MilestoneThreeReplayBranchBreadthRow,
) -> Result<(), TopologyCertificationError> {
    if row.replay_checked_scenario_count != row.required_scenario_count
        || row.replay_step_count == 0
        || row.replay_step_count != row.replay_comparison_step_count
        || row.replay_mismatch_count != 0
    {
        return Err(replay_branch_breadth_error(
            "replay breadth does not cover every required scenario with matching replay steps",
        ));
    }
    Ok(())
}

fn ensure_branch_local_breadth_is_complete(
    row: &MilestoneThreeReplayBranchBreadthRow,
) -> Result<(), TopologyCertificationError> {
    if row.accepted_branch_local_row_count != row.required_accepted_branch_local_count
        || row.branch_local_row_count
            != row.required_accepted_branch_local_count + row.required_rejected_branch_local_count
        || row.rejected_branch_local_row_count != row.required_rejected_branch_local_count
        || row.branch_truth_digest_count != row.accepted_branch_local_row_count
        || row.unchanged_rejected_branch_count != row.required_rejected_branch_local_count
    {
        return Err(replay_branch_breadth_error(
            "branch-local breadth does not prove accepted divergence and rejected locality",
        ));
    }
    Ok(())
}

fn replay_branch_breadth_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three replay/branch breadth failed: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use crate::facade::certify_milestone_three_hostile_suite;
    use crate::validation::reference_integrity::build_milestone_one_runtime;

    use super::ensure_replay_branch_breadth_rows;

    #[test]
    fn replay_branch_breadth_gate_rejects_replay_source_drift() {
        let mut report = certified_report("m3.replay_branch_breadth.replay_drift");

        let replay_row = report
            .edit_replay_parity_rows
            .first_mut()
            .expect("hostile suite should include replay rows");
        replay_row.mismatch_count = 1;

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "replay/branch breadth must reject replay rows that drift from match evidence"
        );
    }

    #[test]
    fn replay_branch_breadth_gate_rejects_missing_accepted_branch_evidence() {
        let mut report = certified_report("m3.replay_branch_breadth.missing_accepted_branch");

        report
            .edit_branch_local_parity_rows
            .retain(|row| row.outcome_class != super::MilestoneThreeHostileOutcomeClass::Accepted);

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "replay/branch breadth must require accepted branch-local divergence evidence"
        );
    }

    #[test]
    fn replay_branch_breadth_gate_rejects_partial_accepted_branch_evidence() {
        let mut report = certified_report("m3.replay_branch_breadth.partial_accepted_branch");

        let mut removed = false;
        report.edit_branch_local_parity_rows.retain(|row| {
            if !removed && row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Accepted {
                removed = true;
                false
            } else {
                true
            }
        });

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "replay/branch breadth must require every accepted scenario branch row"
        );
    }

    #[test]
    fn replay_branch_breadth_gate_rejects_duplicate_accepted_scenario_substitution() {
        let mut report = certified_report("m3.replay_branch_breadth.duplicate_accepted_scenario");

        let duplicate = report
            .edit_branch_local_parity_rows
            .iter()
            .find(|row| row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Accepted)
            .expect("accepted branch-local evidence")
            .clone();
        let mut removed = false;
        report.edit_branch_local_parity_rows.retain(|row| {
            if !removed && row.scenario != duplicate.scenario {
                removed = row.outcome_class == super::MilestoneThreeHostileOutcomeClass::Accepted;
                !removed
            } else {
                true
            }
        });
        report.edit_branch_local_parity_rows.push(duplicate);

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "duplicate accepted scenario rows must not substitute for missing scenario coverage"
        );
    }

    #[test]
    fn replay_branch_breadth_gate_rejects_missing_rejected_branch_evidence() {
        let mut report = certified_report("m3.replay_branch_breadth.missing_rejected_branch");

        report
            .edit_branch_local_parity_rows
            .retain(|row| row.outcome_class != super::MilestoneThreeHostileOutcomeClass::Rejected);

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "replay/branch breadth must require rejected branch-local locality evidence"
        );
    }

    #[test]
    fn replay_branch_breadth_gate_rejects_duplicate_aggregate_rows() {
        let mut report = certified_report("m3.replay_branch_breadth.duplicate");

        let duplicate = report
            .replay_branch_breadth_rows
            .first()
            .expect("hostile suite should include replay/branch breadth row")
            .clone();
        report.replay_branch_breadth_rows.push(duplicate);

        assert!(
            ensure_replay_branch_breadth_rows(&report).is_err(),
            "replay/branch breadth must reject duplicate aggregate rows"
        );
    }

    fn certified_report(
        stem: &str,
    ) -> crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport {
        certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect("milestone one runtime builder"),
            stem,
        )
        .expect("hostile suite should certify before tampering")
    }
}




