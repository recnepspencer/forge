use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::{DedupedNodeBatch, LocallyOrderedShard};
use crate::logic::evaluation::EvaluationRequestMode;

use super::super::types::{EligibleTask, PlanSummary, StageBarrier, StageCursor};
use super::admission::admit_planned_node;
use super::topology::{compute_depths, discover_plan_topology, PlanTopology};

pub(super) fn populate_plan_buffers(
    graph: &mut SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
    out_targets: &mut Vec<NodeId>,
    out_tasks: &mut Vec<EligibleTask>,
    out_stages: &mut Vec<StageCursor>,
) -> Result<PlanSummary, SignalError> {
    let topology = discover_plan_topology(graph, targets, request_mode, resolver)?;
    let depth_cache = compute_depths(graph, &topology.planned_nodes)?;
    let staged_tasks = admit_tasks_by_depth(graph, request_mode, &topology, &depth_cache)?;

    out_targets.clear();
    out_targets.extend(topology.targets);
    out_tasks.clear();
    out_stages.clear();
    append_stage_buffers(staged_tasks, out_tasks, out_stages);
    summarize_plan(out_targets, out_tasks, out_stages, topology.stats)
}

fn admit_tasks_by_depth(
    graph: &SignalGraph,
    request_mode: EvaluationRequestMode,
    topology: &PlanTopology,
    depth_cache: &super::topology::DepthCache,
) -> Result<Vec<Vec<EligibleTask>>, SignalError> {
    let mut stages_by_depth = vec![Vec::<EligibleTask>::new(); depth_cache.max_depth() + 1];
    let planned_nodes =
        DedupedNodeBatch::canonicalize_unordered(topology.planned_nodes.iter().copied()).into_vec();
    for node in planned_nodes {
        let planned_node = topology.planned[node.index() as usize]
            .expect("planned node list should only contain planned nodes");
        let task = admit_planned_node(
            graph,
            node,
            planned_node.direct_request,
            request_mode,
            planned_node.maybe_stale_admission,
        )?;
        let depth = depth_cache.depth_for(node).ok_or_else(|| {
            SignalError::internal("planned node missing depth cache entry during stage assembly")
        })? as usize;
        stages_by_depth[depth].push(task);
    }
    Ok(stages_by_depth)
}

fn append_stage_buffers(
    staged_tasks: Vec<Vec<EligibleTask>>,
    out_tasks: &mut Vec<EligibleTask>,
    out_stages: &mut Vec<StageCursor>,
) {
    for stage_tasks in staged_tasks {
        if stage_tasks.is_empty() {
            continue;
        }
        let stage_tasks = LocallyOrderedShard::canonicalize_unordered(stage_tasks).into_vec();
        let start = out_tasks.len();
        let end = start + stage_tasks.len();
        let stage_index = out_stages.len() as u32;
        out_tasks.extend(stage_tasks);
        out_stages.push(StageCursor {
            index: stage_index,
            start,
            end,
            barrier: Some(StageBarrier::StageBoundary),
        });
    }
}

fn summarize_plan(
    out_targets: &[NodeId],
    out_tasks: &[EligibleTask],
    out_stages: &[StageCursor],
    stats: super::topology::PlanningStats,
) -> Result<PlanSummary, SignalError> {
    Ok(PlanSummary {
        requested_target_count: out_targets.len() as u32,
        stage_count: out_stages.len() as u32,
        task_count: out_tasks.len() as u32,
        max_stage_width: out_stages
            .iter()
            .map(|stage| (stage.end - stage.start) as u32)
            .max()
            .unwrap_or(0),
        contract_pruned_count: stats.contract_pruned_count,
    })
}
