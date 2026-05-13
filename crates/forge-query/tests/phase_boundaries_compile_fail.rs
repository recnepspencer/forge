#[test]
fn query_phase_boundaries_enforce_absent_internal_runtime_builder_methods() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/certification/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/dx/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/lower_runtime/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/migration/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/proof/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/receipts/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/support/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/execution/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/lowering/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/proof/*.rs");
    t.compile_fail("tests/ui/effect_lifecycle/support/*.rs");
}
