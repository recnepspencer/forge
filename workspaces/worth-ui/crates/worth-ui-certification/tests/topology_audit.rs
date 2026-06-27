use std::path::Path;

use worth_ui_certification::topology::{
    audit_host_egui_dependency_boundary, audit_no_cross_crate_deep_imports,
    expected_phase3_lifecycle_subsystems,
};
use worth_ui::facade::WorthUi;

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
}

#[test]
fn host_egui_only_uses_host_contract_surfaces() {
    let violations = audit_host_egui_dependency_boundary(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn no_crate_deep_imports_sibling_internals() {
    let violations = audit_no_cross_crate_deep_imports(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn lifecycle_inventories_match_phase3_closure_inventory() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let expected = expected_phase3_lifecycle_subsystems();

    let runtime_rows: Vec<_> = app
        .runtime_support_inventory()
        .rows()
        .iter()
        .map(|row| row.subsystem())
        .collect();
    let inspection_rows: Vec<_> = app
        .inspection_scope_inventory()
        .rows()
        .iter()
        .map(|row| row.subsystem())
        .collect();

    assert_eq!(runtime_rows, expected);
    assert_eq!(inspection_rows, expected);
}

#[test]
fn facade_inspection_is_available_from_immutable_app_reference() {
    let app = WorthUi::app()
        .with_dsl_package(worth_ui_dsl::WorthUiDslPackage::empty())
        .freeze();
    let app_ref = &app;
    let receipt = app_ref.inspect(worth_ui::facade::UiInspectionQuery::new(
        worth_ui::facade::UiInspectionTarget::product_root(),
        worth_ui::facade::UiInspectionScope::graph(),
    ));

    assert_eq!(
        receipt.query().scope(),
        worth_ui::facade::UiInspectionScope::Graph
    );
}
