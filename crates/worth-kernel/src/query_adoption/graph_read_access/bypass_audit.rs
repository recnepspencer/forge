use forge_query::facade::consumer_kit::{
    graph_read_bypass_audit, ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditSourceInventory,
    ForgeQueryGraphReadBypassReport,
};

pub(super) fn audit_construction_graph_read_bypass(
    inventory: &ForgeQueryBoundaryAuditSourceInventory,
) -> Result<ForgeQueryGraphReadBypassReport, ForgeQueryBoundaryAuditError> {
    graph_read_bypass_audit("worth-kernel-phase-17-construction")
        .required_inventory(inventory)
        .evaluate()
}
