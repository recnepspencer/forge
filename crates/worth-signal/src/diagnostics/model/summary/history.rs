use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, OutputChange};
use crate::data::reuse::{PersistentCorrespondenceKind, ReuseBasis, ReuseOrigin};
use crate::diagnostics::policy::{DetailLimit, OrdinaryAccessLane};
use crate::diagnostics::profile::DiagnosticsTier;
use crate::logic::planner::ExecutionReport;

pub type ReuseOriginCounts = BTreeMap<ReuseOrigin, u32>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionHistoryNodeSummary {
    pub node: NodeId,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub output_change: Option<OutputChange>,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub reuse_basis: Option<ReuseBasis>,
    pub reuse_origin: Option<ReuseOrigin>,
    pub persistent_correspondence_kind: Option<PersistentCorrespondenceKind>,
    pub composition_region_count: u32,
    pub reuse_certification_proof_count: u32,
    pub changed_partition_count: u32,
    pub causality_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionHistorySummary {
    pub profile: DiagnosticsTier,
    pub traced_node_count: u32,
    pub execution_record_count: u32,
    pub latest_execution_record_id: Option<u64>,
    pub reuse_origin_counts: ReuseOriginCounts,
    pub nodes: Vec<ExecutionHistoryNodeSummary>,
}

impl ExecutionHistorySummary {
    pub fn with_profile(&self, profile: DiagnosticsTier) -> Self {
        let mut cloned = self.clone();
        cloned.profile = profile;
        cloned
    }

    pub fn from_graph(
        graph: &SignalGraph,
        profile: DiagnosticsTier,
        detail_limit: DetailLimit,
        retain_history_details: bool,
        _lane: OrdinaryAccessLane,
    ) -> Self {
        let mut traced_node_count = 0_u32;
        let mut execution_record_count = 0_u32;
        let mut latest_execution_record_id = None;
        let mut reuse_origin_counts = ReuseOriginCounts::new();
        let mut nodes = Vec::new();

        for index in 0..graph.arena_capacity() {
            let Some(node) = graph.live_node_id_at(index) else {
                continue;
            };
            let Ok(Some(trace)) = graph.observe().runtime_artifact_state(node) else {
                continue;
            };
            let execution_trace = graph.node_execution_trace_stamp(node).ok().flatten();
            traced_node_count += 1;
            *reuse_origin_counts.entry(trace.reuse_origin()).or_insert(0) += 1;
            if let Some(id) = execution_trace.and_then(|stamp| stamp.execution_record_id) {
                execution_record_count += 1;
                latest_execution_record_id =
                    Some(latest_execution_record_id.map_or(id, |current: u64| current.max(id)));
            }
            if retain_history_details {
                nodes.push(ExecutionHistoryNodeSummary {
                    node,
                    execution_record_id: execution_trace
                        .and_then(|stamp| stamp.execution_record_id),
                    semantic_segment_id: execution_trace
                        .and_then(|stamp| stamp.semantic_segment_id),
                    output_change: Some(trace.output_change()),
                    memoized_origin: Some(trace.memoized_origin()),
                    reuse_basis: Some(trace.reuse_basis().clone_inner()),
                    reuse_origin: Some(trace.reuse_origin()),
                    persistent_correspondence_kind: trace
                        .reuse_boundary_authority()
                        .and_then(|authority| authority.persistent_correspondence_kind()),
                    composition_region_count: trace
                        .reuse_boundary_authority()
                        .map(|authority| authority.composition_region_count())
                        .unwrap_or(0),
                    reuse_certification_proof_count: graph
                        .node_cold_artifact_record(node)
                        .unwrap_or(None)
                        .and_then(|retained| retained.reuse_certification.as_ref())
                        .map(|record| record.proofs.len() as u32)
                        .unwrap_or(0),
                    changed_partition_count: trace.changed_partition_count(),
                    causality_kind: graph
                        .causality_of(node)
                        .unwrap_or(None)
                        .map(|c| c.kind.clone()),
                });
            }
        }

        if retain_history_details {
            nodes.sort_by(|left, right| {
                right
                    .execution_record_id
                    .cmp(&left.execution_record_id)
                    .then_with(|| right.semantic_segment_id.cmp(&left.semantic_segment_id))
                    .then_with(|| left.node.index().cmp(&right.node.index()))
                    .then_with(|| left.node.generation().cmp(&right.node.generation()))
            });
            if nodes.len() > detail_limit.get() {
                nodes.truncate(detail_limit.get());
            }
        }

        Self {
            profile,
            traced_node_count,
            execution_record_count,
            latest_execution_record_id,
            reuse_origin_counts,
            nodes,
        }
    }

    pub fn from_report(
        report: &ExecutionReport,
        profile: DiagnosticsTier,
        detail_limit: DetailLimit,
        retain_history_details: bool,
    ) -> Self {
        if !retain_history_details {
            return Self {
                profile,
                traced_node_count: report.task_count,
                execution_record_count: report.task_count,
                latest_execution_record_id: report.latest_execution_record_id,
                reuse_origin_counts: report.reuse_origin_counts.clone(),
                nodes: Vec::new(),
            };
        }

        let mut traced_node_count = 0_u32;
        let mut execution_record_count = 0_u32;
        let mut latest_execution_record_id = None;
        let mut reuse_origin_counts = ReuseOriginCounts::new();
        let mut nodes = Vec::new();

        for task in report
            .stages
            .iter()
            .flat_map(|stage| stage.task_records.iter())
        {
            traced_node_count += 1;
            execution_record_count += 1;
            latest_execution_record_id = Some(
                latest_execution_record_id.map_or(task.id.0, |current: u64| current.max(task.id.0)),
            );
            *reuse_origin_counts.entry(task.reuse_origin).or_insert(0) += 1;

            if nodes.len() < detail_limit.get() {
                nodes.push(ExecutionHistoryNodeSummary {
                    node: task.node,
                    execution_record_id: Some(task.id.0),
                    semantic_segment_id: Some(task.semantic_segment_id.0),
                    output_change: None,
                    memoized_origin: Some(task.memoized_origin),
                    reuse_basis: Some(task.reuse_basis.clone()),
                    reuse_origin: Some(task.reuse_origin),
                    persistent_correspondence_kind: None,
                    composition_region_count: 0,
                    reuse_certification_proof_count: 0,
                    changed_partition_count: 0,
                    causality_kind: None,
                });
            }
        }

        Self {
            profile,
            traced_node_count,
            execution_record_count,
            latest_execution_record_id,
            reuse_origin_counts,
            nodes,
        }
    }
}
