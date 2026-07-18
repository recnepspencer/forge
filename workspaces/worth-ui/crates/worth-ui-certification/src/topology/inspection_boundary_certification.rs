use super::inspection_boundary_audit::audit_consumers_route_inspection_through_worth_ui_facade;
use super::workspace_source_inventory::WorkspaceSourceInventory;

pub fn certify_consumers_route_inspection_through_worth_ui_facade(
    inventory: &WorkspaceSourceInventory,
) -> Result<(), Vec<String>> {
    let violations = audit_consumers_route_inspection_through_worth_ui_facade(inventory);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
