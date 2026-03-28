use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::{ChangeInputSummary, FlowSummary, InvalidationSummary};
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent};
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::state::{
    SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
    SignalSnapshotMeta, SnapshotArtifactRetentionPolicy,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct DiagnosticsState {
    #[serde(default)]
    policy: SignalRuntimePolicy,
    #[serde(default)]
    latest_flow: Option<FlowSummary>,
    #[serde(default)]
    latest_failure: Option<FailureSummary>,
    #[serde(default)]
    latest_rollback: Option<RollbackDiagnostic>,
    #[serde(default)]
    latest_graph_summary: Option<GraphSummary>,
    #[serde(default)]
    pending_graph_summary: Option<GraphSummary>,
    #[serde(default)]
    recent_history: VecDeque<ExecutionHistorySummary>,
    #[serde(default)]
    replay_events: VecDeque<ReplayEvent>,
    #[serde(default)]
    lineage_records: VecDeque<LineageRecord>,
    #[serde(skip)]
    replay_events_by_branch: BTreeMap<SignalBranchId, VecDeque<ReplayEvent>>,
    #[serde(skip)]
    replay_events_by_node: BTreeMap<NodeId, VecDeque<ReplayEvent>>,
    #[serde(skip)]
    replay_events_by_artifact: BTreeMap<LineageArtifactId, VecDeque<ReplayEvent>>,
    #[serde(skip)]
    replay_cursor_offsets: BTreeMap<ReplayCursor, usize>,
    #[serde(skip, default)]
    replay_cursor_offset_base: usize,
    #[serde(skip)]
    snapshot_replay_cursors: BTreeMap<SignalSnapshotId, ReplayCursor>,
    #[serde(skip)]
    lineage_records_by_artifact: BTreeMap<LineageArtifactId, VecDeque<LineageRecord>>,
    #[serde(skip)]
    lineage_records_by_node: BTreeMap<NodeId, VecDeque<LineageRecord>>,
    #[serde(default)]
    explanation_facts: BTreeMap<NodeId, ExplanationFact>,
    #[serde(default)]
    provenance_facts: BTreeMap<NodeId, ProvenanceFact>,
    #[serde(default)]
    branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
    #[serde(default)]
    active_branch: SignalBranchId,
    #[serde(default)]
    next_replay_cursor: u64,
    #[serde(default)]
    next_snapshot_id: u64,
    #[serde(default)]
    next_branch_id: u64,
    #[serde(default)]
    next_lineage_artifact_id: u64,
    #[serde(default)]
    next_lineage_sequence: u64,
    #[serde(default)]
    pending_input: Option<PendingFlowInput>,
    #[serde(default)]
    latest_frontier_execution: Option<FrontierExecutionSummary>,
    #[serde(default)]
    latest_invalidation_trace_records: Vec<InvalidationTraceRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingFlowInput {
    changed_nodes: BTreeSet<NodeId>,
    changed_aspects: BTreeSet<u8>,
    changed_region_count: u32,
    causality_kind: Option<String>,
}

impl DiagnosticsState {
    pub fn authority_carrier_clone(&self) -> Self {
        let mut state = Self {
            policy: self.policy,
            latest_flow: None,
            latest_failure: None,
            latest_rollback: None,
            latest_graph_summary: None,
            pending_graph_summary: None,
            recent_history: VecDeque::new(),
            replay_events: VecDeque::new(),
            lineage_records: VecDeque::new(),
            replay_events_by_branch: BTreeMap::new(),
            replay_events_by_node: BTreeMap::new(),
            replay_events_by_artifact: BTreeMap::new(),
            replay_cursor_offsets: BTreeMap::new(),
            replay_cursor_offset_base: 0,
            snapshot_replay_cursors: BTreeMap::new(),
            lineage_records_by_artifact: BTreeMap::new(),
            lineage_records_by_node: BTreeMap::new(),
            explanation_facts: BTreeMap::new(),
            provenance_facts: BTreeMap::new(),
            branch_catalog: self.branch_catalog.clone(),
            active_branch: self.active_branch,
            next_replay_cursor: self.next_replay_cursor,
            next_snapshot_id: self.next_snapshot_id,
            next_branch_id: self.next_branch_id,
            next_lineage_artifact_id: self.next_lineage_artifact_id,
            next_lineage_sequence: self.next_lineage_sequence,
            pending_input: None,
            latest_frontier_execution: None,
            latest_invalidation_trace_records: Vec::new(),
        };
        state.bootstrap_defaults();
        state
    }

    pub fn bootstrap_defaults(&mut self) {
        if self.branch_catalog.is_empty() {
            self.branch_catalog.insert(
                SignalBranchId(0),
                SignalBranchHandle {
                    id: SignalBranchId(0),
                    name: "main".to_string(),
                    parent_branch_id: None,
                    head_snapshot_id: None,
                },
            );
        }
    }

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
            crate::diagnostics::policy::FrontierTracingPolicy::SummaryOnly
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

    pub fn recent_history(&self) -> &VecDeque<ExecutionHistorySummary> {
        &self.recent_history
    }

    pub fn replay_events(&self) -> &VecDeque<ReplayEvent> {
        &self.replay_events
    }

    pub fn replay_events_for_branch(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&VecDeque<ReplayEvent>> {
        self.replay_events_by_branch.get(&branch_id)
    }

    pub fn replay_events_for_node(&self, node: NodeId) -> Option<&VecDeque<ReplayEvent>> {
        self.replay_events_by_node.get(&node)
    }

    pub fn replay_events_for_artifact(
        &self,
        artifact_id: LineageArtifactId,
    ) -> Option<&VecDeque<ReplayEvent>> {
        self.replay_events_by_artifact.get(&artifact_id)
    }

    pub fn replay_cursor_offset(&self, cursor: ReplayCursor) -> Option<usize> {
        self.replay_cursor_offsets
            .get(&cursor)
            .copied()
            .map(|absolute| absolute.saturating_sub(self.replay_cursor_offset_base))
    }

    pub fn snapshot_replay_cursor(&self, snapshot_id: SignalSnapshotId) -> Option<ReplayCursor> {
        self.snapshot_replay_cursors.get(&snapshot_id).copied()
    }

    pub fn lineage_records(&self) -> &VecDeque<LineageRecord> {
        &self.lineage_records
    }

    pub fn lineage_records_for_artifact(
        &self,
        artifact_id: LineageArtifactId,
    ) -> Option<&VecDeque<LineageRecord>> {
        self.lineage_records_by_artifact.get(&artifact_id)
    }

    pub fn lineage_records_for_node(&self, node: NodeId) -> Option<&VecDeque<LineageRecord>> {
        self.lineage_records_by_node.get(&node)
    }

    pub fn explanation_facts(&self) -> &BTreeMap<NodeId, ExplanationFact> {
        &self.explanation_facts
    }

    pub fn provenance_facts(&self) -> &BTreeMap<NodeId, ProvenanceFact> {
        &self.provenance_facts
    }

    pub fn branch_catalog(&self) -> &BTreeMap<SignalBranchId, SignalBranchHandle> {
        &self.branch_catalog
    }

    pub fn active_branch(&self) -> SignalBranchHandle {
        self.branch_catalog
            .get(&self.active_branch)
            .cloned()
            .unwrap_or_else(|| SignalBranchHandle {
                id: self.active_branch,
                name: "unknown".to_string(),
                parent_branch_id: None,
                head_snapshot_id: None,
            })
    }

    pub fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[ChangedRegion],
        causality_kind: Option<String>,
    ) {
        let pending = self.pending_input.get_or_insert_with(|| PendingFlowInput {
            changed_nodes: BTreeSet::new(),
            changed_aspects: BTreeSet::new(),
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

    #[allow(dead_code)]
    pub fn complete_flow(
        &mut self,
        flow: FlowSummary,
        history: ExecutionHistorySummary,
        graph_summary: GraphSummary,
    ) {
        self.latest_flow = Some(flow);
        self.latest_graph_summary = Some(graph_summary);
        self.recent_history.push_back(history);
        self.trim_history();
        self.pending_input = None;
        self.pending_graph_summary = None;
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

    pub fn allocate_replay_cursor(&mut self) -> ReplayCursor {
        let cursor = ReplayCursor(self.next_replay_cursor);
        self.next_replay_cursor += 1;
        cursor
    }

    pub fn latest_replay_cursor(&self) -> Option<ReplayCursor> {
        self.replay_events.back().map(|event| event.cursor)
    }

    pub fn record_replay_event(&mut self, event: ReplayEvent) {
        self.replay_events_by_branch
            .entry(event.branch_id)
            .or_default()
            .push_back(event.clone());
        if let Some(node) = event.node {
            self.replay_events_by_node
                .entry(node)
                .or_default()
                .push_back(event.clone());
        }
        if let Some(artifact_id) = event.lineage_artifact_id {
            self.replay_events_by_artifact
                .entry(artifact_id)
                .or_default()
                .push_back(event.clone());
        }
        if let Some(snapshot_id) = event.snapshot_id {
            self.snapshot_replay_cursors
                .insert(snapshot_id, event.cursor);
        }
        self.replay_events.push_back(event);
        let absolute_index = self.replay_cursor_offset_base + self.replay_events.len() - 1;
        let latest_cursor = self.replay_events.back().map(|latest| latest.cursor);
        if let Some(cursor) = latest_cursor {
            self.replay_cursor_offsets.insert(cursor, absolute_index);
        }
        let limit = self.policy.retention_budget.history_limit.max(1) * 32;
        while self.replay_events.len() > limit {
            if let Some(event) = self.replay_events.pop_front() {
                self.replay_cursor_offset_base += 1;
                self.remove_replay_event_from_index(&event);
            }
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

    pub fn allocate_snapshot_meta(
        &mut self,
        policy: SignalRuntimePolicy,
        artifact_retention: SnapshotArtifactRetentionPolicy,
    ) -> SignalSnapshotMeta {
        self.bootstrap_defaults();
        let snapshot_id = SignalSnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        let branch = self.active_branch();
        let replay_head = self.replay_events.back().map(|frame| frame.cursor);
        let meta = SignalSnapshotMeta::new(
            snapshot_id,
            &branch,
            replay_head,
            policy,
            artifact_retention,
        );
        if let Some(branch_entry) = self.branch_catalog.get_mut(&branch.id) {
            branch_entry.head_snapshot_id = Some(snapshot_id);
        }
        meta
    }

    pub fn create_branch(&mut self, name: impl Into<String>) -> SignalBranchHandle {
        self.bootstrap_defaults();
        let handle = SignalBranchHandle {
            id: SignalBranchId(self.next_branch_id.max(1)),
            name: name.into(),
            parent_branch_id: Some(self.active_branch),
            head_snapshot_id: self
                .branch_catalog
                .get(&self.active_branch)
                .and_then(|branch| branch.head_snapshot_id),
        };
        self.next_branch_id = handle.id.0 + 1;
        self.branch_catalog.insert(handle.id, handle.clone());
        handle
    }

    pub fn set_active_branch(&mut self, branch_id: SignalBranchId) {
        self.bootstrap_defaults();
        self.active_branch = branch_id;
    }

    pub fn set_branch_head_snapshot(
        &mut self,
        branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    ) {
        self.bootstrap_defaults();
        if let Some(branch) = self.branch_catalog.get_mut(&branch_id) {
            branch.head_snapshot_id = Some(snapshot_id);
        }
    }

    pub fn synchronize_branch_catalog(
        &mut self,
        branch_catalog: &BTreeMap<SignalBranchId, SignalBranchHandle>,
        active_branch: SignalBranchId,
    ) {
        self.branch_catalog.clone_from(branch_catalog);
        self.active_branch = active_branch;
    }

    pub fn allocate_lineage_artifact_id(&mut self) -> LineageArtifactId {
        let artifact_id = LineageArtifactId(self.next_lineage_artifact_id);
        self.next_lineage_artifact_id += 1;
        artifact_id
    }

    pub fn allocate_lineage_sequence(&mut self) -> u64 {
        let sequence = self.next_lineage_sequence;
        self.next_lineage_sequence += 1;
        sequence
    }

    pub fn branch_snapshot_allocator_state(&self) -> (u64, u64) {
        (self.next_snapshot_id, self.next_branch_id)
    }

    pub fn synchronize_branch_snapshot_allocator(
        &mut self,
        next_snapshot_id: u64,
        next_branch_id: u64,
    ) {
        self.next_snapshot_id = self.next_snapshot_id.max(next_snapshot_id);
        self.next_branch_id = self.next_branch_id.max(next_branch_id);
    }

    pub fn lineage_allocator_state(&self) -> (u64, u64) {
        (self.next_lineage_artifact_id, self.next_lineage_sequence)
    }

    pub fn synchronize_lineage_allocator(
        &mut self,
        next_lineage_artifact_id: u64,
        next_lineage_sequence: u64,
    ) {
        self.next_lineage_artifact_id = self.next_lineage_artifact_id.max(next_lineage_artifact_id);
        self.next_lineage_sequence = self.next_lineage_sequence.max(next_lineage_sequence);
    }

    pub fn record_lineage_record(&mut self, record: LineageRecord) {
        if let Some(node) = record.node() {
            self.lineage_records_by_node
                .entry(node)
                .or_default()
                .push_back(record.clone());
        }
        if let Some(artifact_id) = record.subject_artifact_id() {
            self.lineage_records_by_artifact
                .entry(artifact_id)
                .or_default()
                .push_back(record.clone());
        }
        self.lineage_records.push_back(record);
        let limit = self.policy.retention_budget.history_limit.max(1) * 32;
        while self.lineage_records.len() > limit {
            if let Some(record) = self.lineage_records.pop_front() {
                self.remove_lineage_record_from_index(&record);
            }
        }
    }

    pub fn snapshot_payload_with_retention(
        &self,
        artifact_retention: SnapshotArtifactRetentionPolicy,
    ) -> SignalSnapshotDiagnostics {
        SignalSnapshotDiagnostics {
            latest_flow: self.latest_flow.clone(),
            latest_failure: self.latest_failure.clone(),
            latest_rollback: self.latest_rollback.clone(),
            recent_history: self.recent_history.clone(),
            replay_frames: self.replay_events.clone(),
            explanation_facts: if artifact_retention.retains_explanation_facts() {
                self.explanation_facts.clone()
            } else {
                BTreeMap::new()
            },
            provenance_facts: if artifact_retention.retains_provenance_facts() {
                self.provenance_facts.clone()
            } else {
                BTreeMap::new()
            },
            lineage_records: self.lineage_records.clone(),
            branch_catalog: self.branch_catalog.clone(),
            active_branch: self.active_branch,
            next_replay_cursor: self.next_replay_cursor,
            next_snapshot_id: self.next_snapshot_id,
            next_branch_id: self.next_branch_id,
            next_lineage_artifact_id: self.next_lineage_artifact_id,
            next_lineage_sequence: self.next_lineage_sequence,
        }
    }

    pub fn restore_snapshot_payload(&mut self, payload: SignalSnapshotDiagnostics) {
        self.latest_flow = payload.latest_flow;
        self.latest_failure = payload.latest_failure;
        self.latest_rollback = payload.latest_rollback;
        self.recent_history = payload.recent_history;
        self.replay_events = payload.replay_frames;
        self.explanation_facts = payload.explanation_facts;
        self.provenance_facts = payload.provenance_facts;
        self.lineage_records = payload.lineage_records;
        self.branch_catalog = payload.branch_catalog;
        self.active_branch = payload.active_branch;
        self.next_replay_cursor = payload.next_replay_cursor;
        self.next_snapshot_id = payload.next_snapshot_id;
        self.next_branch_id = payload.next_branch_id;
        self.next_lineage_artifact_id = payload.next_lineage_artifact_id;
        self.next_lineage_sequence = payload.next_lineage_sequence;
        self.pending_input = None;
        self.pending_graph_summary = None;
        self.rebuild_indexes();
    }

    pub fn restore_snapshot_payload_preserving_history_from(
        &mut self,
        payload: SignalSnapshotDiagnostics,
        current: &DiagnosticsState,
    ) {
        let current_recent_history = current.recent_history.clone();
        let current_replay_events = current.replay_events.clone();
        let current_lineage_records = current.lineage_records.clone();
        let current_branch_catalog = current.branch_catalog.clone();
        let current_latest_failure = current.latest_failure.clone();
        let current_latest_rollback = current.latest_rollback.clone();
        let current_next_replay_cursor = current.next_replay_cursor;
        let current_next_snapshot_id = current.next_snapshot_id;
        let current_next_branch_id = current.next_branch_id;
        let current_next_lineage_artifact_id = current.next_lineage_artifact_id;
        let current_next_lineage_sequence = current.next_lineage_sequence;

        let payload_latest_execution_record_id = payload
            .recent_history
            .iter()
            .filter_map(|summary| summary.latest_execution_record_id)
            .max();
        let payload_last_replay_cursor = payload.replay_frames.back().map(|event| event.cursor);
        let payload_last_lineage_sequence =
            payload.lineage_records.back().map(|record| record.sequence);

        self.restore_snapshot_payload(payload);

        for summary in current_recent_history {
            let current_latest = summary.latest_execution_record_id;
            if payload_latest_execution_record_id
                .is_none_or(|latest| current_latest.is_some_and(|current| current > latest))
            {
                self.recent_history.push_back(summary);
            }
        }
        for event in current_replay_events {
            if payload_last_replay_cursor.is_none_or(|latest| event.cursor > latest) {
                self.replay_events.push_back(event);
            }
        }
        for record in current_lineage_records {
            if payload_last_lineage_sequence.is_none_or(|latest| record.sequence > latest) {
                self.lineage_records.push_back(record);
            }
        }

        if current_latest_failure.is_some() {
            self.latest_failure = current_latest_failure;
        }
        if current_latest_rollback.is_some() {
            self.latest_rollback = current_latest_rollback;
        }
        for (branch_id, branch_handle) in current_branch_catalog {
            match self.branch_catalog.get_mut(&branch_id) {
                Some(existing) if branch_id != self.active_branch => {
                    if existing.head_snapshot_id.is_none() {
                        existing.head_snapshot_id = branch_handle.head_snapshot_id;
                    }
                }
                None => {
                    self.branch_catalog.insert(branch_id, branch_handle);
                }
                _ => {}
            }
        }

        self.next_replay_cursor = self.next_replay_cursor.max(current_next_replay_cursor);
        self.next_snapshot_id = self.next_snapshot_id.max(current_next_snapshot_id);
        self.next_branch_id = self.next_branch_id.max(current_next_branch_id);
        self.next_lineage_artifact_id = self
            .next_lineage_artifact_id
            .max(current_next_lineage_artifact_id);
        self.next_lineage_sequence = self
            .next_lineage_sequence
            .max(current_next_lineage_sequence);
        self.trim_history();
        self.rebuild_indexes();
        let limit = self.policy.retention_budget.history_limit.max(1) * 32;
        while self.replay_events.len() > limit {
            if let Some(event) = self.replay_events.pop_front() {
                self.remove_replay_event_from_index(&event);
            }
        }
        while self.lineage_records.len() > limit {
            if let Some(record) = self.lineage_records.pop_front() {
                self.remove_lineage_record_from_index(&record);
            }
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

    fn trim_history(&mut self) {
        let limit = self.policy.retention_budget.history_limit;
        while self.recent_history.len() > limit {
            self.recent_history.pop_front();
        }
    }

    fn rebuild_indexes(&mut self) {
        self.replay_events_by_branch.clear();
        self.replay_events_by_node.clear();
        self.replay_events_by_artifact.clear();
        self.replay_cursor_offsets.clear();
        self.snapshot_replay_cursors.clear();
        self.replay_cursor_offset_base = 0;
        for event in &self.replay_events {
            self.replay_events_by_branch
                .entry(event.branch_id)
                .or_default()
                .push_back(event.clone());
            if let Some(node) = event.node {
                self.replay_events_by_node
                    .entry(node)
                    .or_default()
                    .push_back(event.clone());
            }
            if let Some(artifact_id) = event.lineage_artifact_id {
                self.replay_events_by_artifact
                    .entry(artifact_id)
                    .or_default()
                    .push_back(event.clone());
            }
            if let Some(snapshot_id) = event.snapshot_id {
                self.snapshot_replay_cursors
                    .insert(snapshot_id, event.cursor);
            }
        }
        self.rebuild_replay_cursor_offsets();
        self.lineage_records_by_artifact.clear();
        self.lineage_records_by_node.clear();
        for record in &self.lineage_records {
            if let Some(node) = record.node() {
                self.lineage_records_by_node
                    .entry(node)
                    .or_default()
                    .push_back(record.clone());
            }
            if let Some(artifact_id) = record.subject_artifact_id() {
                self.lineage_records_by_artifact
                    .entry(artifact_id)
                    .or_default()
                    .push_back(record.clone());
            }
        }
    }

    fn remove_replay_event_from_index(&mut self, event: &ReplayEvent) {
        let mut remove_branch = false;
        if let Some(events) = self.replay_events_by_branch.get_mut(&event.branch_id) {
            if let Some(front) = events.front() {
                if front == event {
                    events.pop_front();
                } else if let Some(index) = events.iter().position(|candidate| candidate == event) {
                    events.remove(index);
                }
            }
            remove_branch = events.is_empty();
        }
        if remove_branch {
            self.replay_events_by_branch.remove(&event.branch_id);
        }
        if let Some(node) = event.node {
            let mut remove_node = false;
            if let Some(events) = self.replay_events_by_node.get_mut(&node) {
                if let Some(front) = events.front() {
                    if front == event {
                        events.pop_front();
                    } else if let Some(index) =
                        events.iter().position(|candidate| candidate == event)
                    {
                        events.remove(index);
                    }
                }
                remove_node = events.is_empty();
            }
            if remove_node {
                self.replay_events_by_node.remove(&node);
            }
        }
        if let Some(artifact_id) = event.lineage_artifact_id {
            let mut remove_artifact = false;
            if let Some(events) = self.replay_events_by_artifact.get_mut(&artifact_id) {
                if let Some(front) = events.front() {
                    if front == event {
                        events.pop_front();
                    } else if let Some(index) =
                        events.iter().position(|candidate| candidate == event)
                    {
                        events.remove(index);
                    }
                }
                remove_artifact = events.is_empty();
            }
            if remove_artifact {
                self.replay_events_by_artifact.remove(&artifact_id);
            }
        }
        self.replay_cursor_offsets.remove(&event.cursor);
        if event.snapshot_id.is_some() {
            self.snapshot_replay_cursors
                .retain(|_, cursor| *cursor != event.cursor);
        }
    }

    fn remove_lineage_record_from_index(&mut self, record: &LineageRecord) {
        if let Some(node) = record.node() {
            let mut remove_node = false;
            if let Some(records) = self.lineage_records_by_node.get_mut(&node) {
                if let Some(front) = records.front() {
                    if front == record {
                        records.pop_front();
                    } else if let Some(index) =
                        records.iter().position(|candidate| candidate == record)
                    {
                        records.remove(index);
                    }
                }
                remove_node = records.is_empty();
            }
            if remove_node {
                self.lineage_records_by_node.remove(&node);
            }
        }
        let Some(artifact_id) = record.subject_artifact_id() else {
            return;
        };
        let mut remove_artifact = false;
        if let Some(records) = self.lineage_records_by_artifact.get_mut(&artifact_id) {
            if let Some(front) = records.front() {
                if front == record {
                    records.pop_front();
                } else if let Some(index) = records.iter().position(|candidate| candidate == record)
                {
                    records.remove(index);
                }
            }
            remove_artifact = records.is_empty();
        }
        if remove_artifact {
            self.lineage_records_by_artifact.remove(&artifact_id);
        }
    }

    fn rebuild_replay_cursor_offsets(&mut self) {
        self.replay_cursor_offsets.clear();
        self.replay_cursor_offset_base = 0;
        for (index, event) in self.replay_events.iter().enumerate() {
            self.replay_cursor_offsets.insert(event.cursor, index);
        }
    }
}

impl Default for DiagnosticsState {
    fn default() -> Self {
        let mut state = Self {
            policy: SignalRuntimePolicy::default(),
            latest_flow: None,
            latest_failure: None,
            latest_rollback: None,
            latest_graph_summary: None,
            pending_graph_summary: None,
            recent_history: VecDeque::new(),
            replay_events: VecDeque::new(),
            lineage_records: VecDeque::new(),
            replay_events_by_branch: BTreeMap::new(),
            replay_events_by_node: BTreeMap::new(),
            replay_events_by_artifact: BTreeMap::new(),
            replay_cursor_offsets: BTreeMap::new(),
            replay_cursor_offset_base: 0,
            snapshot_replay_cursors: BTreeMap::new(),
            lineage_records_by_artifact: BTreeMap::new(),
            lineage_records_by_node: BTreeMap::new(),
            explanation_facts: BTreeMap::new(),
            provenance_facts: BTreeMap::new(),
            branch_catalog: BTreeMap::new(),
            active_branch: SignalBranchId(0),
            next_replay_cursor: 0,
            next_snapshot_id: 0,
            next_branch_id: 1,
            next_lineage_artifact_id: 0,
            next_lineage_sequence: 0,
            pending_input: None,
            latest_frontier_execution: None,
            latest_invalidation_trace_records: Vec::new(),
        };
        state.bootstrap_defaults();
        state
    }
}
