#[test]
fn worth_topo_public_boundary_rejects_internal_runtime_bypass() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/private_stage_helper.rs");
    t.compile_fail("tests/compile_fail/mint_materialized_topology_view.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_envelope.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_failure.rs");
    t.compile_fail("tests/compile_fail/public_reader_not_exported.rs");
}
