mod closeout_harness;
mod identity_slots;
mod query_read;
mod selected_plans;
mod topology_fixtures;
mod touched_closures;

pub(crate) use closeout_harness::{
    close_vertex_disk_slice_from_read_source, close_vertex_disk_slice_from_topology,
};
pub(crate) use query_read::{
    query_native_shared_vertex_view, query_native_vertex_disk_read_source,
};
pub(crate) use selected_plans::{
    selected_vertex_disks_plan, selected_vertex_disks_plan_with_query_read_digest,
};
pub(crate) use topology_fixtures::{
    selected_vertex_disk_read_source, selected_vertex_disk_topology_with_unrelated_disks,
    vertex_disk_source_row,
};
pub(crate) use touched_closures::selected_vertex_disk_touched_closure;
