#[path = "catalog_kernel_public_closeout.rs"]
mod catalog_kernel_public_closeout;
#[path = "catalog_kernel_source_firewall.rs"]
mod catalog_kernel_source_firewall;

use super::row::PlannerOwnedRoutingInventoryRow as Row;

pub(super) fn rows() -> Vec<Row> {
    let mut rows = catalog_kernel_public_closeout::rows();
    rows.extend(catalog_kernel_source_firewall::rows());
    rows
}
