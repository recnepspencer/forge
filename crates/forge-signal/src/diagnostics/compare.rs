use crate::diagnostics::diff::{
    compare_execution_reports, compare_explanations, compare_graphs, compare_lineage_records,
    compare_plans, compare_replay_slices, ExecutionReportDiff,
};
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::replay::ReplaySlice;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionReportSummary, ExplanationSummary, GraphSummary,
};

pub fn graphs_semantically_equivalent(left: &GraphSummary, right: &GraphSummary) -> bool {
    compare_graphs(left, right).is_empty()
}

pub fn plans_semantically_equivalent(
    left: &EvaluationPlanSummary,
    right: &EvaluationPlanSummary,
) -> bool {
    compare_plans(left, right).is_empty()
}

pub fn reports_semantically_equivalent(
    left: &ExecutionReportSummary,
    right: &ExecutionReportSummary,
) -> bool {
    compare_execution_reports(left, right).is_empty()
}

pub fn explanations_semantically_equivalent(
    left: &ExplanationSummary,
    right: &ExplanationSummary,
) -> bool {
    compare_explanations(left, right).is_empty()
}

pub fn serial_parallel_reports_equivalent(
    left: &ExecutionReportSummary,
    right: &ExecutionReportSummary,
) -> bool {
    let mut diff: ExecutionReportDiff = compare_execution_reports(left, right);
    diff.mismatches
        .retain(|mismatch| mismatch.field != "stage_outcome_counts");
    diff.is_empty()
}

pub fn repeat_run_summaries_equal<T: PartialEq>(runs: &[T]) -> bool {
    runs.windows(2).all(|pair| pair[0] == pair[1])
}

pub fn replay_slices_equivalent(left: &ReplaySlice, right: &ReplaySlice) -> bool {
    compare_replay_slices(left, right).is_empty()
}

pub fn lineage_records_equivalent(left: &[LineageRecord], right: &[LineageRecord]) -> bool {
    compare_lineage_records(left, right).is_empty()
}
