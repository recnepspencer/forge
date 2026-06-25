mod closeout_harness;
mod identity_slots;
mod selected_plans;
mod topology_fixtures;
mod touched_closures;

pub(crate) use closeout_harness::{admitted_input, close_radial_ring_slice_from_topology};
pub(crate) use selected_plans::{
    selected_radial_rings_plan, selected_radial_rings_plan_for_shell,
    selected_radial_rings_plan_with_query_read_digest, unrelated_geometry_selected_plan,
};
pub(crate) use topology_fixtures::{
    selected_radial_ring_read_source, selected_radial_ring_topology_with_many_unrelated_shells,
    selected_radial_ring_topology_with_unrelated_shells, source_row,
};
pub(crate) use touched_closures::{
    selected_radial_ring_touched_closure, selected_radial_ring_touched_closure_for_shell,
};
