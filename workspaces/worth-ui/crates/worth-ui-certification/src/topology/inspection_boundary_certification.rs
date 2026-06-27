use std::path::Path;

use super::inspection_boundary_audit::audit_consumers_route_inspection_through_worth_ui_facade;

pub fn certify_consumers_route_inspection_through_worth_ui_facade(
    workspace_root: &Path,
) -> Result<(), Vec<String>> {
    let violations = audit_consumers_route_inspection_through_worth_ui_facade(workspace_root);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
