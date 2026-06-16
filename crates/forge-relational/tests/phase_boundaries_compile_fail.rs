#[test]
fn lineage_phase_boundaries_are_compile_time_private() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/lineage/*.rs");
}

#[test]
fn merge_phase_boundaries_are_compile_time_private() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/merge/*.rs");
}

#[test]
fn phase_2a_upstream_identity_boundaries_reject_projection_reentry() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/phase_2a/*.rs");
}
