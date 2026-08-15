use std::collections::{BTreeMap, VecDeque};

use super::DiagnosticsState;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::state::SignalBranchId;

impl DiagnosticsState {
    pub fn authority_carrier_clone(&self) -> Self {
        let mut state = Self {
            policy: self.policy,
            latest_flow: None,
            latest_failure: None,
            latest_rollback: None,
            latest_observation: None,
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
            latest_invalidation_planning_estimate: None,
            latest_invalidation_trace_records: Vec::new(),
        };
        state.bootstrap_defaults();
        state
    }
}

impl Default for DiagnosticsState {
    fn default() -> Self {
        let mut state = Self {
            policy: SignalRuntimePolicy::default(),
            latest_flow: None,
            latest_failure: None,
            latest_rollback: None,
            latest_observation: None,
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
            latest_invalidation_planning_estimate: None,
            latest_invalidation_trace_records: Vec::new(),
        };
        state.bootstrap_defaults();
        state
    }
}
