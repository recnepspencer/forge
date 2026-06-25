use super::super::super::{
    close_loop_cycle_migration_slice, LoopCycleBoundarySourceRow, LoopCycleExecutionInput,
    LoopCycleMigrationCloseout, LoopCycleReadSource, LoopCycleReadStageExecutor,
};
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

pub(crate) fn close_loop_cycle_slice_from_topology(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
    topology: &TopologyView,
) -> LoopCycleMigrationCloseout {
    let read_source =
        LoopCycleReadSource::select_from_touched_closure(plan, touched_closure, topology).unwrap();
    let read_receipt = LoopCycleReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        LoopCycleExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_loop_cycle_migration_slice(plan, input).unwrap()
}

pub(crate) fn admitted_input(
    plan: &DerivedInvalidationSelectedPlan,
    rows: Vec<LoopCycleBoundarySourceRow>,
    available_source_row_count: usize,
) -> LoopCycleExecutionInput {
    let read_source = LoopCycleReadSource::from_rows(rows, available_source_row_count).unwrap();
    let read_receipt = LoopCycleReadStageExecutor::execute(plan, read_source).unwrap();
    LoopCycleExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap()
}
