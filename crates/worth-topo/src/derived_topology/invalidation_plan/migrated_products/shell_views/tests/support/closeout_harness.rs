use super::super::super::{
    close_shell_view_migration_slice, ShellViewBoundarySourceRow, ShellViewExecutionInput,
    ShellViewMigrationCloseout, ShellViewReadSource, ShellViewReadStageExecutor,
};
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

pub(crate) fn close_shell_view_slice_from_topology(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
    topology: &TopologyView,
) -> ShellViewMigrationCloseout {
    let read_source =
        ShellViewReadSource::select_from_touched_closure(plan, touched_closure, topology).unwrap();
    let read_receipt = ShellViewReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        ShellViewExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_shell_view_migration_slice(plan, input).unwrap()
}

pub(crate) fn admitted_input(
    plan: &DerivedInvalidationSelectedPlan,
    rows: Vec<ShellViewBoundarySourceRow>,
    available_source_row_count: usize,
) -> ShellViewExecutionInput {
    let read_source = ShellViewReadSource::from_rows(rows, available_source_row_count).unwrap();
    let read_receipt = ShellViewReadStageExecutor::execute(plan, read_source).unwrap();
    ShellViewExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap()
}
