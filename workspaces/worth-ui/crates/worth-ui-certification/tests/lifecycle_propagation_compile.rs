#[path = "trybuild_support.rs"]
mod trybuild_support;
use worth_ui_certification::topology::lifecycle_propagation_fixture_paths;

#[test]
fn runtime_bootstrap_requires_every_subsystem_field() {
    let tests = trybuild_support::new_test_cases();
    for fixture_path in lifecycle_propagation_fixture_paths() {
        tests.compile_fail(*fixture_path);
    }
}

