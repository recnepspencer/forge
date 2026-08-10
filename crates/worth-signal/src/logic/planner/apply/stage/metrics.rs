use crate::data::graph::SignalGraph;
use crate::data::performance::ResolvedMaintenanceStrategy;

use super::lowering::LoweredStageExecutionForm;

pub(super) fn record_stage_lowering_metrics(
    graph: &mut SignalGraph,
    lowered: &LoweredStageExecutionForm,
) {
    let (dirty_delta, maintenance_strategy) = match lowered {
        LoweredStageExecutionForm::Serial(stage) => {
            (stage.dirty_delta(), stage.maintenance_strategy())
        }
        LoweredStageExecutionForm::Generic(stage) => {
            (stage.dirty_delta(), stage.maintenance_strategy())
        }
    };
    if let Some(dirty) = dirty_delta.dirty.as_ref() {
        graph.telemetry_mut().invalidation.dirty_delta_breadth += dirty.touched_nodes.len() as u64;
        graph.telemetry_mut().storage.structural_delta_size +=
            dirty.changed_regions.as_slice().len() as u64 + dirty.touched_nodes.len() as u64;
    }
    match lowered {
        LoweredStageExecutionForm::Serial(stage) => {
            let batch_width = stage.stage_width() as u64;
            graph.telemetry_mut().execution.apply_group_width_total += batch_width;
            graph.telemetry_mut().execution.max_apply_group_width = graph
                .telemetry()
                .execution
                .max_apply_group_width
                .max(batch_width);
        }
        LoweredStageExecutionForm::Generic(stage) => {
            let apply_groups = stage.apply_groups();
            graph.telemetry_mut().execution.apply_group_width_total += apply_groups
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .sum::<u64>();
            graph.telemetry_mut().execution.max_apply_group_width =
                graph.telemetry().execution.max_apply_group_width.max(
                    apply_groups
                        .iter()
                        .map(|group| group.task_indices.len() as u64)
                        .max()
                        .unwrap_or(0),
                );
            graph.telemetry_mut().execution.apply_group_disjoint_count += apply_groups
                .iter()
                .map(|group| group.task_indices.len().saturating_sub(1) as u64)
                .sum::<u64>();
        }
    }
    match maintenance_strategy {
        ResolvedMaintenanceStrategy::Incremental | ResolvedMaintenanceStrategy::DensityAdaptive => {
            graph.telemetry_mut().planner.incremental_strategy_count += 1;
        }
        ResolvedMaintenanceStrategy::Rebuild => {
            graph.telemetry_mut().planner.rebuild_strategy_count += 1;
        }
    }
}
