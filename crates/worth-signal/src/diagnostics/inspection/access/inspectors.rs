use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::history::{
    inspect_execution, inspect_flow, inspect_graph, inspect_plan, inspect_report,
    ExecutionInspector, FlowInspector, GraphInspector, PlanInspector, ReportInspector,
};
use crate::logic::planner::{EvaluationPlan, ExecutionReport};

use super::GraphInspectDiagnostics;

impl<'a> GraphInspectDiagnostics<'a> {
    pub fn graph(&self) -> GraphInspector<'a> {
        inspect_graph(self.graph)
    }

    pub fn execution(&self) -> ExecutionInspector<'a> {
        inspect_execution(self.graph)
    }

    pub fn plan(&self, plan: &'a EvaluationPlan) -> PlanInspector<'a> {
        inspect_plan(plan)
    }

    pub fn report(&self, report: &'a ExecutionReport) -> ReportInspector<'a> {
        inspect_report(report)
    }

    pub fn flow(&self, flow: &'a FlowSummary) -> FlowInspector<'a> {
        inspect_flow(flow)
    }
}
