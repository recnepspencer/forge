#[test]
fn worth_topo_public_boundary_rejects_internal_runtime_bypass() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/private_stage_helper.rs");
    t.compile_fail("tests/compile_fail/mint_materialized_topology_view.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_envelope.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_failure.rs");
    t.compile_fail("tests/compile_fail/public_query_assembly_not_forgeable.rs");
    t.compile_fail("tests/compile_fail/public_runtime_adapters_not_forgeable.rs");
    t.compile_fail("tests/compile_fail/public_query_edit_runner_not_exported.rs");
    t.compile_fail("tests/compile_fail/public_milestone_one_read_view_cert_not_exported.rs");
    t.compile_fail("tests/compile_fail/public_milestone_two_read_view_cert_not_exported.rs");
    t.compile_fail("tests/compile_fail/public_runtime_constructor_requires_adapters.rs");
    t.compile_fail("tests/compile_fail/public_query_import_not_exported.rs");
    t.compile_fail("tests/compile_fail/public_query_row_materializer_not_exported.rs");
}
