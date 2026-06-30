#[test]
fn topology_compiled_product_family_public_boundary_compile_fail_fixtures_execute() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_family_declaration_constructor_not_exported.rs",
    );
    test_cases.compile_fail(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_family_proof_products_not_deserializable.rs",
    );
    test_cases.compile_fail(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_admission_not_exported.rs",
    );
}
