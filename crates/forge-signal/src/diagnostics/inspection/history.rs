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
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self.graph.get_state(node).ok() == Some(state) {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn nodes_with_dirty_aspect(&self, aspect: Aspect) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .node_dirty_aspects(node)
                .map(|dirty| dirty.contains(aspect.into()))
                .unwrap_or(false)
            {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn nodes_with_partition_scopes(&self) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .dependencies_of(node)
                .map(|dependencies| dependencies.iter().any(|edge| edge.scope_ref().is_some()))
                .unwrap_or(false)
            {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn nodes_with_condition(&self, condition: &EvaluationCondition) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .node_condition(node)
                .map(|stored| stored == *condition)
                .unwrap_or(false)
            {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn nodes_with_execution_record(&self) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .node_execution_trace_stamp(node)
                .ok()
                .flatten()
                .and_then(|stamp| stamp.execution_record_id)
                .is_some()
            {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn nodes_with_causality(&self) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .causality_of(node)
                .map(|causality| causality.is_some())
                .unwrap_or(false)
            {
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

    pub fn tasks_for_node(&self, node: NodeId) -> Vec<&'a crate::logic::planner::EligibleTask> {
        self.plan
            .stages
            .iter()
            .flat_map(|stage| stage.tasks.iter())
            .filter(|task| task.node == node)
            .collect()
    }

    pub fn tasks_by_reason(
        &self,
        reason: TaskReason,
    ) -> Vec<&'a crate::logic::planner::EligibleTask> {
        self.plan
            .stages
            .iter()
            .flat_map(|stage| stage.tasks.iter())
            .filter(|task| task.reason == reason)
            .collect()
    }

    pub fn direct_tasks(&self) -> Vec<&'a crate::logic::planner::EligibleTask> {
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
        let mut nodes = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            if self
                .graph
                .node_runtime_artifact_state_present(node)
                .unwrap_or(false)
            {
                nodes.push(node);
            }
        }
        nodes
    }

    pub fn latest_execution_record_id(&self) -> Option<u64> {
        let mut latest = None;
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            let current = self
                .graph
                .node_execution_trace_stamp(node)
                .ok()
                .flatten()
                .and_then(|stamp| stamp.execution_record_id);
            if let Some(current) = current {
                latest = Some(latest.map_or(current, |seen: u64| seen.max(current)));
            }
        }
        latest
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
