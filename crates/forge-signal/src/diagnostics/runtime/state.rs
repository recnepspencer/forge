use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::{ChangeInputSummary, FlowSummary, InvalidationSummary};
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent};
use crate::diagnostics::summary::ExecutionHistorySummary;
use crate::state::{
    SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
    SignalSnapshotMeta,
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
    recent_history: VecDeque<ExecutionHistorySummary>,
    #[serde(default)]
    replay_events: VecDeque<ReplayEvent>,
    #[serde(default)]
    lineage_records: VecDeque<LineageRecord>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingFlowInput {
    changed_nodes: BTreeSet<NodeId>,
    changed_aspects: BTreeSet<u8>,
    changed_region_count: u32,
    causality_kind: Option<String>,
    invalidated_direct_subscribers: u32,
    maybe_stale_direct_subscribers: u32,
    partition_scoped_checks: u32,
    narrowed_frontier_width: u32,
    transitive_frontier_width: u32,
}

impl DiagnosticsState {
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

    pub fn profile(&self) -> DiagnosticsProfile {
        self.policy.profile
    }

    pub fn set_profile(&mut self, profile: DiagnosticsProfile) {
        self.policy = SignalRuntimePolicy::from_profile(profile);
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

    pub fn recent_history(&self) -> &VecDeque<ExecutionHistorySummary> {
        &self.recent_history
    }

    pub fn replay_events(&self) -> &VecDeque<ReplayEvent> {
        &self.replay_events
    }

    pub fn lineage_records(&self) -> &VecDeque<LineageRecord> {
        &self.lineage_records
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
            invalidated_direct_subscribers: 0,
            maybe_stale_direct_subscribers: 0,
            partition_scoped_checks: 0,
            narrowed_frontier_width: 0,
            transitive_frontier_width: 0,
        });
        pending.changed_nodes.insert(node);
        pending.changed_aspects.insert(aspect.id());
        pending.changed_region_count += changed_regions.len() as u32;
        if pending.causality_kind.is_none() {
            pending.causality_kind = causality_kind;
        }
    }

    pub fn record_invalidation_result(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
        narrowed_frontier_width: u32,
        transitive_frontier_width: u32,
    ) {
        if let Some(pending) = &mut self.pending_input {
            pending.invalidated_direct_subscribers += invalidated_direct_subscribers;
            pending.maybe_stale_direct_subscribers += maybe_stale_direct_subscribers;
            pending.partition_scoped_checks += partition_scoped_checks;
            pending.narrowed_frontier_width += narrowed_frontier_width;
            pending.transitive_frontier_width += transitive_frontier_width;
        }
    }

    pub fn complete_flow(&mut self, flow: FlowSummary, history: ExecutionHistorySummary) {
        self.latest_flow = Some(flow);
        self.recent_history.push_back(history);
        self.trim_history();
        self.pending_input = None;
    }

    pub fn record_failure(&mut self, failure: FailureSummary) {
        self.latest_failure = Some(failure);
    }

    pub fn record_rollback(&mut self, rollback: RollbackDiagnostic) {
        self.latest_rollback = Some(rollback);
    }

    pub fn clear_pending_input(&mut self) {
        self.pending_input = None;
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

    pub fn record_replay_event(&mut self, event: ReplayEvent) {
        self.replay_events.push_back(event);
        let limit = self.policy.history_limit.max(1) * 32;
        while self.replay_events.len() > limit {
            self.replay_events.pop_front();
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

    pub fn allocate_snapshot_meta(&mut self, policy: SignalRuntimePolicy) -> SignalSnapshotMeta {
        self.bootstrap_defaults();
        let snapshot_id = SignalSnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        let branch = self.active_branch();
        let replay_head = self.replay_events.back().map(|frame| frame.cursor);
        let meta = SignalSnapshotMeta::new(snapshot_id, &branch, replay_head, policy);
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
        branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
        active_branch: SignalBranchId,
    ) {
        self.branch_catalog = branch_catalog;
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
        self.lineage_records.push_back(record);
        let limit = self.policy.history_limit.max(1) * 32;
        while self.lineage_records.len() > limit {
            self.lineage_records.pop_front();
        }
    }

    pub fn snapshot_payload(&self) -> SignalSnapshotDiagnostics {
        SignalSnapshotDiagnostics {
            latest_flow: self.latest_flow.clone(),
            latest_failure: self.latest_failure.clone(),
            latest_rollback: self.latest_rollback.clone(),
            recent_history: self.recent_history.clone(),
            replay_frames: self.replay_events.clone(),
            explanation_facts: self.explanation_facts.clone(),
            provenance_facts: self.provenance_facts.clone(),
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
        let limit = self.policy.history_limit.max(1) * 32;
        while self.replay_events.len() > limit {
            self.replay_events.pop_front();
        }
        while self.lineage_records.len() > limit {
            self.lineage_records.pop_front();
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
                InvalidationSummary::new(
                    pending.invalidated_direct_subscribers,
                    pending.maybe_stale_direct_subscribers,
                    pending.partition_scoped_checks,
                    pending.narrowed_frontier_width,
                    pending.transitive_frontier_width,
                ),
            )
        })
    }

    fn trim_history(&mut self) {
        let limit = self.policy.history_limit;
        while self.recent_history.len() > limit {
            self.recent_history.pop_front();
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
            recent_history: VecDeque::new(),
            replay_events: VecDeque::new(),
            lineage_records: VecDeque::new(),
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
        };
        state.bootstrap_defaults();
        state
    }
}
