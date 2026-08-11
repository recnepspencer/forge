use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::{ChangeInputSummary, FlowSummary, InvalidationSummary};
use crate::diagnostics::policy::{FrontierTracingPolicy, SignalRuntimePolicy};
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::transaction::ObservationBoundarySummary;

use super::{DiagnosticsState, PendingFlowInput};

impl DiagnosticsState {
    pub fn profile(&self) -> DiagnosticsTier {
        self.policy.tier
    }

    pub fn tier(&self) -> DiagnosticsTier {
        self.profile()
    }

    pub fn set_profile(&mut self, profile: DiagnosticsTier) {
        self.policy = SignalRuntimePolicy::for_tier(profile);
        self.trim_history();
    }

    pub fn policy(&self) -> SignalRuntimePolicy {
        self.policy
    }

    pub fn set_policy(&mut self, policy: SignalRuntimePolicy) {
        self.policy = policy;
        if !self.policy.retains_explanation_facts() {
            self.explanation_facts.clear();
        }
        if !self.policy.retains_provenance_facts() {
            self.provenance_facts.clear();
        }
        if matches!(
            self.policy.frontier_tracing_policy,
            FrontierTracingPolicy::SummaryOnly
        ) {
            self.latest_invalidation_trace_records.clear();
        }
        self.trim_history();
    }

    pub fn latest_flow(&self) -> Option<&FlowSummary> {
        self.latest_flow.as_ref()
    }

    pub fn latest_failure(&self) -> Option<&FailureSummary> {
        self.latest_failure.as_ref()
    }

    pub fn latest_rollback(&self) -> Option<&RollbackDiagnostic> {
        self.latest_rollback.as_ref()
    }

    pub fn latest_observation(&self) -> Option<&ObservationBoundarySummary> {
        self.latest_observation.as_ref()
    }

    pub fn latest_graph_summary(&self) -> Option<&GraphSummary> {
        self.latest_graph_summary.as_ref()
    }

    pub fn pending_graph_summary(&self) -> Option<&GraphSummary> {
        self.pending_graph_summary.as_ref()
    }

    pub fn latest_frontier_execution(&self) -> Option<&FrontierExecutionSummary> {
        self.latest_frontier_execution.as_ref()
    }

    pub fn latest_invalidation_trace_records(&self) -> &[InvalidationTraceRecord] {
        &self.latest_invalidation_trace_records
    }

    pub fn recent_history(&self) -> &std::collections::VecDeque<ExecutionHistorySummary> {
        &self.recent_history
    }

    pub fn explanation_facts(&self) -> &std::collections::BTreeMap<NodeId, ExplanationFact> {
        &self.explanation_facts
    }

    pub fn provenance_facts(&self) -> &std::collections::BTreeMap<NodeId, ProvenanceFact> {
        &self.provenance_facts
    }

    pub fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[ChangedRegion],
        causality_kind: Option<String>,
    ) {
        let pending = self.pending_input.get_or_insert_with(|| PendingFlowInput {
            changed_nodes: Default::default(),
            changed_aspects: Default::default(),
            changed_region_count: 0,
            causality_kind: None,
        });
        pending.changed_nodes.insert(node);
        pending.changed_aspects.insert(aspect.id());
        pending.changed_region_count += changed_regions.len() as u32;
        if pending.causality_kind.is_none() {
            pending.causality_kind = causality_kind;
        }
    }

    pub fn record_frontier_execution(
        &mut self,
        summary: FrontierExecutionSummary,
        trace_records: Vec<InvalidationTraceRecord>,
    ) {
        self.latest_frontier_execution = Some(summary);
        self.latest_invalidation_trace_records = trace_records;
    }

    pub fn set_pending_graph_summary(&mut self, summary: GraphSummary) {
        self.pending_graph_summary = Some(summary);
    }

    pub fn complete_flow_without_graph_summary(
        &mut self,
        flow: FlowSummary,
        history: ExecutionHistorySummary,
    ) {
        self.latest_flow = Some(flow);
        self.latest_graph_summary = None;
        self.recent_history.push_back(history);
        self.trim_history();
        self.pending_input = None;
        self.pending_graph_summary = None;
    }

    pub fn refresh_retained_views(
        &mut self,
        history: ExecutionHistorySummary,
        graph_summary: GraphSummary,
    ) {
        self.latest_graph_summary = Some(graph_summary);
        self.recent_history.push_back(history);
        self.trim_history();
        self.pending_graph_summary = None;
    }

    pub fn record_failure(&mut self, failure: FailureSummary) {
        self.latest_failure = Some(failure);
    }

    pub fn record_rollback(&mut self, rollback: RollbackDiagnostic) {
        self.latest_rollback = Some(rollback);
    }

    pub fn record_observation(&mut self, observation: ObservationBoundarySummary) {
        self.latest_observation = Some(observation.clone());
        if let Some(flow) = &mut self.latest_flow {
            flow.observation = Some(observation);
        }
    }

    pub fn clear_pending_input(&mut self) {
        self.pending_input = None;
        self.pending_graph_summary = None;
        self.latest_frontier_execution = None;
        self.latest_invalidation_trace_records.clear();
    }

    pub fn attach_event_epochs_to_latest_flow(&mut self, event_epochs: Vec<EventEpochSummary>) {
        if let Some(flow) = &mut self.latest_flow {
            flow.event_epochs = event_epochs;
        }
    }

    pub fn record_explanation_fact(&mut self, fact: ExplanationFact) {
        if self.policy.retains_explanation_facts() {
            self.explanation_facts.insert(fact.node, fact);
        }
    }

    pub fn record_provenance_fact(&mut self, fact: ProvenanceFact) {
        if self.policy.retains_provenance_facts() {
            self.provenance_facts.insert(fact.node, fact);
        }
    }

    pub fn pending_change_summary(&self) -> Option<(ChangeInputSummary, InvalidationSummary)> {
        self.pending_input.as_ref().map(|pending| {
            (
                ChangeInputSummary::new(
                    pending.changed_nodes.iter().copied().collect(),
                    pending
                        .changed_aspects
                        .iter()
                        .copied()
                        .map(Aspect::new)
                        .collect(),
                    pending.changed_region_count,
                    pending.causality_kind.clone(),
                ),
                self.latest_frontier_execution
                    .as_ref()
                    .map(InvalidationSummary::from_frontier_execution)
                    .unwrap_or_else(InvalidationSummary::empty_frontier),
            )
        })
    }

    pub fn has_pending_change_input(&self) -> bool {
        self.pending_input.is_some()
    }

    pub(super) fn trim_history(&mut self) {
        let limit = self.policy.retention_budget.history_limit;
        while self.recent_history.len() > limit {
            self.recent_history.pop_front();
        }
    }
}
