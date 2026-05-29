#[test]
fn schema_boundary_packets_are_not_forgeable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/mint_boundary_envelope.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_failure.rs");
    t.compile_fail("tests/compile_fail/public_topology_authoring_namespace_is_curated.rs");
    t.compile_fail("tests/compile_fail/public_seed_helpers_not_on_main_facade.rs");
    t.compile_fail("tests/compile_fail/public_topology_authoring_root_module_missing.rs");
}
