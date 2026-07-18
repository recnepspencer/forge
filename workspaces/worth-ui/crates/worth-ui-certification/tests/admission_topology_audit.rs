use worth_ui_certification::topology::{
    audit_admission_facades_are_curated_and_glob_free,
    audit_consumers_route_admission_through_worth_ui_facade,
    audit_runtime_admission_surface_routes_through_curated_submodule,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

#[test]
fn runtime_admission_surface_routes_through_curated_submodule() {
    let violations =
        audit_runtime_admission_surface_routes_through_curated_submodule(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn admission_facades_are_curated_and_glob_free() {
    let violations = audit_admission_facades_are_curated_and_glob_free(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn consumers_route_admission_through_worth_ui_facade() {
    let violations = audit_consumers_route_admission_through_worth_ui_facade(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
