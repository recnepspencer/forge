use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_dummy_future_family_extension_home, audit_evidence_family_storage_homes,
    audit_inspection_future_artifact_seed_topology,
    audit_inspection_materialized_detail_growth_posture,
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
fn inspection_growth_posture_stays_seeded_typed_and_family_local() {
    let mut violations = audit_inspection_future_artifact_seed_topology(workspace_root());
    violations.extend(audit_inspection_materialized_detail_growth_posture(
        workspace_root(),
    ));
    violations.extend(audit_evidence_family_storage_homes(workspace_root()));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn growth_posture_audit_rejects_missing_future_seed_homes_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        fixture_root("inspection_missing_artifact_seed_homes"),
    );
    let violations = audit_inspection_future_artifact_seed_topology(&inventory);
    assert_has_violation(
        &violations,
        "receipt/replay/mod.rs",
        "future replay inspection artifacts lack an honest internal home",
    );
    assert_has_violation(
        &violations,
        "receipt/mod.rs",
        "private `replay` child module",
    );
}

#[test]
fn growth_posture_audit_rejects_generic_materialized_detail_drift_fixture() {
    let inventory = source_inventory(fixture_root("inspection_materialized_detail_growth_drift"));
    let violations = audit_inspection_materialized_detail_growth_posture(&inventory);
    assert_has_violation(
        &violations,
        "evidence_materialized_detail.rs",
        "must stay #[non_exhaustive]",
    );
    assert_has_violation(
        &violations,
        "evidence_materialized_detail.rs",
        "forbidden generic fallback `serde_json::Value`",
    );
}

#[test]
fn dummy_future_family_extension_has_one_certified_home() {
    let inventory = source_inventory(positive_fixture_root(
        "inspection_dummy_future_family_good_home",
    ));
    let violations = audit_dummy_future_family_extension_home(&inventory);
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn dummy_future_family_extension_rejects_wrong_home_and_second_substrate() {
    let wrong_inventory =
        source_inventory(fixture_root("inspection_dummy_future_family_wrong_home"));
    let wrong_home = audit_dummy_future_family_extension_home(&wrong_inventory);
    assert_has_violation(
        &wrong_home,
        "dummy_future_family.rs",
        "outside the one certified inspection evidence substrate home",
    );
    assert_has_violation(
        &wrong_home,
        "dummy_future_family.rs",
        "forbidden facade/debug substrate",
    );

    let second_inventory = source_inventory(fixture_root(
        "inspection_dummy_future_family_second_substrate",
    ));
    let second_substrate = audit_dummy_future_family_extension_home(&second_inventory);
    assert_has_violation(
        &second_substrate,
        "dummy_future_family.rs",
        "outside the one certified inspection evidence substrate home",
    );
    assert_has_violation(
        &second_substrate,
        "dummy_future_family.rs",
        "forbidden facade/debug substrate",
    );
}
