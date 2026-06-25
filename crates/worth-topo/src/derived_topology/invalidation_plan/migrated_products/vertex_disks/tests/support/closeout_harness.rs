use super::{
    selected_vertex_disk_topology_with_unrelated_disks, selected_vertex_disk_touched_closure,
    selected_vertex_disks_plan, selected_vertex_disks_plan_with_query_read_digest,
};
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::{
    close_vertex_disk_migration_slice, VertexDiskExecutionInput, VertexDiskMigrationCloseout,
    VertexDiskReadSource, VertexDiskReadStageExecutor,
};

pub(crate) fn close_vertex_disk_slice_from_read_source(
    operator_family: &'static str,
    read_source: VertexDiskReadSource,
) -> VertexDiskMigrationCloseout {
    let query_report_digest = read_source
        .query_report_digests()
        .first()
        .cloned()
        .unwrap_or_else(|| "query.native.read.receipt".to_string());
    let plan =
        selected_vertex_disks_plan_with_query_read_digest(operator_family, &query_report_digest);
    let read_receipt = VertexDiskReadStageExecutor::execute(&plan, read_source).unwrap();
    let input =
        VertexDiskExecutionInput::from_selected_plan_and_read_stage(&plan, read_receipt).unwrap();
    close_vertex_disk_migration_slice(&plan, input).unwrap()
}

pub(crate) fn close_vertex_disk_slice_from_topology(
    operator_family: &'static str,
) -> VertexDiskMigrationCloseout {
    let plan = selected_vertex_disks_plan(operator_family);
    let touched_closure = selected_vertex_disk_touched_closure(operator_family);
    let topology = selected_vertex_disk_topology_with_unrelated_disks();
    let read_source =
        VertexDiskReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
            .unwrap();
    close_vertex_disk_slice_from_read_source(operator_family, read_source)
}
