mod compare;
mod model;

pub use compare::{
    compare_execution_history, compare_execution_reports, compare_explanations, compare_failures,
    compare_flows, compare_graphs, compare_lineage_records, compare_plans, compare_replay_slices,
};
pub use model::{
    DiagnosticMismatch, DiagnosticMismatchCategory, ExecutionReportDiff, ExplanationDiff,
    FailureDiff, FlowDiff, GraphDiff, HistoryDiff, LineageDiff, PlanDiff, ReplayDiff,
};
