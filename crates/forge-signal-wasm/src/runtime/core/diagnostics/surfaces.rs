use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::summaries::{
    ExecutionHistorySurfaceSummary, FlowSurfaceSummary, HealthSummary, LineageEventSummary,
    LineageSummary, ObservationSurfaceSummary, ReplayFrameSummary, ReplaySummary,
};

use super::super::evaluation::signal_value_breadth;
use super::super::RuntimeCore;

impl RuntimeCore {
    pub fn health(&self) -> Result<HealthSummary, ForgeSignalJsError> {
        Ok(self
            .runtime
            .diagnostics()
            .health_view()
            .summary_now()
            .into())
    }

    pub fn diagnostics_summary_now(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::GraphSummary, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().summary_now())
    }

    pub fn execution_history_now(
        &self,
    ) -> Result<ExecutionHistorySurfaceSummary, ForgeSignalJsError> {
        let history = self.runtime.diagnostics().history_now();
        let callback_nodes =
            self.callback_nodes_for_node_ids(history.nodes.iter().map(|node| node.node))?;
        Ok(ExecutionHistorySurfaceSummary {
            history,
            callback_nodes,
        })
    }

    pub fn latest_flow(&self) -> Result<Option<FlowSurfaceSummary>, ForgeSignalJsError> {
        self.runtime
            .diagnostics()
            .latest_flow()
            .cloned()
            .map(|flow| {
                self.callback_nodes_for_node_ids(
                    flow.change
                        .changed_nodes
                        .iter()
                        .copied()
                        .chain(flow.cause_samples.iter().map(|sample| sample.node))
                        .chain(flow.explanation.iter().map(|explanation| explanation.node))
                        .chain(
                            flow.observation
                                .iter()
                                .flat_map(|summary| summary.boundary_events.iter())
                                .flat_map(|event| {
                                    event
                                        .observed_nodes
                                        .iter()
                                        .chain(event.matched_nodes.iter())
                                }),
                        )
                        .chain(
                            self.runtime
                                .diagnostics()
                                .history_now()
                                .nodes
                                .iter()
                                .map(|node| node.node),
                        ),
                )
                .map(|callback_nodes| FlowSurfaceSummary {
                    flow,
                    callback_nodes,
                })
            })
            .transpose()
    }

    pub fn latest_observation(
        &self,
    ) -> Result<Option<ObservationSurfaceSummary>, ForgeSignalJsError> {
        self.runtime
            .diagnostics()
            .latest_observation()
            .cloned()
            .map(|observation| {
                self.callback_nodes_for_node_ids(
                    observation
                        .boundary_events
                        .iter()
                        .flat_map(|event| {
                            event
                                .observed_nodes
                                .iter()
                                .chain(event.matched_nodes.iter())
                        })
                        .chain(
                            self.runtime
                                .diagnostics()
                                .latest_flow()
                                .into_iter()
                                .flat_map(|flow| {
                                    flow.change
                                        .changed_nodes
                                        .iter()
                                        .copied()
                                        .chain(flow.cause_samples.iter().map(|sample| sample.node))
                                        .chain(
                                            flow.explanation
                                                .iter()
                                                .map(|explanation| explanation.node),
                                        )
                                }),
                        )
                        .chain(
                            self.runtime
                                .diagnostics()
                                .history_now()
                                .nodes
                                .iter()
                                .map(|node| node.node),
                        ),
                )
                .map(|callback_nodes| ObservationSurfaceSummary {
                    observation,
                    callback_nodes,
                })
            })
            .transpose()
    }

    pub(crate) fn record_output_serialization(
        &mut self,
        value: &crate::expression::model::SignalValue,
    ) {
        self.web_metrics.output_serialization_count = self
            .web_metrics
            .output_serialization_count
            .saturating_add(1);
        self.web_metrics.output_serialization_breadth = self
            .web_metrics
            .output_serialization_breadth
            .saturating_add(signal_value_breadth(value));
    }

    pub fn latest_failure(
        &self,
    ) -> Result<Option<forge_signal::diagnostics::FailureSummary>, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().latest_failure().cloned())
    }

    pub fn latest_rollback(
        &self,
    ) -> Result<Option<forge_signal::diagnostics::RollbackDiagnostic>, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().latest_rollback().cloned())
    }

    pub fn latest_frontier_execution(
        &self,
    ) -> Result<Option<forge_signal::facade::adapters::FrontierExecutionSummary>, ForgeSignalJsError>
    {
        Ok(self
            .runtime
            .diagnostics()
            .latest_frontier_execution()
            .cloned())
    }

    pub fn latest_invalidation_trace_records(
        &self,
    ) -> Result<Vec<forge_signal::facade::adapters::InvalidationTraceRecord>, ForgeSignalJsError>
    {
        Ok(self
            .runtime
            .diagnostics()
            .latest_invalidation_trace_records()
            .to_vec())
    }

    pub fn recent_history(
        &self,
    ) -> Result<Vec<forge_signal::facade::diagnostics::ExecutionHistorySummary>, ForgeSignalJsError>
    {
        Ok(self
            .runtime
            .diagnostics()
            .recent_history()
            .iter()
            .cloned()
            .collect())
    }

    pub fn replay_for_id(&mut self, id: &str) -> Result<ReplaySummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let replay = {
            let history = self.runtime.history();
            history.replay_for_node(node)
        };
        let mut frames = Vec::with_capacity(replay.frames.len());
        for frame in replay.frames {
            let callback = match frame.node {
                Some(node) => self.callback_node_for_node(node)?,
                None => None,
            };
            frames.push(ReplayFrameSummary {
                cursor: frame.cursor.0,
                kind: format!("{:?}", frame.kind),
                branch_id: frame.branch_id.0,
                snapshot_id: frame.snapshot_id.map(|id| id.0),
                node: frame.node.map(|node| node.to_string()),
                detail: frame
                    .detail
                    .and_then(|detail| detail.as_message().map(str::to_owned))
                    .or_else(|| {
                        frame
                            .execution_record_id
                            .map(|id| format!("executionRecord:{id}"))
                    }),
                callback,
            });
        }
        Ok(ReplaySummary { frames })
    }

    pub fn lineage_for_id(&mut self, id: &str) -> Result<LineageSummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let chain = {
            let history = self.runtime.history();
            history.lineage_for_node(node)
        };
        let mut events = Vec::new();
        for record in chain.to_owned_records() {
            let node = record.node();
            let callback = match node {
                Some(node) => self.callback_node_for_node(node)?,
                None => None,
            };
            events.push(LineageEventSummary {
                sequence: record.sequence,
                label: record.label().to_owned(),
                emitted_on_branch_id: record.emitted_on_branch_id().0,
                node: node.map(|node| node.to_string()),
                subject_artifact_id: record.subject_artifact_id().map(|id| id.0),
                parent_artifact_id: record.parent_artifact_id().map(|id| id.0),
                snapshot_id: record.snapshot_id().map(|id| id.0),
                callback,
            });
        }
        Ok(LineageSummary { events })
    }

    pub fn graph_summary(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::GraphSummary, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().summary_now())
    }
}
