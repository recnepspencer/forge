#[path = "diff/model.rs"]
mod model;
#[path = "diff/compare.rs"]
mod compare;

pub use compare::{
    compare_execution_history, compare_execution_reports, compare_explanations, compare_failures,
    compare_flows, compare_graphs, compare_plans,
};
pub use model::{
    DiagnosticMismatch, DiagnosticMismatchCategory, ExecutionReportDiff, ExplanationDiff,
    FailureDiff, FlowDiff, GraphDiff, HistoryDiff, PlanDiff,
};
