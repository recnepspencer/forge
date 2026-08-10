use std::collections::BTreeMap;

use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent};
use crate::diagnostics::summary::ExecutionHistorySummary;
use crate::logic::transaction::ObservationBoundarySummary;
use crate::state::{
    SignalBranchHandle, SignalBranchId, SignalSnapshotDiagnostics, SignalSnapshotId,
    SignalSnapshotMeta, SnapshotArtifactRetentionPolicy,
};

use super::DiagnosticsState;

impl DiagnosticsState {
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

    pub fn snapshot_payload_with_retention(
        &self,
        artifact_retention: SnapshotArtifactRetentionPolicy,
    ) -> SignalSnapshotDiagnostics {
        SignalSnapshotDiagnostics {
            latest_flow: self.latest_flow.clone(),
            latest_failure: self.latest_failure.clone(),
            latest_rollback: self.latest_rollback.clone(),
            latest_observation: self.latest_observation.clone(),
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
        self.latest_observation = payload.latest_observation;
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
        let preservation = SnapshotHistoryPreservation::capture(current, &payload);

        self.restore_snapshot_payload(payload);
        preservation.merge_into(self);
        self.trim_restored_history();
    }
}

struct SnapshotHistoryPreservation {
    recent_history: std::collections::VecDeque<ExecutionHistorySummary>,
    replay_events: std::collections::VecDeque<ReplayEvent>,
    lineage_records: std::collections::VecDeque<LineageRecord>,
    branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
    latest_failure: Option<FailureSummary>,
    latest_rollback: Option<RollbackDiagnostic>,
    latest_observation: Option<ObservationBoundarySummary>,
    next_replay_cursor: u64,
    next_snapshot_id: u64,
    next_branch_id: u64,
    next_lineage_artifact_id: u64,
    next_lineage_sequence: u64,
    payload_latest_execution_record_id: Option<u64>,
    payload_last_replay_cursor: Option<ReplayCursor>,
    payload_last_lineage_sequence: Option<u64>,
}

impl SnapshotHistoryPreservation {
    fn capture(current: &DiagnosticsState, payload: &SignalSnapshotDiagnostics) -> Self {
        Self {
            recent_history: current.recent_history.clone(),
            replay_events: current.replay_events.clone(),
            lineage_records: current.lineage_records.clone(),
            branch_catalog: current.branch_catalog.clone(),
            latest_failure: current.latest_failure.clone(),
            latest_rollback: current.latest_rollback.clone(),
            latest_observation: current.latest_observation.clone(),
            next_replay_cursor: current.next_replay_cursor,
            next_snapshot_id: current.next_snapshot_id,
            next_branch_id: current.next_branch_id,
            next_lineage_artifact_id: current.next_lineage_artifact_id,
            next_lineage_sequence: current.next_lineage_sequence,
            payload_latest_execution_record_id: payload
                .recent_history
                .iter()
                .filter_map(|summary| summary.latest_execution_record_id)
                .max(),
            payload_last_replay_cursor: payload.replay_frames.back().map(|event| event.cursor),
            payload_last_lineage_sequence: payload
                .lineage_records
                .back()
                .map(|record| record.sequence),
        }
    }

    fn merge_into(self, state: &mut DiagnosticsState) {
        let Self {
            recent_history,
            replay_events,
            lineage_records,
            branch_catalog,
            latest_failure,
            latest_rollback,
            latest_observation,
            next_replay_cursor,
            next_snapshot_id,
            next_branch_id,
            next_lineage_artifact_id,
            next_lineage_sequence,
            payload_latest_execution_record_id,
            payload_last_replay_cursor,
            payload_last_lineage_sequence,
        } = self;

        merge_recent_history_and_replay(
            state,
            recent_history,
            replay_events,
            payload_latest_execution_record_id,
            payload_last_replay_cursor,
        );
        merge_lineage_records(state, lineage_records, payload_last_lineage_sequence);
        restore_latest_diagnostics(state, latest_failure, latest_rollback, latest_observation);
        merge_branch_catalog(state, branch_catalog);

        state.next_replay_cursor = state.next_replay_cursor.max(next_replay_cursor);
        state.next_snapshot_id = state.next_snapshot_id.max(next_snapshot_id);
        state.next_branch_id = state.next_branch_id.max(next_branch_id);
        state.next_lineage_artifact_id =
            state.next_lineage_artifact_id.max(next_lineage_artifact_id);
        state.next_lineage_sequence = state.next_lineage_sequence.max(next_lineage_sequence);
    }
}

fn merge_recent_history_and_replay(
    state: &mut DiagnosticsState,
    recent_history: std::collections::VecDeque<ExecutionHistorySummary>,
    replay_events: std::collections::VecDeque<ReplayEvent>,
    payload_latest_execution_record_id: Option<u64>,
    payload_last_replay_cursor: Option<ReplayCursor>,
) {
    for summary in recent_history {
        let current_latest = summary.latest_execution_record_id;
        if payload_latest_execution_record_id
            .is_none_or(|latest| current_latest.is_some_and(|current| current > latest))
        {
            state.recent_history.push_back(summary);
        }
    }
    for event in replay_events {
        if payload_last_replay_cursor.is_none_or(|latest| event.cursor > latest) {
            state.replay_events.push_back(event);
        }
    }
}

fn merge_lineage_records(
    state: &mut DiagnosticsState,
    lineage_records: std::collections::VecDeque<LineageRecord>,
    payload_last_lineage_sequence: Option<u64>,
) {
    for record in lineage_records {
        if payload_last_lineage_sequence.is_none_or(|latest| record.sequence > latest) {
            state.lineage_records.push_back(record);
        }
    }
}

fn restore_latest_diagnostics(
    state: &mut DiagnosticsState,
    latest_failure: Option<FailureSummary>,
    latest_rollback: Option<RollbackDiagnostic>,
    latest_observation: Option<ObservationBoundarySummary>,
) {
    if latest_failure.is_some() {
        state.latest_failure = latest_failure;
    }
    if latest_rollback.is_some() {
        state.latest_rollback = latest_rollback;
    }
    if let Some(observation) = latest_observation {
        state.latest_observation = Some(observation.clone());
        if let Some(flow) = &mut state.latest_flow {
            flow.observation = Some(observation);
        }
    }
}

fn merge_branch_catalog(
    state: &mut DiagnosticsState,
    branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
) {
    for (branch_id, branch_handle) in branch_catalog {
        match state.branch_catalog.get_mut(&branch_id) {
            Some(existing) if branch_id != state.active_branch => {
                if existing.head_snapshot_id.is_none() {
                    existing.head_snapshot_id = branch_handle.head_snapshot_id;
                }
            }
            None => {
                state.branch_catalog.insert(branch_id, branch_handle);
            }
            _ => {}
        }
    }
}

impl DiagnosticsState {
    fn trim_restored_history(&mut self) {
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
}
