#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn snapshot_internal_indexes_not_publicly_mutable() {
    let tests = trybuild_support::new_test_cases();
    tests
        .compile_fail("tests/ui/facade/snapshot/snapshot_internal_indexes_not_publicly_mutable.rs");
}

