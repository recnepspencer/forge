use crate::diagnostics::summary::ExecutionHistorySummary;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, HistoryDiff};

pub fn compare_execution_history(
    left: &ExecutionHistorySummary,
    right: &ExecutionHistorySummary,
) -> HistoryDiff {
    let mut diff = HistoryDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::History,
        "traced_node_count",
        left.traced_node_count,
        right.traced_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "execution_record_count",
        left.execution_record_count,
        right.execution_record_count,
    );
    if left.latest_execution_record_id != right.latest_execution_record_id {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "latest_execution_record_id",
            format!("{:?}", left.latest_execution_record_id),
            format!("{:?}", right.latest_execution_record_id),
        );
    }
    if left.nodes != right.nodes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::History,
            "nodes",
            format!("{:?}", left.nodes),
            format!("{:?}", right.nodes),
        );
    }
    diff
}
