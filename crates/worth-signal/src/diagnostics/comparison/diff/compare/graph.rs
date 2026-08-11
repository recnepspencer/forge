use crate::diagnostics::summary::GraphSummary;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, GraphDiff};

pub fn compare_graphs(left: &GraphSummary, right: &GraphSummary) -> GraphDiff {
    let mut diff = GraphDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "active_node_count",
        left.active_node_count,
        right.active_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "clean_node_count",
        left.clean_node_count,
        right.clean_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "maybe_stale_node_count",
        left.maybe_stale_node_count,
        right.maybe_stale_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "dirty_node_count",
        left.dirty_node_count,
        right.dirty_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphStructure,
        "dependency_edge_count",
        left.dependency_edge_count,
        right.dependency_edge_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphStructure,
        "nodes_with_partition_scopes",
        left.nodes_with_partition_scopes,
        right.nodes_with_partition_scopes,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "nodes_with_execution_record",
        left.nodes_with_execution_record,
        right.nodes_with_execution_record,
    );
    if left.sample_dirty_nodes != right.sample_dirty_nodes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::GraphState,
            "sample_dirty_nodes",
            format!("{:?}", left.sample_dirty_nodes),
            format!("{:?}", right.sample_dirty_nodes),
        );
    }
    if left.metrics != right.metrics {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Metrics,
            "metrics",
            format!("{:?}", left.metrics),
            format!("{:?}", right.metrics),
        );
    }
    diff
}
