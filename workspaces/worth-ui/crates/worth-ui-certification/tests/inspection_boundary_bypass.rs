use std::path::{Path, PathBuf};

use worth_ui_certification::topology::certify_consumers_route_inspection_through_worth_ui_facade;

fn fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_negative")
        .join(name)
}

#[test]
fn anti_bypass_certification_rejects_hostile_consumer_fixture() {
    let violations = certify_consumers_route_inspection_through_worth_ui_facade(&fixture_root(
        "inspection_facade_bypass_consumer",
    ))
    .expect_err("hostile consumer fixture should fail certification");

    assert!(
        violations.iter().any(|violation| {
            violation.contains("fake-inspection-consumer\\Cargo.toml")
                && violation.contains("depends on `worth-ui-runtime` directly")
        }),
        "expected direct worth-ui-runtime dependency rejection; actual violations:\n{}",
        violations.join("\n")
    );
    assert!(
        violations.iter().any(|violation| {
            violation.contains("src\\lib.rs")
                && violation.contains("must enter through worth_ui::facade")
        }),
        "expected direct source-path bypass rejection; actual violations:\n{}",
        violations.join("\n")
    );
}
