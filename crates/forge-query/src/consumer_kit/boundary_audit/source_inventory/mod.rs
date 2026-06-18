mod builder;
mod evidence;
mod filesystem;
mod inventory;

pub use builder::ForgeQueryBoundaryAuditSourceInventoryBuilder;
pub use inventory::{
    ForgeQueryBoundaryAuditSourceInventory, ForgeQueryBoundaryAuditSourceInventoryFile,
};

pub fn query_boundary_source_inventory(
    crate_name: impl Into<String>,
) -> ForgeQueryBoundaryAuditSourceInventoryBuilder {
    ForgeQueryBoundaryAuditSourceInventoryBuilder::new(crate_name)
}
