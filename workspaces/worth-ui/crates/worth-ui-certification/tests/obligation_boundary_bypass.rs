use std::path::{Path, PathBuf};

use worth_ui_certification::topology::{
    audit_consumers_route_obligations_through_worth_ui_facade,
    certify_consumers_route_admission_through_worth_ui_facade,
};

fn fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_negative")
        .join(name)
}

#[test]
fn anti_bypass_certification_rejects_hostile_obligation_consumer_fixture() {
    let violations = audit_consumers_route_obligations_through_worth_ui_facade(&fixture_root(
        "obligation_facade_bypass_consumer",
    ));

    assert_fixture_violation(
        &violations,
        "fake-obligation-consumer\\Cargo.toml",
        "depends on `worth-ui-runtime` directly",
    );
    assert_fixture_violation(
        &violations,
        "fake-obligation-consumer\\src\\lib.rs",
        "must enter through `worth_ui::facade::obligations`",
    );
}

#[test]
fn anti_bypass_certification_rejects_hostile_runtime_admission_consumer_fixture() {
    let violations = certify_consumers_route_admission_through_worth_ui_facade(&fixture_root(
        "admission_facade_bypass_consumer",
    ))
    .expect_err("hostile admission consumer fixture should fail certification");

    assert_fixture_violation(
        &violations,
        "fake-admission-consumer\\Cargo.toml",
        "depends on `worth-ui-runtime` directly",
    );
    assert_fixture_violation(
        &violations,
        "fake-admission-consumer\\src\\lib.rs",
        "must enter through `worth_ui::facade::admission`",
    );
}

fn assert_fixture_violation(violations: &[String], path_suffix: &str, message: &str) {
    assert!(
        violations
            .iter()
            .any(|violation| violation.contains(path_suffix) && violation.contains(message)),
        "expected violation for {path_suffix} containing `{message}`; actual violations:\n{}",
        violations.join("\n")
    );
}
