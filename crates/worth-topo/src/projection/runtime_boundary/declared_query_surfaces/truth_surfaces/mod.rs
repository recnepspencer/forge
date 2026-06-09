mod materialized_live_view;
mod persistent_naming;

use super::derived_surfaces::TopologyQuerySurfaceError;

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

pub(crate) use materialized_live_view::{
    declare_topology_entity_live_view, declare_topology_materialized_surface,
    declare_topology_relation_live_view,
};
pub(crate) use persistent_naming::{
    declare_persistent_name_live_view, naming_attachment_report_from_query_input,
    TopologyNamingAttachmentInput,
};
pub use persistent_naming::{NamingAttachmentReport, NamingAttachmentRow};
