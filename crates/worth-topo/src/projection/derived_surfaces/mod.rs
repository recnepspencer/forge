mod computed_views;

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

pub(crate) use computed_views::decode_single_computed_row;
pub use computed_views::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
    interpreted_topology_from_materialized_rows, topology_interpreted_computed_declaration,
    topology_validation_computed_declaration, validation_report_from_query_rows,
    TopologyInterpretedMaintainer, TopologyQuerySurfaceError, TopologyValidationMaintainer,
};




