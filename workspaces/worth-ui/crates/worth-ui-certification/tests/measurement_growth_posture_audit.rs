use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_measurement_basis_artifact_growth_posture,
    audit_measurement_future_family_extension_home, audit_measurement_future_growth_posture,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

fn fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_negative")
        .join(name)
}

fn positive_fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_positive")
        .join(name)
}

fn source_inventory(root: PathBuf) -> worth_ui_certification::topology::WorkspaceSourceInventory {
    worth_ui_certification::topology::WorkspaceSourceInventory::capture(root)
}

fn assert_has_violation(violations: &[String], file_fragment: &str, reason_fragment: &str) {
    assert!(
        violations
            .iter()
            .any(|violation| violation.contains(file_fragment) && violation.contains(reason_fragment)),
        "expected a violation containing `{file_fragment}` and `{reason_fragment}`;\nactual violations:\n{}",
        violations.join("\n")
    );
}

#[test]
fn measurement_future_growth_stays_kernel_local_and_typed() {
    let violations = audit_measurement_future_growth_posture(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn measurement_basis_growth_audit_rejects_generic_fallback_fixture() {
    let inventory = source_inventory(fixture_root("measurement_basis_growth_drift"));
    let violations = audit_measurement_basis_artifact_growth_posture(&inventory);
    assert_has_violation(
        &violations,
        "basis/admit.rs",
        "forbidden generic fallback `serde_json::Value`",
    );
}

#[test]
fn measurement_dummy_future_family_has_one_certified_home() {
    let inventory = source_inventory(positive_fixture_root(
        "measurement_dummy_future_family_good_home",
    ));
    let violations = audit_measurement_future_family_extension_home(&inventory);
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn measurement_dummy_future_family_rejects_wrong_home_and_second_substrate() {
    let wrong_inventory =
        source_inventory(fixture_root("measurement_dummy_future_family_wrong_home"));
    let wrong_home = audit_measurement_future_family_extension_home(&wrong_inventory);
    assert_has_violation(
        &wrong_home,
        "dummy_measurement_family.rs",
        "outside the one certified measurement growth home",
    );

    let second_inventory = source_inventory(fixture_root(
        "measurement_dummy_future_family_second_substrate",
    ));
    let second_substrate = audit_measurement_future_family_extension_home(&second_inventory);
    assert_has_violation(
        &second_substrate,
        "dummy_measurement_family.rs",
        "forbidden facade/debug/host substrate",
    );
}
