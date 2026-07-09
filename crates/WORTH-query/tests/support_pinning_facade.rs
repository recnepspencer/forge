#[test]
fn support_pinning_facade_is_usable_and_construction_is_sealed() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/support_pinning/facade_consumer_pass.rs");
    t.compile_fail("tests/ui/support_pinning/*_constructor_private.rs");
}
