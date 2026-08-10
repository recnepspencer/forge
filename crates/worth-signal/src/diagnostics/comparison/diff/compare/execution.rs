use crate::diagnostics::summary::ExecutionReportSummary;

use super::super::model::{
    compare_value, push_mismatch, DiagnosticMismatchCategory, ExecutionReportDiff,
};

pub fn compare_execution_reports(
    left: &ExecutionReportSummary,
    right: &ExecutionReportSummary,
) -> ExecutionReportDiff {
    let mut diff = ExecutionReportDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "stage_count",
        left.stage_count,
        right.stage_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "task_count",
        left.task_count,
        right.task_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_executed",
        left.tasks_executed,
        right.tasks_executed,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_validated_clean",
        left.tasks_validated_clean,
        right.tasks_validated_clean,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_satisfied_by_memoization",
        left.tasks_satisfied_by_memoization,
        right.tasks_satisfied_by_memoization,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_with_suppressed_propagation",
        left.tasks_with_suppressed_propagation,
        right.tasks_with_suppressed_propagation,
    );
    if left.task_outcome_counts != right.task_outcome_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::TaskOutcome,
            "task_outcome_counts",
            format!("{:?}", left.task_outcome_counts),
            format!("{:?}", right.task_outcome_counts),
        );
    }
    if left.stage_outcome_counts != right.stage_outcome_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "stage_outcome_counts",
            format!("{:?}", left.stage_outcome_counts),
            format!("{:?}", right.stage_outcome_counts),
        );
    }
    diff
}
