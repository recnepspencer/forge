use crate::data::aspect::Aspect;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::diagnostics::flow::FlowSummary;
use crate::logic::planner::{EvaluationPlan, ExecutionReport, TaskExecutionOutcome, TaskReason};

pub fn inspect_graph(graph: &SignalGraph) -> GraphInspector<'_> {
    GraphInspector { graph }
}

pub fn inspect_plan(plan: &EvaluationPlan) -> PlanInspector<'_> {
    PlanInspector { plan }
}

pub fn inspect_report(report: &ExecutionReport) -> ReportInspector<'_> {
    ReportInspector { report }
}

pub fn inspect_execution(graph: &SignalGraph) -> ExecutionInspector<'_> {
    ExecutionInspector { graph }
}

pub fn inspect_flow(flow: &FlowSummary) -> FlowInspector<'_> {
    FlowInspector { flow }
}

pub struct GraphInspector<'a> {
    pub(crate) graph: &'a SignalGraph,
}

impl<'a> GraphInspector<'a> {
    pub fn nodes_in_state(&self, state: NodeState) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| self.graph.get_state(*node).ok() == Some(state))
            .collect()
    }

    pub fn nodes_with_dirty_aspect(&self, aspect: Aspect) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| entry.get_dirty_aspects().contains(aspect.into()))
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn nodes_with_partition_scopes(&self) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| entry.get_dependencies().iter().any(|edge| edge.scope_ref().is_some()))
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn nodes_with_condition(&self, condition: &EvaluationCondition) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| entry.get_eval_config().condition == *condition)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn nodes_with_execution_record(&self) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .ok()
                    .and_then(|entry| entry.get_trace_summary())
                    .and_then(|trace| trace.execution_record_id)
                    .is_some()
            })
            .collect()
    }

    pub fn nodes_with_causality(&self) -> Vec<NodeId> {
        self.live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| entry.get_causality().is_some())
                    .unwrap_or(false)
            })
            .collect()
    }

    fn live_nodes(&self) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            if let Some(node) = self.graph.live_node_id_at(index) {
                nodes.push(node);
            }
        }
        nodes
    }
}

pub struct PlanInspector<'a> {
    pub(crate) plan: &'a EvaluationPlan,
}

impl<'a> PlanInspector<'a> {
    pub fn stage_count(&self) -> usize {
        self.plan.stages.len()
    }

    pub fn tasks_for_node(&self, node: NodeId) -> Vec<&'a crate::logic::planner::EvaluationTask> {
        self.plan
            .stages
            .iter()
            .flat_map(|stage| stage.tasks.iter())
            .filter(|task| task.node == node)
            .collect()
    }

    pub fn tasks_by_reason(&self, reason: TaskReason) -> Vec<&'a crate::logic::planner::EvaluationTask> {
        self.plan
            .stages
            .iter()
            .flat_map(|stage| stage.tasks.iter())
            .filter(|task| task.reason == reason)
            .collect()
    }

    pub fn direct_tasks(&self) -> Vec<&'a crate::logic::planner::EvaluationTask> {
        self.plan
            .stages
            .iter()
            .flat_map(|stage| stage.tasks.iter())
            .filter(|task| task.direct_request)
            .collect()
    }
}

pub struct ReportInspector<'a> {
    pub(crate) report: &'a ExecutionReport,
}

impl<'a> ReportInspector<'a> {
    pub fn tasks_with_outcome(
        &self,
        outcome: TaskExecutionOutcome,
    ) -> Vec<&'a crate::logic::planner::TaskExecutionRecord> {
        self.report
            .stages
            .iter()
            .flat_map(|stage| stage.task_records.iter())
            .filter(|task| task.outcome == outcome)
            .collect()
    }

    pub fn task_record_for_node(
        &self,
        node: NodeId,
    ) -> Option<&'a crate::logic::planner::TaskExecutionRecord> {
        self.report
            .stages
            .iter()
            .flat_map(|stage| stage.task_records.iter())
            .find(|task| task.node == node)
    }
}

pub struct ExecutionInspector<'a> {
    pub(crate) graph: &'a SignalGraph,
}

impl<'a> ExecutionInspector<'a> {
    pub fn nodes_with_trace_summaries(&self) -> Vec<NodeId> {
        inspect_graph(self.graph)
            .live_nodes()
            .into_iter()
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| entry.get_trace_summary().is_some())
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn latest_execution_record_id(&self) -> Option<u64> {
        self.nodes_with_trace_summaries()
            .into_iter()
            .filter_map(|node| {
                self.graph
                    .get_entry(node)
                    .ok()
                    .and_then(|entry| entry.get_trace_summary())
                    .and_then(|trace| trace.execution_record_id)
            })
            .max()
    }
}

pub struct FlowInspector<'a> {
    pub(crate) flow: &'a FlowSummary,
}

impl<'a> FlowInspector<'a> {
    pub fn changed_nodes(&self) -> &[NodeId] {
        &self.flow.change.changed_nodes
    }

    pub fn execution_tasks(&self) -> u32 {
        self.flow.apply.report.task_count
    }
}
