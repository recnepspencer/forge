use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::replay::ReplayFrame;
use crate::diagnostics::summary::ExecutionHistorySummary;
use crate::diagnostics::{FailureSummary, RollbackDiagnostic};
use crate::logic::transaction::ObservationBoundarySummary;

use super::lifecycle::{SignalBranchHandle, SignalBranchId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Snapshot-resident diagnostics payload needed for deterministic restore,
/// replay inspection, and lineage continuity.
pub struct SignalSnapshotDiagnostics {
    pub latest_flow: Option<FlowSummary>,
    pub latest_failure: Option<FailureSummary>,
    pub latest_rollback: Option<RollbackDiagnostic>,
    pub latest_observation: Option<ObservationBoundarySummary>,
    pub recent_history: VecDeque<ExecutionHistorySummary>,
    pub replay_frames: VecDeque<ReplayFrame>,
    pub explanation_facts: BTreeMap<NodeId, ExplanationFact>,
    pub provenance_facts: BTreeMap<NodeId, ProvenanceFact>,
    pub lineage_records: VecDeque<LineageRecord>,
    pub branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
    pub active_branch: SignalBranchId,
    pub next_replay_cursor: u64,
    pub next_snapshot_id: u64,
    pub next_branch_id: u64,
    pub next_lineage_artifact_id: u64,
    pub next_lineage_sequence: u64,
    #[serde(default)]
    pub observation_activation_mask: u8,
}
