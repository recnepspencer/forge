use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_host_and_inspection_layers_do_not_import_declaration_authority,
    audit_non_owner_code_does_not_reopen_declaration_source,
    audit_phase4_authored_lookup_lane_does_not_reopen_declaration_source,
    audit_phase4_authored_lookup_lane_is_indexed_not_scan_first,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
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
fn non_owner_code_does_not_reopen_declaration_source() {
    let violations = audit_non_owner_code_does_not_reopen_declaration_source(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn phase4_authored_lookup_lane_does_not_reopen_declaration_source() {
    let violations =
        audit_phase4_authored_lookup_lane_does_not_reopen_declaration_source(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn phase4_authored_lookup_lane_is_indexed_not_scan_first() {
    let violations = audit_phase4_authored_lookup_lane_is_indexed_not_scan_first(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn declaration_residue_audit_rejects_known_bad_source_reopening_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("declaration_source_reopening_non_owner"),
    );
    let violations = audit_non_owner_code_does_not_reopen_declaration_source(&inventory);
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "DSL semantic authority type `UiDslLoweringReceipt`",
    );
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "DSL semantic accessor `semantic_artifact()`",
    );
}

#[test]
fn host_and_inspection_layers_do_not_import_declaration_authority() {
    let violations =
        audit_host_and_inspection_layers_do_not_import_declaration_authority(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
