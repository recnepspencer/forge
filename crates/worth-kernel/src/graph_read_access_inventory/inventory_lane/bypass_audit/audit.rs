use forge_query::facade::consumer_kit::{
    graph_read_bypass_audit, ForgeQueryBoundaryAuditSourceInventory,
    ForgeQueryGraphReadBypassReport,
};

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};

pub(in crate::graph_read_access_inventory::inventory_lane) fn audit_graph_read_bypass_inventory(
    inventory: &ForgeQueryBoundaryAuditSourceInventory,
) -> Result<ForgeQueryGraphReadBypassReport, WorthGraphReadAccessInventoryError> {
    graph_read_bypass_audit("worth-kernel-graph-read-access-inventory")
        .required_inventory(inventory)
        .evaluate()
        .map_err(|query_error| {
            error_with_message(
                WorthGraphReadAccessInventoryErrorKind::GraphReadBypassBoundaryAuditFailed,
                query_error.message(),
            )
        })
}

fn error_with_message(
    kind: WorthGraphReadAccessInventoryErrorKind,
    message: impl Into<String>,
) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::with_message(kind, message)
}
