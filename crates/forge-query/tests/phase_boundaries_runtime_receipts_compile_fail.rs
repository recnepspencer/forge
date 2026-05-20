#[test]
fn query_phase_boundaries_enforce_runtime_receipt_constructor_privacy() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/runtime_receipts/*.rs");
}
