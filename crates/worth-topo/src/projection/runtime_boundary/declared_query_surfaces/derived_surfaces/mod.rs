mod computed_views;

pub(crate) use computed_views::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
};
#[cfg(test)]
pub(crate) use computed_views::{
    decode_query_surface_row, interpreted_topology_from_materialized_rows,
    validation_report_from_query_rows,
};
