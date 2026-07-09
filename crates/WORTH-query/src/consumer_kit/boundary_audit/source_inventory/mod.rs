mod builder;
mod evidence;
mod filesystem;
mod inventory;

pub use builder::WorthQueryBoundaryAuditSourceInventoryBuilder;
pub use inventory::{
    WorthQueryBoundaryAuditSourceInventory, WorthQueryBoundaryAuditSourceInventoryFile,
};

pub fn query_boundary_source_inventory(
    crate_name: impl Into<String>,
) -> WorthQueryBoundaryAuditSourceInventoryBuilder {
    WorthQueryBoundaryAuditSourceInventoryBuilder::new(crate_name)
}
