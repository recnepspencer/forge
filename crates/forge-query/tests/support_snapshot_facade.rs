#[test]
fn support_snapshot_facade_is_usable_and_construction_is_sealed() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/support_snapshot/facade_consumer_compiles.rs");
    t.compile_fail("tests/ui/support_snapshot/*_constructor_private.rs");
}
