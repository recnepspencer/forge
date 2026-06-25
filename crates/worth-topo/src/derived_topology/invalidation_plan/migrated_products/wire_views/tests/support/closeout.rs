use super::super::super::{
    close_wire_view_migration_slice, WireViewExecutionInput, WireViewMigrationCloseout,
    WireViewReadStageExecutor,
};
use super::query_read_rows::selected_wire_view_read_source_fixture;

pub(in super::super) fn close_wire_view_slice_from_query_read_source(
    operator_family: &'static str,
) -> WireViewMigrationCloseout {
    let fixture = selected_wire_view_read_source_fixture(operator_family);
    let read_receipt =
        WireViewReadStageExecutor::execute(&fixture.plan, fixture.read_source).unwrap();
    let input =
        WireViewExecutionInput::from_selected_plan_and_read_stage(&fixture.plan, read_receipt)
            .unwrap();
    close_wire_view_migration_slice(&fixture.plan, input).unwrap()
}
