use super::super::super::{
    close_radial_ring_migration_slice, RadialRingBoundarySourceRow, RadialRingExecutionInput,
    RadialRingMigrationCloseout, RadialRingReadSource, RadialRingReadStageExecutor,
};
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

pub(crate) fn close_radial_ring_slice_from_topology(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
    topology: &TopologyView,
) -> RadialRingMigrationCloseout {
    let read_source =
        RadialRingReadSource::select_from_touched_closure(plan, touched_closure, topology).unwrap();
    let read_receipt = RadialRingReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        RadialRingExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_radial_ring_migration_slice(plan, input).unwrap()
}

pub(crate) fn admitted_input(
    plan: &DerivedInvalidationSelectedPlan,
    rows: Vec<RadialRingBoundarySourceRow>,
    available_source_row_count: usize,
) -> RadialRingExecutionInput {
    let read_source = RadialRingReadSource::from_rows(rows, available_source_row_count).unwrap();
    let read_receipt = RadialRingReadStageExecutor::execute(plan, read_source).unwrap();
    RadialRingExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap()
}
