mod closeout_harness;
mod identity_slots;
mod selected_plans;
mod topology_fixtures;
mod touched_closures;

pub(crate) use closeout_harness::{admitted_input, close_shell_view_slice_from_topology};
pub(crate) use selected_plans::{
    selected_shell_views_plan, selected_shell_views_plan_for_shell,
    selected_shell_views_plan_with_query_read_digest, unrelated_geometry_selected_plan,
};
pub(crate) use topology_fixtures::{
    selected_shell_view_read_source, selected_shell_view_topology_with_many_unrelated_shells,
    selected_shell_view_topology_with_unrelated_shells, source_row,
};
pub(crate) use touched_closures::{
    selected_shell_view_touched_closure, selected_shell_view_touched_closure_for_shell,
};
