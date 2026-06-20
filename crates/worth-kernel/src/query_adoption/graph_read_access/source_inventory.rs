use std::path::PathBuf;

use forge_query::facade::consumer_kit::{
    query_boundary_source_inventory, ForgeQueryBoundaryAuditError,
    ForgeQueryBoundaryAuditSourceInventory,
};

pub(super) fn construction_graph_read_source_inventory(
) -> Result<ForgeQueryBoundaryAuditSourceInventory, ForgeQueryBoundaryAuditError> {
    query_boundary_source_inventory("worth-kernel")
        .required_root(construction_dir())
        .include_rs_files()
        .seal()
}

pub(super) fn construction_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/construction")
}
