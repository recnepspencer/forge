#[test]
fn snapshot_internal_indexes_not_publicly_mutable() {
    let tests = trybuild::TestCases::new();
    tests
        .compile_fail("tests/ui/facade/snapshot/snapshot_internal_indexes_not_publicly_mutable.rs");
}
