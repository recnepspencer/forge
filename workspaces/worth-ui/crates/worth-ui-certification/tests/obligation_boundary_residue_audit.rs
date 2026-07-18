use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_legality_resolution_stays_in_admission_owner_lane,
    audit_non_owner_code_does_not_reopen_obligation_declaration_source,
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
fn obligation_phase_keeps_declaration_source_reopening_out_of_later_layers() {
    let violations =
        audit_non_owner_code_does_not_reopen_obligation_declaration_source(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn obligation_residue_audit_rejects_known_bad_source_reopening_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("obligation_declaration_source_reopening_non_owner"),
    );
    let violations = audit_non_owner_code_does_not_reopen_obligation_declaration_source(&inventory);
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "DSL authority type `UiDslLoweringReceipt`",
    );
    assert_has_violation(
        &violations,
        "worth-ui-inspection",
        "DSL accessor `semantic_artifact()`",
    );
}

#[test]
fn legality_resolution_stays_in_the_admission_owner_lane() {
    let violations = audit_legality_resolution_stays_in_admission_owner_lane(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn obligation_residue_audit_rejects_known_bad_legality_resolution_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("obligation_legality_resolution_non_owner"),
    );
    let violations = audit_legality_resolution_stays_in_admission_owner_lane(&inventory);
    assert_has_violation(
        &violations,
        "worth-ui-runtime",
        "legality reason variant path",
    );
    assert_has_violation(
        &violations,
        "worth-ui-runtime",
        "legality posture variant path",
    );
}
