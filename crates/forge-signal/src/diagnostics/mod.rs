pub mod access;
pub mod compare;
pub mod diff;
pub mod display;
pub mod failure;
pub mod flow;
pub mod history;
pub mod policy;
pub mod profile;
pub(crate) mod recorder;
pub mod summary;
pub(crate) mod state;

pub use access::{diagnostics_for_graph, diagnostics_for_runtime, GraphDiagnostics, RuntimeDiagnostics};
pub use compare::{
    explanations_semantically_equivalent, graphs_semantically_equivalent,
    plans_semantically_equivalent, repeat_run_summaries_equal,
    reports_semantically_equivalent, serial_parallel_reports_equivalent,
};
pub use diff::{
    compare_execution_history, compare_execution_reports, compare_explanations, compare_failures,
    compare_flows, compare_graphs, compare_plans, DiagnosticMismatch,
    DiagnosticMismatchCategory, ExecutionReportDiff, ExplanationDiff, FailureDiff, FlowDiff,
    GraphDiff, HistoryDiff, PlanDiff,
};
pub use display::{
    render_execution_history_summary, render_execution_report_summary, render_explanation_summary,
    render_failure_summary, render_flow_summary, render_graph_summary, render_plan_summary,
};
pub use failure::{
    ExecutionFailureContext, ExecutionFailurePhase, FailureSummary, RollbackDiagnostic,
};
pub use flow::{
    ChangeInputSummary, FlowSummary, InvalidationSummary, PlanningSummary, PrecomputeSummary,
    ApplySummary, RollbackSummary,
};
pub use history::{
    inspect_execution, inspect_flow, inspect_graph, inspect_plan, inspect_report, ExecutionInspector,
    FlowInspector, GraphInspector, PlanInspector, ReportInspector,
};
pub use policy::DiagnosticsPolicy;
pub use profile::DiagnosticsProfile;
pub use summary::{
    EvaluationPlanSummary, ExecutionHistoryNodeSummary, ExecutionHistorySummary,
    ExecutionReportSummary, ExplanationSummary, GraphSummary,
};
