mod branching;
mod indexes;
mod lifecycle;
mod lineage;
mod replay;
mod restore_history;
mod retained;
mod snapshot;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::proof::{
    FrontierDiagnosticsSidecar, InvalidationPlanningEstimate, InvalidationTraceRecord,
};
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent};
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::transaction::ObservationBoundarySummary;
use crate::runtime_policy::SignalRuntimePolicy;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct DiagnosticsState {
    #[serde(default)]
    request_mirror: SignalRuntimePolicy,
    #[serde(skip)]
    installed_retention_budget: crate::diagnostics::policy::RetentionBudget,
    #[serde(skip)]
    installed_tier: crate::diagnostics::profile::DiagnosticsTier,
    #[serde(skip)]
    installed_frontier_tracing_policy: crate::diagnostics::policy::FrontierTracingPolicy,
    #[serde(default)]
    latest_flow: Option<FlowSummary>,
    #[serde(default)]
    latest_failure: Option<FailureSummary>,
    #[serde(default)]
    latest_rollback: Option<RollbackDiagnostic>,
    #[serde(default)]
    latest_observation: Option<ObservationBoundarySummary>,
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
    latest_frontier_execution: Option<FrontierDiagnosticsSidecar>,
    #[serde(default)]
    latest_invalidation_planning_estimate: Option<InvalidationPlanningEstimate>,
    #[serde(default)]
    latest_invalidation_trace_records: Vec<InvalidationTraceRecord>,
    /// Surfaces that have been explicitly activated for observation on this
    /// graph. This remains separate from the current policy so historical
    /// reads can distinguish inactive evidence from policy omission.
    #[serde(default)]
    observation_activation_mask: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingFlowInput {
    changed_nodes: BTreeSet<NodeId>,
    changed_aspects: BTreeSet<u8>,
    changed_region_count: u32,
    causality_kind: Option<String>,
}
