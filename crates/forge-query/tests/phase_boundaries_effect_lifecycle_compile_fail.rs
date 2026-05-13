#[test]
fn query_effect_lifecycle_phase_boundaries_enforce_absent_internal_runtime_builder_methods() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/effect_lifecycle/execution/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/lowering/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/proof/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/support/*.rs");
}
