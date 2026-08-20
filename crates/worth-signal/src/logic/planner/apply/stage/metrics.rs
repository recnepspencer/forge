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
        let touched = dirty.touched_nodes.len() as u64;
        let structural = dirty.changed_regions.as_slice().len() as u64 + touched;
        graph.with_telemetry(|telemetry| {
            telemetry.invalidation.dirty_delta_breadth += touched;
            telemetry.storage.structural_delta_size += structural;
        });
    }
    match lowered {
        LoweredStageExecutionForm::Serial(stage) => {
            let batch_width = stage.stage_width() as u64;
            graph.with_telemetry(|telemetry| {
                telemetry.execution.apply_group_width_total += batch_width;
                telemetry.execution.max_apply_group_width =
                    telemetry.execution.max_apply_group_width.max(batch_width);
            });
        }
        LoweredStageExecutionForm::Generic(stage) => {
            let apply_groups = stage.apply_groups();
            let group_width = apply_groups
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .sum::<u64>();
            let max_group_width = apply_groups
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .max()
                .unwrap_or(0);
            let disjoint = apply_groups
                .iter()
                .map(|group| group.task_indices.len().saturating_sub(1) as u64)
                .sum::<u64>();
            graph.with_telemetry(|telemetry| {
                telemetry.execution.apply_group_width_total += group_width;
                telemetry.execution.max_apply_group_width = telemetry
                    .execution
                    .max_apply_group_width
                    .max(max_group_width);
                telemetry.execution.apply_group_disjoint_count += disjoint;
            });
        }
    }
    match maintenance_strategy {
        ResolvedMaintenanceStrategy::Incremental | ResolvedMaintenanceStrategy::DensityAdaptive => {
            graph.with_telemetry(|telemetry| telemetry.planner.incremental_strategy_count += 1);
        }
        ResolvedMaintenanceStrategy::Rebuild => {
            graph.with_telemetry(|telemetry| telemetry.planner.rebuild_strategy_count += 1);
        }
    }
}
