mod computed_views;

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

#[allow(unused_imports)]
pub(crate) use computed_views::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
    decode_query_surface_row, interpreted_topology_from_materialized_rows,
    validation_report_from_query_rows, TopologyQuerySurfaceError,
};
