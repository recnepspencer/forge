#[test]
fn query_basis_lifecycle_phase_boundaries_enforce_absent_internal_runtime_builder_methods() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/basis_lifecycle/certification/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/dx/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/lower_runtime/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/migration/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/proof/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/receipts/*.rs");
    t.compile_fail("tests/ui/basis_lifecycle/support/*.rs");
}
