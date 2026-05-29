mod materialized_live_view;
mod persistent_naming;

use crate::projection::derived_surfaces::TopologyQuerySurfaceError;

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

pub use materialized_live_view::{
    declare_topology_entity_live_view, declare_topology_materialized_surface,
    declare_topology_relation_live_view, topology_entity_live_view_declaration,
    topology_materialized_computed_declaration, topology_relation_live_view_declaration,
    TopologyMaterializedMaintainer,
};
pub use persistent_naming::{
    declare_persistent_name_live_view, naming_attachment_report_from_query_input,
    persistent_name_live_view_declaration, TopologyNamingAttachmentInput,
};
