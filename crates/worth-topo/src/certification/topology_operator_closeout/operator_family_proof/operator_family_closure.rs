use super::super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport};
use super::operator_family_closure_types::MilestoneThreeOperatorFamilyClosureRow;
use crate::certification::error::TopologyCertificationError;
use crate::projection::runtime_boundary::query_runtime::TopologyRuntimeSupport;
use crate::topology_operators::TopologyMutationFamily;

pub(in crate::certification::topology_operator_closeout) fn build_operator_family_closure_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<MilestoneThreeOperatorFamilyClosureRow> {
    let current_head_support = TopologyRuntimeSupport::current_head_authoritative();
    let snapshot_support = TopologyRuntimeSupport::snapshot_read_only();
    TopologyMutationFamily::ALL
        .into_iter()
        .map(|family| {
            let current_head_row = current_head_support
                .query_mutation_family_support_rows()
                .iter()
                .find(|row| row.family() == family)
                .expect("current-head support should cover every family");
            let snapshot_row = snapshot_support
                .query_mutation_family_support_rows()
                .iter()
                .find(|row| row.family() == family)
                .expect("snapshot support should cover every family");
            let direct_scenarios = direct_scenario_labels(report, family);
            let mut legal_evidence_labels = vec![format!(
                "current_head_support={:?}",
                current_head_row.status()
            )];
            legal_evidence_labels.extend(accepted_execution_evidence_labels(report, family));
            let hostile_evidence_labels = hostile_evidence_labels(report, family);
            let replay_evidence_labels = replay_evidence_labels(report, family);
            let mut rejection_evidence_labels = rejected_execution_evidence_labels(report, family);
            rejection_evidence_labels.push(format!(
                "snapshot_read_only_rejection={:?}",
                snapshot_row.status()
            ));
            let admitted_lane_labels = current_head_row
                .admitted_lanes()
                .iter()
                .map(|lane| lane.as_str().to_string())
                .collect::<Vec<_>>();
            let primitive_family_evidence_count = primitive_family_evidence_count(report, family);
            let scale_pressure_evidence_count = scale_pressure_evidence_count(report, family);
            let branch_local_evidence_count = branch_local_evidence_count(report, family);
            let localized_rejection_evidence_count =
                localized_rejection_evidence_count(report, family);
            let derived_breadth_evidence_count = derived_breadth_evidence_count(report, family);
            let legal_execution_count = accepted_execution_evidence_count(report, family)
                + primitive_family_evidence_count
                + scale_pressure_evidence_count;
            let hostile_workload_count = direct_scenarios.len() + scale_pressure_evidence_count;
            let replay_evidence_count = replay_evidence_labels.len();
            let rejection_evidence_count = rejection_evidence_labels.len();
            MilestoneThreeOperatorFamilyClosureRow {
                family,
                admitted_lane_labels,
                row_digest: row_digest(
                    family,
                    &legal_evidence_labels,
                    &hostile_evidence_labels,
                    &replay_evidence_labels,
                    &rejection_evidence_labels,
                    legal_execution_count,
                    hostile_workload_count,
                    replay_evidence_count,
                    rejection_evidence_count,
                    localized_rejection_evidence_count,
                    branch_local_evidence_count,
                    primitive_family_evidence_count,
                    scale_pressure_evidence_count,
                    derived_breadth_evidence_count,
                ),
                legal_evidence_labels,
                hostile_evidence_labels,
                replay_evidence_labels,
                rejection_evidence_labels,
                direct_hostile_scenario_labels: direct_scenarios,
                legal_execution_count,
                hostile_workload_count,
                replay_evidence_count,
                rejection_evidence_count,
                localized_rejection_evidence_count,
                branch_local_evidence_count,
                primitive_family_evidence_count,
                scale_pressure_evidence_count,
                derived_breadth_evidence_count,
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_operator_family_closure_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for family in TopologyMutationFamily::ALL {
        let row = report
            .operator_family_closure_rows
            .iter()
            .find(|row| row.family == family)
            .ok_or_else(|| operator_family_closure_error("missing operator-family row"))?;
        if row.admitted_lane_labels.is_empty()
            || row.legal_evidence_labels.is_empty()
            || row.hostile_evidence_labels.is_empty()
            || row.replay_evidence_labels.is_empty()
            || row.rejection_evidence_labels.is_empty()
            || row.legal_execution_count == 0
            || row.hostile_workload_count == 0
            || row.replay_evidence_count == 0
            || row.rejection_evidence_count == 0
            || row.derived_breadth_evidence_count == 0
        {
            return Err(operator_family_closure_error(&format!(
                "operator family closure row is incomplete for {family:?}"
            )));
        }
        if row.rejection_evidence_labels.iter().any(|label| {
            label.starts_with("rejected_scenario=") || label.starts_with("branch_local_rejection=")
        }) && row.localized_rejection_evidence_count == 0
        {
            return Err(operator_family_closure_error(&format!(
                "operator family closure row lacks localized rejection evidence for {family:?}"
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn operator_family_closure_labels(
    report: &MilestoneThreeHostileSuiteReport,
) -> (usize, Vec<String>, Vec<String>) {
    let mut evidence_labels = report
        .operator_family_closure_rows
        .iter()
        .map(|row| format!("operator_family_closure={:?}", row.family))
        .collect::<Vec<_>>();
    evidence_labels.sort();
    let gap_labels = TopologyMutationFamily::ALL
        .iter()
        .filter(|family| {
            !report
                .operator_family_closure_rows
                .iter()
                .any(|row| row.family == **family)
        })
        .map(|family| format!("missing_operator_family_closure={family:?}"))
        .collect::<Vec<_>>();
    (evidence_labels.len(), evidence_labels, gap_labels)
}

fn direct_scenario_labels(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> Vec<String> {
    report
        .scenario_reports
        .iter()
        .filter(|scenario| scenario.mutation_families().contains(&family))
        .map(|scenario| scenario.scenario.as_str().to_string())
        .collect()
}

fn hostile_evidence_labels(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> Vec<String> {
    let mut labels = direct_scenario_labels(report, family)
        .into_iter()
        .map(|scenario| format!("hostile_scenario={scenario}"))
        .collect::<Vec<_>>();
    labels.push("snapshot_read_only_denial_surface=hostile_posture".to_string());
    labels
}

fn replay_evidence_labels(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> Vec<String> {
    let mut labels = report
        .scenario_reports
        .iter()
        .filter(|scenario| {
            scenario.mutation_families().contains(&family)
                && scenario.mutation_replay_parity_report.replay_checked
        })
        .map(|scenario| format!("scenario_replay={}", scenario.scenario.as_str()))
        .collect::<Vec<_>>();
    labels.extend(
        report
            .primitive_family_closure_rows
            .iter()
            .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
            .map(|row| format!("primitive_family_replay={}", row.primitive_family)),
    );
    labels.extend(
        report
            .scale_pressure_rows
            .iter()
            .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
            .map(|row| format!("scale_pressure_replay={}", row.sweep.as_str())),
    );
    if report.mutation_branch_local_parity_rows.iter().any(|row| {
        row.mutation_families.contains(&family)
            && row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
    }) {
        labels.push("branch_local_replay=accepted_history".to_string());
    }
    labels
}

fn accepted_execution_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    report
        .scenario_reports
        .iter()
        .filter(|scenario| {
            scenario.mutation_families().contains(&family)
                && scenario.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
        })
        .count()
}

fn accepted_execution_evidence_labels(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> Vec<String> {
    let mut labels = report
        .scenario_reports
        .iter()
        .filter(|scenario| {
            scenario.mutation_families().contains(&family)
                && scenario.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
        })
        .map(|scenario| format!("accepted_scenario={}", scenario.scenario.as_str()))
        .collect::<Vec<_>>();
    labels.extend(
        report
            .primitive_family_closure_rows
            .iter()
            .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
            .map(|row| format!("accepted_primitive_family={}", row.primitive_family)),
    );
    labels.extend(
        report
            .scale_pressure_rows
            .iter()
            .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
            .map(|row| format!("accepted_scale_pressure={}", row.sweep.as_str())),
    );
    labels
}

fn primitive_family_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    report
        .primitive_family_closure_rows
        .iter()
        .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
        .count()
}

fn scale_pressure_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    report
        .scale_pressure_rows
        .iter()
        .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
        .count()
}

fn branch_local_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    report
        .mutation_branch_local_parity_rows
        .iter()
        .filter(|row| row.mutation_families.contains(&family))
        .count()
}

fn localized_rejection_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    report
        .failure_locality_rows
        .iter()
        .filter(|row| row.families.contains(&family) && row.scope_row_count > 0)
        .count()
}

fn derived_breadth_evidence_count(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> usize {
    let scenario_count = report
        .scenario_reports
        .iter()
        .filter(|scenario| scenario.mutation_families().contains(&family))
        .filter(|scenario| {
            report
                .mutation_fallout_breadth_rows
                .iter()
                .any(|row| row.scenario == scenario.scenario)
        })
        .count();
    scenario_count
        + report
            .primitive_family_closure_rows
            .iter()
            .filter(|row| {
                row.mutation_families.contains(&family) && row.derived_validation_row_count > 0
            })
            .count()
        + report
            .scale_pressure_rows
            .iter()
            .filter(|row| row.mutation_families.contains(&family) && row.replay_verified)
            .count()
}

fn rejected_execution_evidence_labels(
    report: &MilestoneThreeHostileSuiteReport,
    family: TopologyMutationFamily,
) -> Vec<String> {
    let mut labels = report
        .scenario_reports
        .iter()
        .filter(|scenario| {
            scenario.mutation_families().contains(&family)
                && scenario.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
        })
        .map(|scenario| format!("rejected_scenario={}", scenario.scenario.as_str()))
        .collect::<Vec<_>>();
    labels.extend(
        report
            .mutation_branch_local_parity_rows
            .iter()
            .filter(|row| {
                row.mutation_families.contains(&family)
                    && row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
            })
            .map(|row| format!("branch_local_rejection={}", row.branch_label)),
    );
    labels
}

fn row_digest(
    family: TopologyMutationFamily,
    legal_evidence_labels: &[String],
    hostile_evidence_labels: &[String],
    replay_evidence_labels: &[String],
    rejection_evidence_labels: &[String],
    legal_execution_count: usize,
    hostile_workload_count: usize,
    replay_evidence_count: usize,
    rejection_evidence_count: usize,
    localized_rejection_evidence_count: usize,
    branch_local_evidence_count: usize,
    primitive_family_evidence_count: usize,
    scale_pressure_evidence_count: usize,
    derived_breadth_evidence_count: usize,
) -> String {
    format!(
        "operator_family={family:?};legal={};hostile={};replay={};rejection={};legal_executions={legal_execution_count};hostile_workloads={hostile_workload_count};replay_evidence={replay_evidence_count};rejection_evidence={rejection_evidence_count};localized_rejections={localized_rejection_evidence_count};branch_local={branch_local_evidence_count};primitive_family={primitive_family_evidence_count};scale_pressure={scale_pressure_evidence_count};derived_breadth={derived_breadth_evidence_count}",
        legal_evidence_labels.len(),
        hostile_evidence_labels.len(),
        replay_evidence_labels.len(),
        rejection_evidence_labels.len()
    )
}

fn operator_family_closure_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three operator-family closure failed: {reason}"
    ))
}
