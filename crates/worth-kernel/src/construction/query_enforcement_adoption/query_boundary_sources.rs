use forge_query::facade::consumer_kit::{
    query_boundary_source_inventory, ForgeQueryBoundaryAuditError,
    ForgeQueryBoundaryAuditSourceInventory, ForgeQueryBoundaryAuditSourceSet,
};

pub(crate) fn worth_kernel_query_boundary_inventory(
) -> Result<ForgeQueryBoundaryAuditSourceInventory, ForgeQueryBoundaryAuditError> {
    query_boundary_source_inventory("worth-kernel")
        .required_root(format!("{}/src", env!("CARGO_MANIFEST_DIR")))
        .include_rs_files()
        .seal()
}

pub(crate) fn worth_kernel_query_boundary_sources() -> ForgeQueryBoundaryAuditSourceSet {
    worth_kernel_query_boundary_inventory()
        .expect("worth-kernel source inventory must be discoverable")
        .boundary_sources()
}

pub(crate) fn worth_kernel_query_boundary_source_count() -> usize {
    worth_kernel_query_boundary_inventory()
        .expect("worth-kernel source inventory must be discoverable")
        .source_count()
}
