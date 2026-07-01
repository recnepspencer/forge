use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_declaration_facades_are_curated_and_glob_free, audit_no_cross_crate_deep_imports,
    audit_non_product_crates_route_declaration_through_worth_ui_facade,
    audit_runtime_declaration_surface_routes_through_curated_submodule,
};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
}

fn topology_negative_fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_negative")
        .join(name)
}

fn assert_has_violation(
    violations: &[String],
    expected_file_fragment: &str,
    expected_reason_fragment: &str,
) {
    assert!(
        violations.iter().any(|violation| {
            violation.contains(expected_file_fragment)
                && violation.contains(expected_reason_fragment)
        }),
        "expected a violation containing file fragment `{expected_file_fragment}` and reason fragment `{expected_reason_fragment}`;\nactual violations:\n{}",
        violations.join("\n")
    );
}

#[test]
fn runtime_declaration_surface_routes_through_curated_submodule() {
    let violations =
        audit_runtime_declaration_surface_routes_through_curated_submodule(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn declaration_facades_are_curated_and_glob_free() {
    let violations = audit_declaration_facades_are_curated_and_glob_free(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn declaration_phase_lane_owns_cross_crate_dependency_audit() {
    let violations = audit_no_cross_crate_deep_imports(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn declaration_dependency_audit_rejects_use_based_deep_import_fixture() {
    let violations = audit_no_cross_crate_deep_imports(&topology_negative_fixture_root(
        "declaration_use_based_deep_import",
    ));
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "deep-imports `worth_ui_runtime::source`",
    );
}

#[test]
fn non_product_crates_route_declaration_through_worth_ui_facade() {
    let violations =
        audit_non_product_crates_route_declaration_through_worth_ui_facade(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn declaration_facade_bypass_audit_rejects_known_bad_fixture() {
    let violations = audit_non_product_crates_route_declaration_through_worth_ui_facade(
        &topology_negative_fixture_root("declaration_facade_bypass_consumer"),
    );
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "must enter through `worth_ui::facade::declaration`",
    );
}
