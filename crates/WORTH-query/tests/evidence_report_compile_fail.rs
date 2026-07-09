#[test]
fn evidence_report_boundaries_are_compile_time_enforced() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/evidence_report/*.rs");
}
