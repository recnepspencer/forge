mod admission;
mod core_execution;
mod grouped_transition;
mod ordinary_patch;
mod result_assembly;

pub(crate) use admission::{
    admit_grouped_live_view, execute_grouped_live_view_shape_change, execute_live_view_shape_change,
};
